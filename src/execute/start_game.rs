use crate::{
  error::ContractError,
  models::Game,
  state::{require_admin, GAME},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

pub fn start_game(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
) -> Result<Response, ContractError> {
  GAME.update(deps.storage, |mut game| -> Result<Game, ContractError> {
    // address executing this request must be admin
    require_admin(&game, &info.sender)?;
    if game.has_started {
      return Err(ContractError::AlreadyStarted {});
    }
    game.has_started = true;
    Ok(game)
  })?;
  Ok(Response::new().add_attributes(vec![attr("action", "start_game")]))
}
