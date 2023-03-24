use thiserror::Error;

#[derive(Debug, Error)]
enum NetworkError {
    #[error("connection timed out")]
    Timeout,
}

#[derive(Debug, Error)]
enum DatabaseError {
    #[error("error querying DB")]
    QueryFailure,
}

#[derive(Debug, Error)]
enum ApiError {
    #[error("network error: {0}")]
    Network(#[from] NetworkError),
    #[error("database error: {0}")]
    Database(#[from] DatabaseError),
}

// impl From<NetworkError> for ApiError {
//     fn from(value: NetworkError) -> Self {
//         Self::Network(value)
//     }
// }

// impl From<DatabaseError> for ApiError {
//     fn from(value: DatabaseError) -> Self {
//         Self::Database(value)
//     }
// }

fn do_stuff() -> Result<(), ApiError> {
    Err(NetworkError::Timeout)?
}

fn main() {}
