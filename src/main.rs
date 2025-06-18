mod balances;
mod system;
mod support;
mod proof_of_existence;

use crate::support::Dispatch;
// This is our main Runtime.
// It accumulates all of the different pallets we want to use.

mod types {
    pub type AccountID = String;
    pub type Balance = u128;
    pub type Nonce = u32;
    pub type BlockNumber = u32;
    //TODO: Define a concrete `Extrinsic` type using `AccountId` and `RuntimeCall`.
    pub type Extrinsic = crate::support::Extrinsic<AccountID, crate::RuntimeCall>;
    //TODO: Define a concrete `Header` type using `BlockNumber`.
    pub type Header = crate::support::Header<BlockNumber>;
    //TODO: Define a concrete `Block` type using `Header` and `Extrinsic`.
    pub type Block = crate::support::Block<Header, Extrinsic>;
    /* TODO: Add the concrete `Content` type for your runtime. */
    pub type Content = &'static str;
}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
    /* TODO: Turn this into a nested enum where variant `Balances` contains a `balances::Call`. */
	//BalancesTransfer {to: types::AccountID, amount: types::Balance}, /* refactored out */
    Balances(balances::Call<Runtime>),
    /* TODO: Add a `ProofOfExistence` variant to access `proof_of_existence::Call`. */
    ProofOfExistence(proof_of_existence::Call<Runtime>),
}

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
    proof_of_existence: proof_of_existence::Pallet<Self>,
    
}
impl system::Config for Runtime {
	type AccountID = types::AccountID;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		/* TODO: Create a new `Runtime` by creating new instances of `system` and `balances`. */
		Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
            proof_of_existence: proof_of_existence::Pallet::new(),
        }
	}

    // Execute a block of extrinsics. Increments the block number.
	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		//Increment the system's block number.
        self.system.inc_block_number();
        //Check that the block number of the incoming block matches the current block number, or return an error.
        if block.header.block_number != self.system.block_number() {
            return Err("Block number is not as expected")
        }
        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            // do stuff with `caller` and `call`
            // Increment the nonce of the caller.
            self.system.inc_nonce(&caller);
            // Dispatch the extrinsic using the `caller` and the `call` contained in the extrinsic.
            // You can extend the error message to include information like the block number and extrinsic number.
            let _res = self.dispatch(caller, call).map_err(|e| 
                eprintln!(
                    "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                    block.header.block_number, i, e)
                );
        }
		Ok(())
	}
}

impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountID;
	type Call = RuntimeCall;
	// Dispatch a call on behalf of a caller. Increments the caller's nonce.
	//
	// Dispatch allows us to identify which underlying module call we want to execute.
	// Note that we extract the `caller` from the extrinsic, and use that information
	// to determine who we are executing the call on behalf of.
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
        match runtime_call {
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            },
            RuntimeCall::ProofOfExistence(call) => {
                self.proof_of_existence.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}


fn main() {
    let mut runtime = Runtime::new();
    //println!("The before state system pallet contents is {:#?}", runtime.system);
    //println!("The before state balances pallet contents is {:#?}", runtime.balances);
    
    let alice = String::from("Alice"); //remove reference
    let bob = String::from("Bob"); //remove reference
    let charlie = String::from("Charlie"); //remove reference
    let lozza = String::from("Lozza");

    runtime.balances.set_balance(&alice, 100);
    
    //println!("The after state system pallet contents is {:#?}", runtime.system);
    //println!("The after state balances pallet contents is {:#?}", runtime.balances);

    /*
		TODO: Replace the logic above with a new `Block`.
			- Set the block number to 1 in the `Header`.
			- Move your existing transactions into extrinsic format, using the
			  `Extrinsic` and `RuntimeCall`.
	*/
    let block_1 = types::Block {
	    header: support::Header { block_number: 1 },
	    extrinsics: vec![
		    support::Extrinsic {
		    	caller: alice.clone(),
		    	call: RuntimeCall::Balances(balances::Call::Transfer{to: bob.clone(), amount: 30 }),
		    },
            support::Extrinsic {
		    	caller: alice.clone(),
		    	call: RuntimeCall::Balances(balances::Call::Transfer{to: charlie.clone(), amount: 20 }),
		    },
	    ],
    };
    //lozza makes a bold claim. Lets hope it doesnt get revoked!
    let block_2 = types::Block {
	    header: support::Header { block_number: 2 },
	    extrinsics: vec![
		    support::Extrinsic {
		    	caller: alice.clone(),
		    	call: RuntimeCall::Balances(balances::Call::Transfer{to: lozza.clone(), amount: 40 }),
		    },
            support::Extrinsic {
		    	caller: charlie.clone(),
		    	call: RuntimeCall::Balances(balances::Call::Transfer{to: lozza.clone(), amount: 10 }),
		    },
            support::Extrinsic {
		    	caller: lozza.clone(),
		    	call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim { claim: ("Lozza is going (wants!) to PBA Bali 2025 ;) !!!") }),
		    },
	    ],
    };
    //charlie please no!
    let block_3 = types::Block {
	    header: support::Header { block_number: 3 },
	    extrinsics: vec![
            support::Extrinsic {
		    	caller: charlie.clone(),
		    	call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::RevokeClaim { claim: ("Lozza is going (wants!) to PBA Bali 2025 ;) !!!") }),
		    },
	    ],
    };
    //alices runs out of DOT
    let block_4 = types::Block {
	    header: support::Header { block_number: 4 },
	    extrinsics: vec![
            support::Extrinsic {
		    	caller: alice.clone(),
		    	call: RuntimeCall::Balances(balances::Call::Transfer { to: charlie.clone(), amount: 20 } ),
		    },
	    ],
    };

    runtime.execute_block(block_1).expect("invalid block");
    println!("{:#?}", runtime);
    runtime.execute_block(block_2).expect("invalid block");
	println!("{:#?}", runtime);
    runtime.execute_block(block_3).expect("invalid block");
	println!("{:#?}", runtime);
    runtime.execute_block(block_4).expect("invalid block");
	println!("{:#?}", runtime);
}
