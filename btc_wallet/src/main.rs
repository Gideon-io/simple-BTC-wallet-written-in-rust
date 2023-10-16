use std::path::Path;

use bdk::{Wallet, bitcoin::Network, database::{MemoryDatabase, SqliteDatabase}, blockchain::{self, ElectrumBlockchain}, SyncOptions, electrum_client::Client, wallet::AddressIndex};


/* 
Workshop format:
1. cargo new (for creating a new project)
2. hello world (automatically done when creating the new project)
3. read descriptor from env
   - cargo add dotenv for reading from file
   - cargo add anyhow for handling error
4. add bdk: so cargo add bdk for the crate
5. return addresses on each run ( simply comment out lines 'let blockchain = Elect.. and wallet.sync lines)
6. import AddressIndex
7. last unused
8. print address index and address.address
9. make it so the app uses sqlite to persist. Should return a new address on each run
   - feature sqlite (cargo add bdk --features sqlite)
*/
fn main() -> anyhow::Result<()> {  //was initially this before using anyhow 'Result<(), Box<dyn std::error::Error>>'. anyhow will assist with automatically handling all different error types

    dotenv::from_filename(".env")?; //1 part of the first section of just setting up env
    dotenv::dotenv()?; //part of the first section of just setting up env 

    let descriptor = std::env::var("WALLET_DESCRIPTOR")?;  //2 when adding this line of code initially, the env had a red underline. Utilise quick fix by'  command + . ' Useulf for imports 

    println!("{}", descriptor); //part of the first section of just setting up env

    //dbg!(descriptor);//part of the first section of just setting up env. Later commented out due to &descriptor.clone() below. Ownership issues

    let my_path: &Path = Path::new("paypaul.db");

    let wallet = Wallet::new(&descriptor, None, Network::Testnet, SqliteDatabase::new(my_path))?; //for part 4, step 1. cmd + click Wallet::new to go into the source file so you can have a look at what it requires and also utilise examples on create webpage.
                                                                                                                                                    // changed database from MemoryDatabase::default() to SqliteDatabase
    //let blockchain = ElectrumBlockchain::from(Client::new("ssl://electrum.blockstream.info:60002")?); //defining the blockchain here
    
   // wallet.sync(&blockchain, SyncOptions::default())?; //syncs blockchain data

    let balance = wallet.get_balance()?; //grabs balances of the wallet and puts it into balance

    dbg!(balance);  //dbg! is pretty much used to get rid of the yellow underline as an unsused variable and prints

    let address = wallet.get_address(AddressIndex::New)?;

    dbg!(address);

    let address = wallet.get_address(AddressIndex::New)?;

    dbg!(address);

    Ok(())//part of the first section of just setting up env
}
