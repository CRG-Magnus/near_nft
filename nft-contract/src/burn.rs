use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_burn(
        &mut self, 
        token_id: TokenId
    ) {
        
        assert_one_yocto();

        // check that sender is owner of the token
        // expect to recieve a token or panic
        let token = self.tokens_by_id.get(&token_id).expect("No token"); // token obj and token owner
        let sender = env::predecessor_account_id();
        assert_eq!(
            token.owner_id, sender, 
            "Sender is not the owner of the token"
        );
        
        //we remove the token from the owner
        self.internal_remove_token_from_owner(&sender, &token_id);

        // remove token from metadata mapping
        self.token_metadata_by_id.remove(&token_id);

        // remove token token id mapping
        self.tokens_by_id.remove(&token_id);

        // Construct the burn log as per the events standard.
        let nft_burn_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftBurn(vec![NftBurnLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_burn_log.to_string());
    }
}