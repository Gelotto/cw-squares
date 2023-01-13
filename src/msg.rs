use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::models::{Cell, GridCoordinates, Player, Quarter, Team, Token};

/// Initial contract state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
  pub name: String,
  pub teams: Vec<Team>,
  pub is_public: bool,
  pub players: Option<Vec<Player>>,
  pub max_players_per_cell: Option<u16>,
  pub quarters: Vec<Quarter>,
  pub grid: Vec<Cell>,
  pub token: Token,
}

/// Executable contract endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
  AddPlayer {
    wallet: Addr,
    name: Option<String>,
    color: Option<String>,
  },
  BuyCells {
    coordinates: Vec<GridCoordinates>,
  },
  ResolveWinner {
    winner: GridCoordinates,
  },
}

/// Custom contract query endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  IsAdmin { address: Addr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BooleanResponse {
  pub value: bool,
}
