use reth_db::DatabaseError;
use reth_interfaces::RethError;
use reth_primitives::PruneSegmentError;
use reth_provider::ProviderError;
use thiserror::Error;

/// Errors that can occur during pruning.
#[derive(Error, Debug)]
pub enum PrunerError {
    #[error(transparent)]
    PruneSegment(#[from] PruneSegmentError),

    #[error("inconsistent data: {0}")]
    InconsistentData(&'static str),

    #[error(transparent)]
    Database(#[from] DatabaseError),

    #[error(transparent)]
    Provider(#[from] ProviderError),
}

impl From<PrunerError> for RethError {
    fn from(err: PrunerError) -> Self {
        match err {
            PrunerError::PruneSegment(_) | PrunerError::InconsistentData(_) => {
                RethError::other(err)
            }
            PrunerError::Database(err) => RethError::Database(err),
            PrunerError::Provider(err) => RethError::Provider(err),
        }
    }
}
