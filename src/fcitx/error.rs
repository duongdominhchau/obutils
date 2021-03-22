use thiserror::Error;

#[derive(Debug, Error)]
pub enum FcitxError {
    #[error("Failed to deserialize data")]
    FailToDeserialize(),
}
