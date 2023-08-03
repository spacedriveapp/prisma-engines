use psl::{builtin_connectors::*, Datasource, PreviewFeatures};
use query_core::{executor::InterpretingExecutor, Connector, QueryExecutor};
use std::collections::HashMap;
use tracing::trace;
use url::Url;

#[cfg(feature = "mongodb")]
use mongodb_query_connector::MongoDb;
#[cfg(feature = "sql")]
use sql_query_connector::*;

/// Loads a query executor based on the parsed Prisma schema (datasource).
pub async fn load(
    source: &Datasource,
    features: PreviewFeatures,
    url: &str,
) -> query_core::Result<Box<dyn QueryExecutor + Send + Sync + 'static>> {
    match source.active_provider {
        #[cfg(feature = "sqlite")]
        p if SQLITE.is_provider(p) => sqlite(source, url, features).await,
        #[cfg(feature = "mysql")]
        p if MYSQL.is_provider(p) => mysql(source, url, features).await,
        #[cfg(feature = "postgresql")]
        p if POSTGRES.is_provider(p) => postgres(source, url, features).await,
        #[cfg(feature = "mssql")]
        p if MSSQL.is_provider(p) => mssql(source, url, features).await,
        #[cfg(feature = "postgresql")]
        p if COCKROACH.is_provider(p) => postgres(source, url, features).await,

        #[cfg(feature = "mongodb")]
        p if MONGODB.is_provider(p) => mongodb(source, url, features).await,

        #[cfg(feature = "js-connectors")]
        p if JsConnector::is_provider(p) => jsconnector(source, url, features).await,

        x => Err(query_core::CoreError::ConfigurationError(format!(
            "Unsupported connector type: {x}"
        ))),
    }
}

#[cfg(feature = "sqlite")]
async fn sqlite(
    source: &Datasource,
    url: &str,
    features: PreviewFeatures,
) -> query_core::Result<Box<dyn QueryExecutor + Send + Sync>> {
    trace!("Loading SQLite query connector...");
    let sqlite = Sqlite::from_source(source, url, features).await?;
    trace!("Loaded SQLite query connector.");
    Ok(executor_for(sqlite, false))
}

#[cfg(feature = "postgresql")]
async fn postgres(
    source: &Datasource,
    url: &str,
    features: PreviewFeatures,
) -> query_core::Result<Box<dyn QueryExecutor + Send + Sync>> {
    trace!("Loading Postgres query connector...");
    let database_str = url;
    let psql = PostgreSql::from_source(source, url, features).await?;

    let url = Url::parse(database_str)
        .map_err(|err| query_core::CoreError::ConfigurationError(format!("Error parsing connection string: {err}")))?;
    let params: HashMap<String, String> = url.query_pairs().into_owned().collect();

    let force_transactions = params
        .get("pgbouncer")
        .and_then(|flag| flag.parse().ok())
        .unwrap_or(false);
    trace!("Loaded Postgres query connector.");
    Ok(executor_for(psql, force_transactions))
}

#[cfg(feature = "mysql")]
async fn mysql(
    source: &Datasource,
    url: &str,
    features: PreviewFeatures,
) -> query_core::Result<Box<dyn QueryExecutor + Send + Sync>> {
    let mysql = Mysql::from_source(source, url, features).await?;
    trace!("Loaded MySQL query connector.");
    Ok(executor_for(mysql, false))
}

#[cfg(feature = "mssql")]
async fn mssql(
    source: &Datasource,
    url: &str,
    features: PreviewFeatures,
) -> query_core::Result<Box<dyn QueryExecutor + Send + Sync>> {
    trace!("Loading SQL Server query connector...");
    let mssql = Mssql::from_source(source, url, features).await?;
    trace!("Loaded SQL Server query connector.");
    Ok(executor_for(mssql, false))
}

fn executor_for<T>(connector: T, force_transactions: bool) -> Box<dyn QueryExecutor + Send + Sync>
where
    T: Connector + Send + Sync + 'static,
{
    Box::new(InterpretingExecutor::new(connector, force_transactions))
}

#[cfg(feature = "mongodb")]
async fn mongodb(
    source: &Datasource,
    url: &str,
    _features: PreviewFeatures,
) -> query_core::Result<Box<dyn QueryExecutor + Send + Sync>> {
    trace!("Loading MongoDB query connector...");
    let mongo = MongoDb::new(source, url).await?;
    trace!("Loaded MongoDB query connector.");
    Ok(executor_for(mongo, false))
}

#[cfg(feature = "js-connectors")]
async fn jsconnector(
    source: &Datasource,
    url: &str,
    features: PreviewFeatures,
) -> Result<Box<dyn QueryExecutor + Send + Sync>, query_core::CoreError> {
    trace!("Loading js connector ...");
    let js = Js::from_source(source, url, features).await?;
    trace!("Loaded js connector ...");
    Ok(executor_for(js, false))
}
