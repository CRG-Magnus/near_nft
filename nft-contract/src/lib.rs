use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
};

use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::approval::*;
pub use crate::royalty::*;
pub use crate::events::*;
pub use crate::burn::*;
pub use crate::permission::*;

mod internal;
mod approval;
mod enumeration;
mod metadata;
mod mint;
mod nft_core;
mod royalty;
mod events;
mod burn;
mod permission;

/// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "2.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";
/// Collection image in optimized svg as a data URI
const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/png;base64,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";
/// Collection name
const COLLECTION_NAME: &str = "Paramunz - Collectibles";
/// Collection symbol
const COLLECTION_SYMBOL: &str = "CBL";
/// Mint permission
const MINTER: &str = "paramunz.testnet";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,

    //mint permission accounts @mbt
    pub minters: UnorderedSet<AccountId>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
    Minters, // @mbt
}

#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in 
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: COLLECTION_NAME.to_string(),
                symbol: COLLECTION_SYMBOL.to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id. 
    */
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        //create a variable of type Self with all the fields initialized. 
        let mut this = Self { // @mbt mut added
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            //set the owner_id field equal to the passed in owner_id. 
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            // @mbt
            minters: UnorderedSet::new(   
                StorageKey::Minters.try_to_vec().unwrap(), // try.to.vec, unwrap -> 'Serialize this instance into a vector of bytes.'
            ),
        };

        //insert pre-defined minter @mbt
        let minter = AccountId::new_unchecked(MINTER.to_string());
        this.minters.insert(&minter);

        //return the Contract object
        this
    }
}


// unit tests @mbt
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    use super::*;

    const MINT_STORAGE_COST: u128 = 5870000000000000000000;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn sample_token_metadata() -> TokenMetadata {
        TokenMetadata {
            title: Some("Olympus Mons".into()),
            description: Some("The tallest mountain in the charted solar system".into()),
            media: None,
            media_hash: None,
            copies: Some(1u64),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        }
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.minters.len(), 1);
    }

    #[test]
    #[should_panic]
    fn test_mint() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());

        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), sample_token_metadata(), accounts(0), None);
    }
}