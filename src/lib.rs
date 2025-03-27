#[cfg(test)] mod tests {
    use solana_sdk::{message::Message,signature::{Keypair, Signer, read_keypair_file}, transaction::Transaction};
    use::bs58;
    use std::io::{self, BufRead};
    use std::str::FromStr;
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey,system_instruction::transfer, hash::hash}; 

    const RPC_URL: &str = "https://api.devnet.solana.com";

    // change of program address 
    // ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa
    
    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string()); println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");

        println!("{:?}", kp.to_bytes());
    } 
    //ETRYwRSSU9Se3U81Av25KPyctt3tUM6DubTLviTCGbAf

    #[test]
    fn base58_to_wallet() {

        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap(); println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap(); println!("{:?}", wallet);

    }

    #[test]
    fn wallet_to_base58() {

        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();
        let wallet =

        stdin.lock().lines().next().unwrap().unwrap().trim_start_matches('[').trim_end_matches(']').
        split(',') .map(|s| s.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>();

        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string(); println!("{:?}", base58);

    }
    
    #[test] 
    fn airdop() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        // Connected to Solana Devnet RPC Client
        let client = RpcClient::new(RPC_URL);
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {

            Ok(s) => {
            println!("Success! Check out your TX here:");
            
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            
            },
            
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()) 
        };
    } 
    
    #[test] 
    fn transfer_sol() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        // With the imported Keypair, we can sign a new message.
        let pubkey = keypair.pubkey();
        let message_bytes = b"I verify my solana Keypair!";
        let sig = keypair.sign_message(message_bytes);
        let sig_hashed = hash(sig.as_ref());
        // After that we can verify the singature, using the default implementation
        match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
        true => println!("Signature verified"),
        false => println!("Verification failed"),
        }

        // Define our Turbin3 public key
        let to_pubkey = Pubkey::from_str("76yGe1GaxLUjwCYC18dwQ2g5vPt5d4uefer52jtqhi1V").unwrap();
        
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);

        // Get recent blockhash
        let recent_blockhash = rpc_client 
        .get_latest_blockhash() 
        .expect("Failed to get recent blockhash");

        // Get balance of dev wallet
        let balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get balance");

        // Create a test transaction to calculate fees
        let message = Message::new_with_blockhash(
            &[transfer( &keypair.pubkey(), &to_pubkey, balance,)], 
            Some(&keypair.pubkey()), &recent_blockhash
            );

        // Calculate fees
        let fee = rpc_client.get_fee_for_message(&message) .expect("Failed to get fee calculator");

        // Deduct fee from lamports amount and create a TX with correct balance
        let transaction = Transaction::new_signed_with_payer(
            &[transfer( &keypair.pubkey(), &to_pubkey, balance - fee,)], 
            Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash);

        // // transaction of 0.001 SOL
        // let transaction = Transaction::new_signed_with_payer( &[transfer(
        //     &keypair.pubkey(), &to_pubkey, 
        //     1_000_000)], 
        //     Some(&keypair.pubkey()), 
        //     &vec![&keypair], recent_blockhash
        // );

        // Send the transaction
        let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");

        // Print transaction 
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",signature);

    }
    }