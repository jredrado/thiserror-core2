use thiserror_core2::Error;

#[derive(Error, Debug)]
#[error(transparent)]
#[error("...")]
pub struct Error(anyhow::Error);

fn main() {}
