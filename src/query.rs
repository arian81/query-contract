use crate::msg::{
    AnchorOracleMsg, AstroPortSwapMsg, LunaPriceResponse, PriceResponse,
    SimulateSwapOperationsResponse, SwapOperation,
};
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, Empty, QueryRequest, StdResult, Uint128, WasmQuery,
};

pub fn encode_msg_query(msg: Binary, address: Addr) -> StdResult<QueryRequest<Empty>> {
    Ok(WasmQuery::Smart {
        contract_addr: address.to_string(),
        msg: msg,
    }
    .into())
}

pub fn try_get_luna_price(deps: Deps) -> StdResult<LunaPriceResponse> {
    let msg = AnchorOracleMsg::Price {
        base: String::from("terra1u0t35drzyy0mujj8rkdyzhe264uls4ug3wdp3x"),
        quote: String::from("uusd"),
    };

    // DONT DO THIS IN PROD
    let anchor_oracle_address =
        Addr::unchecked(String::from("terra1p4gg3p2ue6qy2qfuxtrmgv2ec3f4jmgqtazum8"));
    let wasm = encode_msg_query(to_binary(&msg).unwrap(), anchor_oracle_address)?;
    let query_response: PriceResponse = deps.querier.query(&wasm.into())?;

    let our_response: LunaPriceResponse = LunaPriceResponse {
        price: query_response.rate,
    };

    Ok(our_response)
}

pub fn try_swap_ust_luna_astro(
    deps: Deps,
    value: u128,
) -> StdResult<SimulateSwapOperationsResponse> {
    let myoperations: SwapOperation = SwapOperation::NativeSwap {
        offer_denom: String::from("uusd"),
        ask_denom: String::from("uluna"),
    };
    let msg = AstroPortSwapMsg::SimulateSwapOperations {
        offer_amount: Uint128::from(value),
        operations: vec![myoperations],
    };

    // DONT DO THIS IN PROD
    let astroport_addr =
        Addr::unchecked(String::from("terra13wf295fj9u209nknz2cgqmmna7ry3d3j5kv7t4"));
    let wasm = encode_msg_query(to_binary(&msg).unwrap(), astroport_addr)?;
    let query_response: SimulateSwapOperationsResponse = deps.querier.query(&wasm.into())?;

    let our_response: SimulateSwapOperationsResponse = SimulateSwapOperationsResponse {
        amount: query_response.amount,
    };

    Ok(our_response)
}

pub fn try_swap_ust_luna_tswap(
    deps: Deps,
    value: u128,
) -> StdResult<SimulateSwapOperationsResponse> {
    let myoperations: SwapOperation = SwapOperation::NativeSwap {
        offer_denom: String::from("uusd"),
        ask_denom: String::from("uluna"),
    };
    let msg = AstroPortSwapMsg::SimulateSwapOperations {
        offer_amount: Uint128::from(value),
        operations: vec![myoperations],
    };

    // DONT DO THIS IN PROD
    let astroport_addr =
        Addr::unchecked(String::from("terra1c58wrdkyc0ynvvxcv834kz65nfsxmw2w0pwusq"));
    let wasm = encode_msg_query(to_binary(&msg).unwrap(), astroport_addr)?;
    let query_response: SimulateSwapOperationsResponse = deps.querier.query(&wasm.into())?;

    let our_response: SimulateSwapOperationsResponse = SimulateSwapOperationsResponse {
        amount: query_response.amount,
    };

    Ok(our_response)
}
