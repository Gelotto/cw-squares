use crate::{
  error::ContractError,
  models::{GridCoordinates, Token},
  state::{require_admin, GAME, GRID},
};
use cosmwasm_std::{
  attr, to_binary, Addr, BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, SubMsg, Uint128, WasmMsg,
};
use cw20::Cw20ExecuteMsg;

pub fn resolve_winner(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  winner: GridCoordinates,
) -> Result<Response, ContractError> {
  let mut game = GAME.load(deps.storage)?;

  require_admin(&game, &info.sender)?;

  // game must be started and not over
  if game.is_over() {
    return Err(ContractError::GameOver {});
  }
  if !game.has_started {
    return Err(ContractError::NotStarted {});
  }

  // save the winning coordinates for the resolving quarter
  let quarter = &mut game.quarters[game.quarter_index as usize];
  if quarter.winner != None {
    return Err(ContractError::AlreadyResolved {});
  }
  quarter.winner = Some(winner.clone());

  // calculate prize amount for the winning wallets, distributed evenly to each
  // player in the winning grid cell.
  let quarter_prize_amount = game.token_amount * Uint128::from(quarter.pct / 100);
  let mut transfer_msgs: Vec<CosmosMsg> = vec![];
  let mut cw20_transfer_msgs: Vec<SubMsg> = vec![];

  if let Some(winning_cell) = GRID.may_load(deps.storage, winner)? {
    // get addrs of players in grid cell who won
    let winning_player_addrs = winning_cell.player_addrs.unwrap_or(vec![]);
    // if players exist in it....
    if winning_player_addrs.len() > 0 {
      // calc the amount owed to each winning address
      let player_prize_amount = quarter_prize_amount * Uint128::from(winning_player_addrs.len() as u16 / 100);
      // create a transfer message from the contract to winner address
      for addr in winning_player_addrs.iter() {
        match &game.token {
          Token::Native { denom } => transfer_msgs.push(build_native_send_msg(addr, denom, player_prize_amount)?),
          Token::Cw20 { address: cw20_addr } => cw20_transfer_msgs.push(build_cw20_transfer_msg(
            &env.contract.address,
            &addr,
            &cw20_addr,
            player_prize_amount,
          )?),
        }
      }
    } else {
      // if no one bought the cell that won, we just carry over the winnings into the next
      // quarters, spreading it out according to the existing proportions set up by the split
      let remaining_quarters = &game.quarters.clone()[((game.quarter_index + 1) as usize)..];
      let total_pct_remaining: u8 = remaining_quarters.iter().map(|q| q.pct).sum();
      // update the pct split on the remaining quarters
      for (i, quarter) in remaining_quarters.iter().enumerate() {
        let new_pct = 100u8 * (quarter.pct / total_pct_remaining);
        game.quarters[i + game.quarter_index as usize].pct = new_pct;
      }
    }
  } else {
    return Err(ContractError::CoordinatesOutOfBounds {});
  }

  // increment the quarter index, effectively moving to the next
  // quarter or, in the final case, ending the game.
  game.quarter_index += 1;

  GAME.save(deps.storage, &game)?;

  Ok(
    Response::new()
      .add_submessages(cw20_transfer_msgs)
      .add_messages(transfer_msgs)
      .add_attributes(vec![attr("action", "resolve_winner")]),
  )
}

// Return a Response that performs a bank transfer of native funds to the
/// contract. Validates the payment amount sent in the tx.
pub fn build_native_send_msg(
  to_address: &Addr,
  ibc_denom: &String,
  amount: Uint128,
) -> Result<CosmosMsg, ContractError> {
  // Perform transfer of IBC asset from sender to contract.
  Ok(CosmosMsg::Bank(BankMsg::Send {
    to_address: to_address.clone().into_string(),
    amount: vec![Coin::new(amount.u128(), ibc_denom)],
  }))
}

/// Build transfer message for CW20 tokens
pub fn build_cw20_transfer_msg(
  _from_address: &Addr,
  to_address: &Addr,
  cw20_token_address: &Addr,
  amount: Uint128,
) -> Result<SubMsg, ContractError> {
  // perform CW20 transfer from sender to contract.  note that the cw20
  // token allowance for this contract must be set.
  let msg = SubMsg::new(WasmMsg::Execute {
    contract_addr: cw20_token_address.clone().into(),
    msg: to_binary(&Cw20ExecuteMsg::Transfer {
      recipient: to_address.clone().into(),
      amount,
    })?,
    funds: vec![],
  });
  Ok(msg)
}
