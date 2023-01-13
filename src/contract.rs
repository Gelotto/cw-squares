#[cfg(not(feature = "library"))]
use crate::error::ContractError;
use crate::execute::buy_squares::buy_squares;
use crate::execute::choose_winner::choose_winner;
use crate::execute::claim_refund::claim_refund;
use crate::execute::register_player::register_player;
use crate::execute::start_game::start_game;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::query_game::query_game;
use crate::state;
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:cw-acl";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: InstantiateMsg,
) -> Result<Response, ContractError> {
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
  state::initialize(deps, &env, &info, &msg)?;
  Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: ExecuteMsg,
) -> Result<Response, ContractError> {
  match msg {
    ExecuteMsg::RegisterPlayer { wallet, name, color } => register_player(deps, env, info, &wallet, name, color),
    ExecuteMsg::StartGame {} => start_game(deps, env, info),
    ExecuteMsg::BuySquares {
      coordinates,
      player_name,
      player_color,
    } => buy_squares(deps, env, info, &coordinates, player_name, player_color),
    ExecuteMsg::ChooseWinner { winner } => choose_winner(deps, env, info, winner),
    ExecuteMsg::ClaimRefund {} => claim_refund(deps, env, info),
  }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
  deps: Deps,
  _env: Env,
  msg: QueryMsg,
) -> StdResult<Binary> {
  let result = match msg {
    QueryMsg::Game {
      with_grid,
      with_players,
    } => to_binary(&query_game(deps, with_grid, with_players)?),
  }?;
  Ok(result)
}
