extern crate rustc_hex;
extern crate tokio_core;
extern crate web3;

use web3::contract::{Contract, Options};
use web3::futures::{Future, Stream};
use web3::types::FilterBuilder;
use web3::types::{Address};

fn main() {
    let mut eloop = tokio_core::reactor::Core::new().unwrap();
    let web3 =
        web3::Web3::new(web3::transports::WebSocket::with_event_loop("ws://localhost:8546", &eloop.handle()).unwrap());

    // Get the contract bytecode for instance from Solidity compiler
    let bytecode = include_str!("./contract/SimpleEmit.bin");
    let my_account: Address = "b47f736b9b15dcc888ab790c38a6ad930217cbee".parse().unwrap();

    let contract_address: Address = "d95211a71ea61D2E608061A329bA936117a6f243".parse().unwrap();
    let contract = Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("./contract/receive_usdt.json"),
    )
        .unwrap();

    // Filter for Hello event in our contract
    let filter = FilterBuilder::default()
        .address(vec![contract.address()])
        .topics(
            Some(vec![
                "d282f389399565f3671145f5916e51652b60eee8e5c759293a2f5771b8ddfd2e"
                    .parse()
                    .unwrap(),
            ]),
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
                        Ok(())
                    })
                })
                .map_err(|_| ())
        }))
        .unwrap();
}
