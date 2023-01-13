use std::collections::HashSet;

use crate::{
  error::ContractError,
  models::{Cell, Game, Player, Quarter},
  msg::InstantiateMsg,
};
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Uint128};
use cw_storage_plus::{Item, Map};

pub const GAME: Item<Game> = Item::new("game");
pub const PLAYERS: Map<Addr, Player> = Map::new("players");
pub const GRID: Map<(u8, u8), Cell> = Map::new("grid");

/// Initialize contract state data.
pub fn initialize(
  deps: DepsMut,
  _env: &Env,
  info: &MessageInfo,
  msg: &InstantiateMsg,
) -> Result<(), ContractError> {
  let mut player_wallets: HashSet<Addr> = HashSet::with_capacity(1);

  // save each player, preventing duplicate wallet addresses
  if let Some(players) = &msg.players {
    for player in players.clone().iter_mut() {
      if player_wallets.contains(&player.wallet) {
        return Err(ContractError::DuplicatePlayerAddress {});
      }

      // force default internal player fields:
      player.has_claimed_refund = Some(false);
      player.positions = None;

      player_wallets.insert(player.wallet.clone());

      PLAYERS.save(deps.storage, player.wallet.clone(), player)?;
    }
  }

  // ensure game creator exists as a player
  if !player_wallets.contains(&info.sender) {
    PLAYERS.save(
      deps.storage,
      info.sender.clone(),
      &Player {
        wallet: info.sender.clone(),
        has_claimed_refund: Some(false),
        positions: None,
        name: None,
        color: None,
      },
    )?;
  }

  // validate quarters
  if msg.quarters.len() != 4 {
    return Err(ContractError::InsufficientQuarters {});
  }
  let mut total_pct = 0u8;
  let mut quarters: Vec<Quarter> = Vec::with_capacity(2);
  for quarter in msg.quarters.iter() {
    let mut validated_quarter = quarter.clone();
    validated_quarter.winner = None;
    quarters.push(validated_quarter);
    total_pct += quarter.pct;
    if total_pct > 100 {
      return Err(ContractError::InvalidQuarterSplit {});
    }
  }

  // the total split shouldn't be less or greater than 100
  if total_pct != 100 {
    return Err(ContractError::InvalidQuarterSplit {});
  }

  // validate number of expected grid cells
  if msg.grid.len() != 100 {
    return Err(ContractError::InsufficientGridCells {});
  }
  // save each grid cell
  for (i, cell) in msg.grid.iter().enumerate() {
    // validate grid cell price
    if cell.price.is_zero() {
      return Err(ContractError::InvalidGridCellPrice {});
    }
    // validate player addresses
    if let Some(cell_player_addrs) = &cell.wallets {
      for player_addr in cell_player_addrs.iter() {
        if !player_wallets.contains(&player_addr) {
          return Err(ContractError::UnknownPlayerAddress {});
        }
      }
    }
    GRID.save(
      deps.storage,
      ((i / 10) as u8, (i % 10) as u8),
      &Cell {
        wallets: cell.wallets.clone(),
        price: cell.price,
      },
    )?;
  }

  // validate teams
  if msg.teams.len() != 2 {
    return Err(ContractError::InvalidTeamCount {});
  }

  // save validated game data
  GAME.save(
    deps.storage,
    &Game {
      admin: info.sender.clone(),
      name: msg.name.clone(),
      is_public: msg.is_public.clone(),
      teams: msg.teams.clone(),
      token: msg.token.clone(),
      max_players_per_cell: msg.max_players_per_cell,
      has_started: false,
      can_claim_refund: false,
      token_amount: Uint128::zero(),
      quarter_index: 0,
      quarters,
    },
  )?;

  Ok(())
}

pub fn require_admin(
  game: &Game,
  addr: &Addr,
) -> Result<(), ContractError> {
  if game.admin != *addr {
    return Err(ContractError::NotAuthorized {});
  }
  Ok(())
}
