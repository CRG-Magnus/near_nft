var fs = require('fs');

const startId = 10;
const copies = 10; // Number of NFTs minted
const contractName = "collectibles.paramunz.testnet"; // NFT contract address
const account = await near.account("paramunz.testnet"); // signer account
let args = {
    token_id:"",
    metadata: {
        title: "Badge of support",
        description: "",
        attributes: [
        {
            trait_type: "Rarity",
            value: "Archetype"
        },
        {
            trait_type: "Season",
            value: "0 - Alphaville"
        }
        ]
    }
  }
 
function mint() {
    for (let index = startId; index < startId+copies; index++) {
        args.token_id = index.toString();
        let SignAndSendTransactionOptions = {
            receiverId: contractName,
            actions: [
                nearAPI.transactions.functionCall("nft_mint", Buffer.from(JSON.stringify(args)), 10000000000000, "1"),
            ]
        }
        const result = account.signAndSendTransaction(SignAndSendTransactionOptions);
        fs.appendFileSync('logs/mints.json', result+"\n\n");
    }
}

mint();





// ---- EXAMPLES ---- //

// [{"type":"FunctionCall","method_name":"ft_transfer","args":"eyJyZ….MCJ9","gas":"10000000000000","deposit":"1"}]
// nearAPI.transactions.functionCall("ft_mint", Buffer.from(JSON.stringify(newArgs)), 10000000000000, "0")



/* action functioncall
pub struct FunctionCallAction {
    /// Name of exported Wasm function
    pub method_name: String,
    /// Serialized arguments
    pub args: Vec<u8>,
    /// Prepaid gas (gas_limit) for a function call
    pub gas: Gas,
    /// Amount of tokens to transfer to a receiver_id
    pub deposit: Balance,
}




/* js example https://docs.near.org/docs/api/naj-quick-reference#call-contract
await contract.method_name(
  {
    arg_name: "value", // argument name and value - pass empty object if no args required
  },
  "300000000000000", // attached GAS (optional)
  "1000000000000000000000000" // attached deposit in yoctoNEAR (optional)
);
*/



// EXAMPLE FROM MULTISIG
/*
const fs = require('fs');
require('dotenv').config()

const account = await near.account("naramunzdev.testnet");
const contractName = "multisig4.naramunzdev.testnet";
const methodNames = ["add_request", "delete_request", "confirm"];
const newArgs = { "num_confirmations": 2 };
const SignAndSendTransactionOptions = {
    receiverId: contractName,
    actions: [
        nearAPI.transactions.createAccount(), // creates the contractName account.
        nearAPI.transactions.transfer("5000000000000000000000000"), // 5 Near
        nearAPI.transactions.addKey(
            nearAPI.utils.PublicKey.from(process.env.PUBLIC_KEY_1), // nils.testnet - near-credentials
            nearAPI.transactions.functionCallAccessKey(contractName, methodNames, null)),
        nearAPI.transactions.addKey(
            nearAPI.utils.PublicKey.from(process.env.PUBLIC_KEY_2), // magnusb.testnet - near-credentials
            nearAPI.transactions.functionCallAccessKey(contractName, methodNames, null)),
        nearAPI.transactions.addKey(
            nearAPI.utils.PublicKey.from(process.env.PUBLIC_KEY_3), // robban.testnet - near-credentials
            nearAPI.transactions.functionCallAccessKey(contractName, methodNames, null)),
        nearAPI.transactions.deployContract(fs.readFileSync("res/multisig.wasm")),
        nearAPI.transactions.functionCall("new", Buffer.from(JSON.stringify(newArgs)), 10000000000000, "0"),
    ]
}
const result = account.signAndSendTransaction(SignAndSendTransactionOptions);
console.log("Multisig contract deployes and initialized!\ncontract host account: "+contractName);

// args obj         =>      {"receiver_id":"magnusb.testnet","amount":"100000000"}
// json to base64   =>      https://codebeautify.org/json-to-base64-converter

// CLI call example
/*
near call $ID add_request '{"request":{"receiver_id":"token_address","actions":[{"type":"FunctionCall","method_name":"ft_transfer","args":"eyJyZ….MCJ9","gas":"10000000000000","deposit":"1"}]}}' --accountId $ID
*/