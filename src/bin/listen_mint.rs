extern crate rustc_hex;
extern crate tokio_core;
extern crate web3;


use web3::contract::{Contract, Options};
use web3::futures::{Future, Stream};
use web3::types::FilterBuilder;
use web3::types::{Address, H256, U256};

fn main() {
    let mut eloop = tokio_core::reactor::Core::new().unwrap();
    let web3 =
        web3::Web3::new(web3::transports::WebSocket::with_event_loop("ws://localhost:8546", &eloop.handle()).unwrap());

    // Get the contract bytecode for instance from Solidity compiler
    let bytecode = include_str!("./contract/SimpleEmit.bin");
    let my_account: Address = "b47f736b9b15dcc888ab790c38a6ad930217cbee".parse().unwrap();
    let transfer_to: Address = "75df5695686338883675bb27bd06fc7578aa01b7".parse().unwrap();

    let contract_address: Address = "84693860dE82C4558911aF1aa26B5791481F9dF7".parse().unwrap();
    let contract = Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("./contract/USDG.json"),
    )
        .unwrap();

    let usdg_contract_address: Address = "F722df5D769426bF0495C5af60e03abaB4c93B38".parse().unwrap();
    let usdg_contract = Contract::from_json(
        web3.eth(),
        usdg_contract_address,
        include_bytes!("./contract/USDG.json"),
    )
        .unwrap();

    // Filter for events in our USDT contract
    let filter = FilterBuilder::default()
        .address(vec![contract.address()])
        .topics(
            None,
            None,
            None,
            None,
        )
        .build();


    eloop
        .run(web3.eth().accounts().then(|accounts| {
            let accounts = accounts.unwrap();
            println!("accounts: {:?}", &accounts);

            web3
                .eth_subscribe()
                .subscribe_logs(filter)
                .then(|sub| {
                    sub.unwrap().for_each(|log| {
                        println!("got log: {:?}", log);
                        let topic_function: H256 = "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef".parse().unwrap();
                        let topic_addr_from: H256 = "000000000000000000000000b47f736b9b15dcc888ab790c38a6ad930217cbee".parse().unwrap();
                        let topic_addr_to: H256 = "000000000000000000000000c29553e4d9b2d1ffde5d89763dcc6bfaa0e006c3".parse().unwrap();

                        // Get transfer event in USDT contract
                        if log.topics[0]==topic_function{
                            println!("Minting some additional tokens...");
                            // convert data from log to integer. This is how much


                            // Read log and mint tokens according to incoming USDT

                            // Check if USDT transfer was to our account
                            if log.topics[2] == topic_addr_to{
                                println!("Receved USDT to the wallet... Minting ne USDG.");
                                let result = usdg_contract.call("mint", (my_account, U256::from(10)), my_account, Options::default());
                                println!("{}", result.wait().unwrap());
//                                let result = usdg_contract.query("_totalSupply", (), None, Options::default(), None);
//                                let total_supply: U256 = result.wait().unwrap();
//                                println!("New total supply of USDG {}",total_supply);
                            }
                        }
                        Ok(())
                    })
                })
                .map_err(|_| ())
        }))
        .unwrap();
}
