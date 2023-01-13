use cosmwasm_std::{Deps, StdResult, Storage};

use crate::{
  models::{Cell, Player},
  msg::GameResponse,
  state::{GAME, GRID, PLAYERS},
};

pub fn query_game(
  deps: Deps,
  with_grid: Option<bool>,
  with_players: Option<bool>,
) -> StdResult<GameResponse> {
  Ok(GameResponse {
    game: GAME.load(deps.storage)?,
    players: if with_players.unwrap_or(true) {
      Some(build_players_vec(deps.storage))
    } else {
      None
    },
    grid: if with_grid.unwrap_or(true) {
      Some(build_cells_vec(deps.storage))
    } else {
      None
    },
  })
}

fn build_cells_vec(storage: &dyn Storage) -> Vec<Cell> {
  GRID
    .range(storage, None, None, cosmwasm_std::Order::Ascending)
    .map(|result| -> Cell {
      let (_coord, cell) = result.unwrap();
      cell
    })
    .collect()
}

fn build_players_vec(storage: &dyn Storage) -> Vec<Player> {
  PLAYERS
    .range(storage, None, None, cosmwasm_std::Order::Ascending)
    .map(|result| -> Player {
      let (_addr, player) = result.unwrap();
      player
    })
    .collect()
}
