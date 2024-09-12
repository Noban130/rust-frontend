use std::error::Error;
// use std::iter::repeat_with;
use anchor_client;
#[allow(unused_imports)]
pub use borsh::{BorshDeserialize, BorshSerialize};
pub use solana_client::rpc_client::RpcClient;
#[allow(unused_imports)]
pub use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::Signature,
    signature::{Keypair, Signer},
    signer::EncodableKey,
    system_program,
    transaction::Transaction,
};
use std::str::FromStr;
#[tokio::main]
async  fn main() -> Result<(), Box<dyn Error>>{
    let url = "https://api.devnet.solana.com".to_string();
     let timeout = std::time::Duration::from_secs(50);
     let connection = RpcClient::new_with_timeout(url, timeout);
     let program_id = Pubkey::from_str("GsX4b44N2vkDjnZPLucGV7ou5qxADN2N6BZ7zU8vnJ1X").unwrap();
     // let account_new = Keypair::new().pubkey();
     let payer = Keypair::read_from_file("src/wallet-keypair.json").unwrap();
    //  let mut seed_text = if transaction_num == 0 {
    //     b"new_init_seed"[..12].to_vec()
    // } else {
    //     b"save_added_seed"[..15].to_vec()
    // };
    let seed_text = b"final_seed";
    // Convert string to &[u8]
    let seed_text_slice: &[u8] = seed_text;
    let (account_new, _) = Pubkey::find_program_address(&[&seed_text_slice,&payer.pubkey().to_bytes()], &program_id);
    // Check if the PDA account exists
    match connection.get_account(&account_new) {
        Ok(account) => {
            println!("PDA account exists!");

            // Deserialize the data if needed
            // Assuming you have a struct for the account data
            if account.data.len() > 0 {
                println!("PDA account is initialized!: {:?}", account.data);
                // You can deserialize the account data here to check its state
                // instruction_name = "save_data"
            } else {
                println!("PDA account exists but is not initialized.");
            }
        }
        Err(_) => {
            println!("PDA account does not exist or is not initialized.");
        }
    }
    Ok(())
        
}