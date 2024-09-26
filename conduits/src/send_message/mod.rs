use pwr_rs::Wallet;
use dotenvy::dotenv;
use std::env;
use serde_json::json;

pub async fn send_message() {
    dotenv().ok();
    // Setting up your wallet in the SDK
    let private_key = env::var("PRIVATE_KEY").unwrap();
    let wallet = Wallet::from_hex(&private_key).unwrap();

    let obj = json!({ "message": "please send me pwr" });
    let data = serde_json::to_vec(&obj).unwrap(); // Serialize to JSON bytes
    let vm_id = 123;
    // Sending the VM data transaction
    let res = wallet.send_vm_data(vm_id, data).await;
    println!("{}", res);
}