use cosmwasm_std::StdError;
use thiserror::Error;


#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Not authorized to perform action")]
    Unauthorized {},

    #[error("Define Error types here")]
    DummyError {},
}
