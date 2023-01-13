use crate::{
  error::ContractError,
  models::{GridCoordinates, Token},
  state::{require_admin, GAME, GRID},
  util::{build_cw20_transfer_msg, build_native_send_msg, compute_amount_from_pct},
};
use cosmwasm_std::{attr, Addr, CosmosMsg, DepsMut, Env, MessageInfo, Response, SubMsg, Uint128};

// addresses for base gelotto tax:
pub const GELOTTO_ADDR: &str = "juno1jume25ttjlcaqqjzjjqx9humvze3vcc8z87szj";
pub const GELOTTO_ANNUAL_GRAND_PRIZE_ADDR: &str = "juno1fxu5as8z5qxdulujzph3rm6c39r8427mjnx99r";
pub const GELOTTO_NFT_1_REWARDS_ADDR: &str = "juno1tlyqv2ss4p9zelllxm39hq5g6zw384mvvym6tp";

// percentage split of the 10% base gelotto tax:
pub const GELOTTO_PCT: u8 = 20;
pub const GELOTTO_ANNUAL_GRAND_PRIZE_PCT: u8 = 50;
pub const GELOTTO_NFT_1_REWARDS_PCT: u8 = 30;

pub fn choose_winner(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  winner: GridCoordinates,
) -> Result<Response, ContractError> {
  let mut game = GAME.load(deps.storage)?;

  // only game creator and resolve quarterly winners
  require_admin(&game, &info.sender)?;

  // game must be started and not over
  if game.is_over() {
    return Err(ContractError::GameOver {});
  }
  if !game.has_started {
    return Err(ContractError::NotStarted {});
  }

  let n_quarters: usize = game.quarters.len();
  let quarter = &mut game.quarters[game.quarter_index as usize];

  // can't resolve the same quarter winner twice
  if quarter.winner != None {
    return Err(ContractError::AlreadyResolved {});
  }

  // save the winning coordinates for the resolving quarter
  quarter.winner = Some(winner.clone());

  // calculate prize amount for the winning wallets, distributed evenly to each
  // player in the winning grid cell.
  let full_quarter_prize_amount = compute_amount_from_pct(game.token_amount, quarter.pct);

  // subtract 10%, as 10% is Gelotto's tax
  let quarter_prize_amount = compute_amount_from_pct(full_quarter_prize_amount, 90);

  // storage for transfer msgs to winners:
  let mut transfer_msgs: Vec<CosmosMsg> = vec![];
  let mut cw20_transfer_msgs: Vec<SubMsg> = vec![];

  // init response
  let mut resp = Response::new().add_attributes(vec![attr("action", "resolve_winner")]);

  if let Some(winning_cell) = GRID.may_load(deps.storage, winner)? {
    // get addrs of players in grid cell that won
    let winning_wallets = winning_cell.wallets.unwrap_or(vec![]);
    // if there are any....
    if winning_wallets.len() > 0 {
      // calc the amount owed to each winning player address
      let player_prize_pct = winning_wallets.len() as u8 / 100;
      let player_prize_amount = compute_amount_from_pct(quarter_prize_amount, player_prize_pct);

      // create a transfer message from the contract to winner address
      for addr in winning_wallets.iter() {
        match &game.token {
          Token::Native { denom } => transfer_msgs.push(build_native_send_msg(addr, denom, player_prize_amount)?),
          Token::Cw20 { address: cw20_addr } => {
            cw20_transfer_msgs.push(build_cw20_transfer_msg(&addr, &cw20_addr, player_prize_amount)?)
          },
        }
      }

      // add transfer msgs required by gelotto tax
      let gelotto_tax_amount = full_quarter_prize_amount - quarter_prize_amount;
      resp = append_gelotto_tax_msgs(&resp, &game.token, gelotto_tax_amount)?;
    } else if (game.quarter_index as usize) == (n_quarters - 1) {
      // There aren't any buyers for the winning square AND it's the last round.
      // In this case, put the contract in a state where each player can claim
      // a refund for their remaining balance.
      game.can_claim_refund = true;
    } else {
      // If we're here, then the winning square has no buyers. In this case, we
      // zero out the ending quarter's percent, and distribute it
      // proportionally to the remaining quarters.
      let ending_quarter_pct = quarter.pct;
      quarter.pct = 0;

      let remaining_quarters = &game.quarters.clone()[((game.quarter_index + 1) as usize)..];
      let total_future_pct: u8 = remaining_quarters.iter().map(|q| q.pct).sum();

      // increment the pct value of remaining quarters
      for (i, future_quarter) in remaining_quarters.iter().enumerate() {
        let ratio = future_quarter.pct / total_future_pct;
        let new_pct = future_quarter.pct + (ending_quarter_pct * ratio);
        let future_quarter_index = i + game.quarter_index as usize;
        game.quarters[future_quarter_index].pct = new_pct;
      }
      // if, due to integer arithmetic, the new total percent is less than 100,
      // add the difference, which should always only be 0 or 1, to the final quarter.
      let new_total_pct: u8 = game.quarters.iter().map(|q| q.pct).sum();
      if new_total_pct < 100 {
        game.quarters[n_quarters - 1].pct += 100 - new_total_pct;
      }
    }
  } else {
    // The given coordinates for the winning grid cell are out of bounds
    return Err(ContractError::CoordinatesOutOfBounds {});
  }

  // increment the quarter index, effectively moving to the next
  // quarter or, in the final case, ending the game
  game.quarter_index += 1;

  // save all changes to quarters through the game
  GAME.save(deps.storage, &game)?;

  // send response with transfer msgs to winning wallets
  Ok(resp.add_submessages(cw20_transfer_msgs).add_messages(transfer_msgs))
}

/// add transfer msgs to response for Gelotto's tax
fn append_gelotto_tax_msgs(
  resp: &Response,
  token: &Token,
  total_tax: Uint128,
) -> Result<Response, ContractError> {
  let mut params: Vec<(&str, u8)> = Vec::with_capacity(3);
  params.push((GELOTTO_ADDR, GELOTTO_PCT));
  params.push((GELOTTO_ANNUAL_GRAND_PRIZE_ADDR, GELOTTO_ANNUAL_GRAND_PRIZE_PCT));
  params.push((GELOTTO_NFT_1_REWARDS_ADDR, GELOTTO_NFT_1_REWARDS_PCT));

  match &token {
    Token::Native { denom } => {
      let mut msgs: Vec<CosmosMsg> = Vec::with_capacity(3);
      for (to_addr_str, pct) in params.iter() {
        let amount: Uint128 = compute_amount_from_pct(total_tax, *pct);
        msgs.push(build_native_send_msg(
          &Addr::unchecked(to_addr_str.to_string()),
          &denom,
          amount,
        )?);
      }
      Ok(resp.clone().add_messages(msgs))
    },
    Token::Cw20 {
      address: cw20_token_addr,
    } => {
      let mut msgs: Vec<SubMsg> = Vec::with_capacity(3);
      for (to_addr_str, pct) in params.iter() {
        let amount: Uint128 = compute_amount_from_pct(total_tax, *pct);
        msgs.push(build_cw20_transfer_msg(
          &Addr::unchecked(to_addr_str.to_string()),
          &cw20_token_addr,
          amount,
        )?);
      }
      Ok(resp.clone().add_submessages(msgs))
    },
  }
}
