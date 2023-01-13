use crate::{
  error::ContractError,
  models::{Cell, GridCoordinates, Position},
  state::{GAME, GRID, PLAYERS},
};
use cosmwasm_std::{attr, Addr, Coin, DepsMut, Env, MessageInfo, Response, Uint128};
use cw20::Cw20QueryMsg;

pub fn buy_cells(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  coordinates_list: &Vec<GridCoordinates>,
) -> Result<Response, ContractError> {
  let player_addr = &info.sender;
  let mut payment_amount = Uint128::zero();

  if let Some(mut player) = PLAYERS.may_load(deps.storage, player_addr.clone())? {
    let mut game = GAME.load(deps.storage)?;
    let mut positions = player.positions.unwrap_or(vec![]);

    if game.is_over() {
      return Err(ContractError::GameOver {});
    }

    for coords in coordinates_list.iter() {
      // update each puchased cell's state
      GRID.update(
        deps.storage,
        coords.clone(),
        |some_cell| -> Result<Cell, ContractError> {
          if let Some(mut cell) = some_cell {
            let mut player_addrs = cell.player_addrs.unwrap_or(vec![]);
            if let Some(max_players_per_cell) = game.max_players_per_cell {
              if player_addrs.len() == max_players_per_cell as usize {
                // sold out of spots in this cell
                return Err(ContractError::CellSoldOut {});
              }
            }
            if player_addrs.contains(player_addr) {
              // a player can't buy the same cell twice
              return Err(ContractError::NotAuthorized {});
            }

            // save the player's addr to the cell
            player_addrs.push(player_addr.clone());
            cell.player_addrs = Some(player_addrs);

            // increment running subtotal for the purchase amount
            payment_amount += cell.price;

            Ok(cell)
          } else {
            // invalid grid cell coordinates
            Err(ContractError::CoordinatesOutOfBounds {})
          }
        },
      )?;

      // add the purchase cell's coordinates and current quarter index
      // to the player's positions.
      positions.push(Position {
        coords: coords.clone(),
        quarter_index: game.quarter_index,
      });
    }

    // ensure the player is sending the exact funds required for their purchase
    match &game.token {
      crate::models::Token::Native { denom } => verify_native_funds(&info.funds, payment_amount, &denom)?,
      crate::models::Token::Cw20 { address } => verify_cw20_funds(&deps, player_addr, payment_amount, &address)?,
    }
    // increment prize pool size with total payment amount for this order
    game.token_amount += payment_amount;

    // update the player with their new positions vec
    player.positions = Some(positions);

    PLAYERS.save(deps.storage, player_addr.clone(), &player)?;
  } else {
    // sender must be a player or the game must be public
    return Err(ContractError::NotAuthorized {});
  }

  Ok(Response::new().add_attributes(vec![attr("action", "buy_cells")]))
}

// Check for the payment amount required by querying the CW20 token contract.
fn verify_cw20_funds(
  deps: &DepsMut,
  wallet: &Addr,
  payment_amount: Uint128,
  cw20_token_address: &Addr,
) -> Result<(), ContractError> {
  let resp: cw20::BalanceResponse = deps.querier.query_wasm_smart(
    cw20_token_address.clone(),
    &Cw20QueryMsg::Balance {
      address: wallet.clone().into(),
    },
  )?;
  if resp.balance < payment_amount {
    return Err(ContractError::InsufficientFunds {});
  }
  Ok(())
}

// Check for the exact payment amount required in the tx's funds.
fn verify_native_funds(
  funds: &Vec<Coin>,
  payment_amount: Uint128,
  denom: &String,
) -> Result<(), ContractError> {
  if let Some(coin) = funds.iter().find(|coin| -> bool { coin.denom == *denom }) {
    if coin.amount < payment_amount {
      return Err(ContractError::InsufficientFunds {});
    } else if coin.amount > payment_amount {
      return Err(ContractError::ExcessFunds {});
    }
  } else {
    return Err(ContractError::InsufficientFunds {});
  }
  Ok(())
}
