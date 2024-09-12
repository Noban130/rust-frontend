pub use solana_client::rpc_client::RpcClient;
pub use solana_program::pubkey::Pubkey;
#[allow(unused_imports)]
pub use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction },
    message::Message,
    signature::Signature,
    signature::{Keypair, Signer},
    signer::EncodableKey,
    system_program,
    transaction::Transaction,
};
use std::str::FromStr;
pub use mpl_token_metadata::accounts::Metadata;
// use borsh::{BorshDeserialize, BorshSerialize};


fn main() {
    // Step 1: Create an RPC client (for Devnet)
    let url = "https://api.devnet.solana.com".to_string();
     let timeout = std::time::Duration::from_secs(50);
     let connection = RpcClient::new_with_timeout(url, timeout);
     
     // Step 2: Specify the account Pubkey (this is just an example Pubkey)
     // let account_pubkey = Pubkey::from_str("GsX4b44N2vkDjnZPLucGV7ou5qxADN2N6BZ7zU8vnJ1X").unwrap();
     let program_id = Pubkey::from_str("GsX4b44N2vkDjnZPLucGV7ou5qxADN2N6BZ7zU8vnJ1X").unwrap();
     let payer = Keypair::read_from_file("src/wallet-keypair.json").unwrap();
    
    let seed_text = b"final_seed";
    // Convert string to &[u8]
    let seed_text_slice: &[u8] = seed_text;
    let (account_new, _) = Pubkey::find_program_address(&[&seed_text_slice,&payer.pubkey().to_bytes()], &program_id);

    let mut value1: Vec<f64> = Vec::new();
    let mut value2: Vec<f64> = Vec::new();
    // Step 3: Fetch the account data
    match connection.get_account_data(&account_new) {
        Ok(data) => {
            // let meta: Metadata = Metadata::from_bytes(&data).unwrap();
            for i in 0..80 {
                let start = i * 8;
                let end = start + 8;
                let bytes: [u8; 8] = data[start..end].try_into().expect("slice with incorrect length");
                value1.push(f64::from_le_bytes(bytes));
            }
            println!("slope: {:?}", value1);
            // Decode `value2` (next `num_elements_value2` f64 values)
            let offset = (80 + 1) * 8;
            for i in 0..80 {
                let start = offset + i * 8;
                let end = start + 8;
                let bytes: [u8; 8] = data[start..end].try_into().expect("slice with incorrect length");
                value2.push(f64::from_le_bytes(bytes));
    }
    println!("intercept: {:?}", value2);
        }
        Err(e) => {
            eprintln!("Error retrieving account data: {:?}", e);
        }
    }
}