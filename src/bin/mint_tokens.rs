extern crate usdg;
extern crate env_logger;
extern crate rustc_hex;
extern crate web3;

use web3::contract::{Contract, Options};
use web3::futures::Future;
use web3::types::{Address, U256};

fn main() {
    env_logger::init();
    let (eloop, http) = web3::transports::Http::new("http://localhost:8545").unwrap();
    // run the event loop in the background
    eloop.into_remote();

    let web3 = web3::Web3::new(http);

    let my_account: Address = "d028d24f16a8893bd078259d413372ac01580769".parse().unwrap();

    // Accessing existing contract
    let contract_address: Address = "03bd5552b72FA3725f8A1381ce1252C00CEF899D".parse().unwrap();
    let contract = Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("./contract/USDG.json"),
    )
        .unwrap();
    let transfer_to: Address = "75df5695686338883675bb27bd06fc7578aa01b7".parse().unwrap();
    let result = contract.call("mint", (my_account, U256::from(1000)), my_account, Options::default());
    println!("{}", result.wait().unwrap());
    let result = contract.query("_totalSupply", (), None, Options::default(), None);
    let totalSupply: U256 = result.wait().unwrap();
    println!("{}",totalSupply);
}