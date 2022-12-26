use avalanche_vm::{
    precompiles::Precompiles,
    state::State,
    vm::{ActionParams, Ext, Vm},
};
use ethereum_types::{Address, H256, U256};
use std::sync::Arc;

struct MyVm;

impl Ext for MyVm {
    fn balance(&self, _address: &Address) -> U256 {
        // Return a balance for the given address.
        U256::from(100)
    }

    fn execution_result(&mut self) -> Result<(), String> {
        // Return the result of the execution.
        Ok(())
    }

    fn call(&mut self, _params: ActionParams, _code: &[u8]) -> Result<(), String> {
        // Execute a call to another contract.
        Ok(())
    }

    // Other Ext functions that you can implement as needed.
}

fn main() {
    // Create a new instance of the MyVm struct.
    let ext = MyVm;

    // Create a new virtual machine and pass in the MyVm instance as the Ext implementation.
    let mut vm = Vm::new(Arc::new(ext), Precompiles::default());

    // Create a new ActionParams struct with the code that you want to execute.
    let params = ActionParams {
        code_address: Address::zero(),
        code_hash: H256::zero(),
        data: Vec::new(),
        address: Address::zero(),
        origin: Address::zero(),
        caller: Address::zero(),
        gas: U256::max_value(),
        gas_price: U256::from(1),
        value: U256::from(0),
        apparent_value: U256::from(0),
        input_data: Vec::new(),
        call_type: Default::default(),
    };

    // Execute the code on the virtual machine.
    vm.exec(params, State::InMemory).unwrap();
}
