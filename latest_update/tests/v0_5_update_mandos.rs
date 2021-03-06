use auction_mock::*;
extern crate delegation_latest_update;
use delegation_latest_update::*;
use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../../../auction-mock/output/auction-mock.wasm",
        Box::new(|context| Box::new(AuctionMockImpl::new(context))),
    );

    contract_map.register_contract(
        "file:../output/delegation_latest_update.wasm",
        Box::new(|context| Box::new(DelegationImpl::new(context))),
    );
    contract_map
}

#[test]
fn genesis_addr_fix() {
    parse_execute_mandos("mandos/genesis_addr_fix.scen.json", &contract_map());
}
