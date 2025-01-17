use crate::database::Database;
use either::Either;
use std::convert::identity;

/// Provides information on a prepared statement.
///
/// Returned from [`Executor::describe`](trait.Executor.html#method.describe).
///
/// The query macros (e.g., `query!`, `query_as!`, etc.) use the information here to validate
/// output and parameter types; and, generate an anonymous record.
#[derive(Debug)]
#[cfg_attr(feature = "offline", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "offline",
    serde(bound(
        serialize = "DB::TypeInfo: serde::Serialize, DB::Column: serde::Serialize",
        deserialize = "DB::TypeInfo: serde::de::DeserializeOwned, DB::Column: serde::de::DeserializeOwned",
    ))
)]
#[doc(hidden)]
pub struct StatementInfo<DB: Database> {
    pub(crate) columns: Vec<DB::Column>,
    pub(crate) parameters: Option<Either<Vec<DB::TypeInfo>, usize>>,
    pub(crate) nullable: Vec<Option<bool>>,
}

impl<DB: Database> StatementInfo<DB> {
    /// Gets the column information at `index`.
    ///
    /// Panics if `index` is out of bounds.
    pub fn column(&self, index: usize) -> &DB::Column {
        &self.columns[index]
    }

    /// Gets the column information at `index` or `None` if out of bounds.
    pub fn try_column(&self, index: usize) -> Option<&DB::Column> {
        self.columns.get(index)
    }

    /// Gets all columns in this statement.
    pub fn columns(&self) -> &[DB::Column] {
        &self.columns
    }

    /// Gets the available information for parameters in this statement.
    ///
    /// Some drivers may return more or less than others. As an example, **PostgreSQL** will
    /// return `Some(Either::Left(_))` with a full list of type information for each parameter.
    /// However, **MSSQL** will return `None` as there is no information available.
    pub fn parameters(&self) -> Option<Either<&[DB::TypeInfo], usize>> {
        self.parameters.as_ref().map(|p| match p {
            Either::Left(params) => Either::Left(&**params),
            Either::Right(count) => Either::Right(*count),
        })
    }

    /// Gets whether a column may be `NULL`, if this information is available.
    pub fn nullable(&self, column: usize) -> Option<bool> {
        self.nullable.get(column).copied().and_then(identity)
    }
}
