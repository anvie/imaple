use crate::error::WError;

pub type Result<T, E = WError> = anyhow::Result<T, E>;
