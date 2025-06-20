use core::fmt::Debug;
use std::collections::BTreeMap;
use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
	/// The type which represents the content that can be claimed using this pallet.
	/// Could be the content directly as bytes, or better yet the hash of that content.
	/// We leave that decision to the runtime developer.
	type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// A simple storage map from content to the owner of that content.
	/// Accounts can make multiple different claims, but each claim can only have one owner.
	/* TODO: Add a field `claims` which is a `BTreeMap` fom `T::Content` to `T::AccountId`. */
    claims: BTreeMap<T::Content, T::AccountID>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the Proof of Existence Module.
	pub fn new() -> Self {
		/* TODO: Return a new instance of the `Pallet` struct. */
        Self {
            claims: BTreeMap::new(),
        }
	}

    /// Get the owner (if any) of a claim.
	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountID> {
		/* TODO: `get` the `claim` */
		match self.claims.get(claim) {
            Some(account) => Some(account),
            None => None,
        }
	}

	/// Create a new claim on behalf of the `caller`.
	/// This function will return an error if someone already has claimed that content.
	pub fn create_claim(&mut self, caller: T::AccountID, claim: T::Content) -> DispatchResult {
		/* TODO: Check that a `claim` does not already exist. If so, return an error. */
            if self.claims.contains_key(&claim) {
                return Err("Claim already exists");
            } else {
                // If the claim does not exist, we can proceed to insert it.
                self.claims.insert(claim, caller);
            }
		Ok(())
	}

	/// Revoke an existing claim on some content.
	/// This function should only succeed if the caller is the owner of an existing claim.
	/// It will return an error if the claim does not exist, or if the caller is not the owner.
	pub fn revoke_claim(&mut self, caller: T::AccountID, claim: T::Content) -> DispatchResult {
		/* TODO: Get the owner of the `claim` to be revoked. */
        let owner_verif = self.get_claim(&claim).ok_or("Claim doesnt exist")?;
        //TODO: Check that the `owner` matches the `caller`.
        if caller != *owner_verif {
            return Err("Caller is not the verified owner");
        }
        //TODO: If all checks pass, then `remove` the `claim`.
        self.claims.remove(&claim);
		Ok(())
	}
}

// A public enum which describes the calls we want to expose to the dispatcher.
// We should expect that the caller of each call will be provided by the dispatcher,
// and not included as a parameter of the call.
pub enum Call<T: Config> {
	/*
		TODO:
		Create variants for:
		- `CreateClaim`
		- `RevokeClaim`

		Remember that you only need to pass in the `claim` data, as `caller` information is passed
		in through the `dispatch` logic.
	*/
    CreateClaim {claim: T::Content},
    RevokeClaim {claim: T::Content},
}

/// Implementation of the dispatch logic, mapping from `POECall` to the appropriate underlying
/// function we want to execute.
/*
	TODO:
	Implement `crate::support::Dispatch` for `Pallet<T>`.

	In your `dispatch` logic, match on `call` and forward the `caller` and `claim` data to the
	appropriate function.
*/
impl<T: Config> crate::support::Dispatch for Pallet<T> {
	type Caller = T::AccountID;
	type Call = Call<T>;

	fn dispatch(
		&mut self,
		caller: Self::Caller,
		call: Self::Call,
	) -> crate::support::DispatchResult {
        match call {
            Call::CreateClaim { claim } => {
                self.create_claim(caller, claim)?;
            },
            Call::RevokeClaim { claim } => {
                self.revoke_claim(caller, claim)?;
            },
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = &'static str;
	}

	impl crate::system::Config for TestConfig {
		type AccountID = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		/*
			TODO:
			Create an end to end test verifying the basic functionality of this pallet.
				- Check the initial state is as you expect.
				- Check that all functions work successfully.
				- Check that all error conditions error as expected.
		*/
	}
}
