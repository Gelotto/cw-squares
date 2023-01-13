use crate::{
  error::ContractError,
  state::{GAME, GRID, PLAYERS},
  util::{build_cw20_transfer_msg, build_native_send_msg, compute_amount_from_pct},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, Uint128};

pub fn claim_refund(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
) -> Result<Response, ContractError> {
  let game = GAME.load(deps.storage)?;

  // game must be over and refundable
  if !(game.is_over() && game.can_claim_refund) {
    return Err(ContractError::NotAuthorized {});
  }

  let mut resp = Response::new().add_attributes(vec![attr("action", "claim_refund")]);

  // only existing players can claim refund
  if let Some(mut player) = PLAYERS.may_load(deps.storage, info.sender.clone())? {
    if player.has_claimed_refund.unwrap_or(false) {
      return Err(ContractError::AlreadyClaimedRefund {});
    }
    if let Some(positions) = &player.positions {
      // tabulate total amount spent by player
      let mut total_spend = Uint128::zero();
      for p in positions.iter() {
        let cell = GRID.load(deps.storage, p.coords.clone())?;
        total_spend += cell.price;
      }

      // compute refund amount
      let final_quarter = &game.quarters[game.quarters.len() - 1];
      let refund_amount = compute_amount_from_pct(total_spend, final_quarter.pct);

      // add refund transfer msg to response
      resp = match &game.token {
        crate::models::Token::Native { denom } => {
          resp.add_message(build_native_send_msg(&info.sender, &denom, refund_amount)?)
        },
        crate::models::Token::Cw20 {
          address: cw20_token_address,
        } => resp.add_submessage(build_cw20_transfer_msg(
          &info.sender,
          &cw20_token_address,
          refund_amount,
        )?),
      }
    }

    // flag the player as refunded to prevent double-claims
    player.has_claimed_refund = Some(true);

    PLAYERS.save(deps.storage, info.sender.clone(), &player)?;

    Ok(resp)
  } else {
    // claimant is not a registered player
    Err(ContractError::NotAuthorized {})
  }
}
