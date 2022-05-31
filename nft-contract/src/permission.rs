use crate::*;
    
#[near_bindgen]
impl Contract {
    #[payable]
    pub fn add_minter(
        &mut self, 
        account_id: &AccountId
    ) {
        assert_one_yocto();

        // check if account is already a minter
        assert!(
            self.minters.contains(account_id),
            "Account is already a minter!"
        );

        //we insert the token ID into the set
        self.minters.insert(account_id);
    }

    #[payable]
    pub fn remove_minter(
        &mut self, 
        account_id: &AccountId
    ) {
        assert_one_yocto();

        // check if account is a minter
        assert!(
            self.minters.contains(account_id),
            "Account is already a minter!"
        );

        //we insert the token ID into the set
        self.minters.insert(account_id);
    }
}
    