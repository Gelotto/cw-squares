#[cfg(not(feature = "library"))]
use crate::error::ContractError;
use crate::execute::add_player::add_player;
use crate::execute::buy_cells::buy_cells;
use crate::execute::resolve_winner::resolve_winner;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
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
    ExecuteMsg::AddPlayer { wallet, name, color } => add_player(deps, env, info, &wallet, name, color),
    ExecuteMsg::BuyCells { coordinates } => buy_cells(deps, env, info, &coordinates),
    ExecuteMsg::ResolveWinner { winner } => resolve_winner(deps, env, info, winner),
  }
}

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn query(
//   deps: Deps,
//   _env: Env,
//   msg: QueryMsg,
// ) -> StdResult<Binary> {
//   let result = match msg {
//     QueryMsg::IsAllowed { principal, action } => to_binary(&is_allowed(deps, &principal, &action)?),
//   }?;
//   Ok(result)
// }
