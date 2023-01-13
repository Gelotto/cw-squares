use crate::{
  error::ContractError,
  models::Player,
  state::{require_admin, GAME, PLAYERS},
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

pub fn register_player(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  player_addr: &Addr,
  name: Option<String>,
  color: Option<String>,
) -> Result<Response, ContractError> {
  let game = GAME.load(deps.storage)?;

  // address executing this request must be admin
  require_admin(&game, &info.sender)?;

  // game must be new, not started and not over
  if game.is_over() {
    return Err(ContractError::GameOver {});
  }
  if game.has_started {
    return Err(ContractError::AlreadyStarted {});
  }

  // "register" (i.e. save) the player. Now they can buy cells
  PLAYERS.update(
    deps.storage,
    player_addr.clone(),
    |some_player| -> Result<Player, ContractError> {
      if some_player == None {
        Ok(Player {
          wallet: player_addr.clone(),
          positions: None,
          has_claimed_refund: Some(false),
          name,
          color,
        })
      } else {
        Err(ContractError::DuplicatePlayerAddress {})
      }
    },
  )?;

  Ok(Response::new().add_attributes(vec![attr("action", "add_player")]))
}
