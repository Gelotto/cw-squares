use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContractError {
  #[error("{0}")]
  Std(#[from] StdError),

  #[error("NotAuthorized")]
  NotAuthorized {},

  #[error("InsufficientFunds")]
  InsufficientFunds {},

  #[error("ExcessFunds")]
  ExcessFunds {},

  #[error("DuplicatePlayerAddress")]
  DuplicatePlayerAddress {},

  #[error("InsufficientQuarters")]
  InsufficientQuarters {},

  #[error("InsufficientGridCells")]
  InsufficientGridCells {},

  #[error("CoordinatesOutOfBounds")]
  CoordinatesOutOfBounds {},

  #[error("InvalidGridCellPrice")]
  InvalidGridCellPrice {},

  #[error("InvalidTeamCount")]
  InvalidTeamCount {},

  #[error("UnknownPlayerAddress")]
  UnknownPlayerAddress {},

  #[error("InvalidQuarterSplit")]
  InvalidQuarterSplit {},

  #[error("NotStarted")]
  NotStarted {},

  #[error("AlreadyResolved")]
  AlreadyResolved {},

  #[error("AlreadyClaimedRefund")]
  AlreadyClaimedRefund {},

  #[error("AlreadyStarted")]
  AlreadyStarted {},

  #[error("CellSoldOut")]
  CellSoldOut {},

  #[error("GameOver")]
  GameOver {},
}
