use cosmwasm_std::{Addr, Deps, StdResult};

use crate::msg::BooleanResponse;

pub fn get_game(
  deps: Deps,
  address: &Addr,
) -> StdResult<BooleanResponse> {
  Ok(BooleanResponse { value: true })
}
