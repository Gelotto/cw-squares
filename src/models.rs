use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type GridCoordinates = (u8, u8);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Token {
  Native { denom: String },
  Cw20 { address: Addr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Game {
  pub admin: Addr,
  pub name: String,
  pub is_public: bool,
  pub has_started: bool,
  pub quarters: Vec<Quarter>,
  pub quarter_index: u8,
  pub max_players_per_cell: Option<u16>,
  pub teams: Vec<Team>,
  pub token: Token,
  pub token_amount: Uint128,
}

impl Game {
  pub fn is_over(&self) -> bool {
    self.quarter_index as usize == self.quarters.len()
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Quarter {
  pub winner: Option<GridCoordinates>,
  pub name: Option<String>,
  pub pct: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Team {
  pub name: String,
  pub color: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Cell {
  pub player_addrs: Option<Vec<Addr>>,
  pub price: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Position {
  pub coords: GridCoordinates,
  pub quarter_index: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Player {
  pub wallet: Addr,
  pub name: Option<String>,
  pub color: Option<String>,
  pub positions: Option<Vec<Position>>,
}
