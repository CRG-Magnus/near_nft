use crate::*;
use near_sdk::Gas;

const MIN_GAS_FOR_ADD_MINTER_CALL: Gas = Gas(5_000_000_000_000);

// Minting permission administration @mbt
#[near_bindgen]
impl Contract {
    // Add minter
    // - who can call this?
    #[payable]
    pub fn minter_add(
        &mut self, 
        account_id: &AccountId
    ) {

        assert_at_least_one_yocto();

        //get the GAS attached to the call
        let attached_gas = env::prepaid_gas();

        assert!( 
            attached_gas >= MIN_GAS_FOR_ADD_MINTER_CALL,
            "You cannot attach less than {:?} Gas to nft_transfer_call",
            MIN_GAS_FOR_ADD_MINTER_CALL
        );

        // measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        // check if sender is a minter and therefore has permission to add another minter
        assert!(
            self.minters.contains(&env::predecessor_account_id()),
            "Sender has no minter administration permission"
        );

        // check if account is already a minter
        assert!(
            self.minters.contains(account_id),
            "Account is already a minter!"
        );

        //we insert the account into the set of minters
        self.minters.insert(account_id);

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);

        //log when a new minter is added
        env::log_str(&format!("New minter added").to_string());
    }

    // Remove minter
    #[payable]
    pub fn minter_remove(
        &mut self, 
        account_id: &AccountId
    ) {
        assert_one_yocto(); // needed?

        // check if sender is a minter and therefore has permission to remove another minter
        assert!(
            self.minters.contains(&env::predecessor_account_id()),
            "Sender has no minter administration permission"
        );

        // check if account to remove is a minter
        assert!(
            self.minters.contains(account_id),
            "Account is already a minter!"
        );

        // check minter can not remove own account
        assert_ne!(
            account_id, env::predecessor_account_id(), 
            "Can not remove own account"
        );

        //we remove the token ID from the set
        self.minters.remove(account_id);

        //log when a new minter is added
        env::log_str(&format!("Minter removed").to_string());
    }

    // get amount of minters
    pub fn minter_amount(
        &self
    ) -> U128 {
        return U128(self.minters.len() as u128);
    }

    // check if account is minter
    pub fn is_minter(
        &self,
        account_id: &AccountId
    ) -> bool {
        return self.minters.contains(account_id);
    }
}
    