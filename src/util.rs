use cosmwasm_std::{to_binary, Addr, BankMsg, Coin, CosmosMsg, SubMsg, Uint128, WasmMsg};
use cw20::Cw20ExecuteMsg;

use crate::error::ContractError;

pub fn compute_amount_from_pct(
  total: Uint128,
  pct: u8,
) -> Uint128 {
  Uint128::from((total.u128() * pct as u128) / 100)
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
