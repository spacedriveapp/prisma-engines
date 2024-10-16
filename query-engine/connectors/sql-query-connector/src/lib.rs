#![allow(clippy::wrong_self_convention)]
#![deny(unsafe_code)]

mod column_metadata;
mod context;
mod cursor_condition;
mod database;
mod error;
mod filter_conversion;
mod join_utils;
mod model_extensions;
mod nested_aggregations;
mod ordering;
mod query_arguments_ext;
mod query_builder;
mod query_ext;
mod row;
mod value;
mod value_ext;

use self::{column_metadata::*, context::Context, filter_conversion::*, query_ext::QueryExt, row::*};
use quaint::prelude::Queryable;

pub use database::FromSource;

#[cfg(feature = "mssql")]
pub use database::Mssql;
#[cfg(feature = "mysql")]
pub use database::Mysql;
#[cfg(feature = "postgresql")]
pub use database::PostgreSql;
#[cfg(feature = "sqlite")]
pub use database::Sqlite;

#[cfg(feature = "js-connectors")]
pub use database::{register_js_connector, Js};
pub use error::SqlError;

type Result<T> = std::result::Result<T, error::SqlError>;
