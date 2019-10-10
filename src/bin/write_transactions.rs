extern crate usdg;
extern crate diesel;

use self::usdg::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What is usdt_wallet?");
    let mut usdt_wallet = String::new();
    stdin().read_line(&mut usdt_wallet).unwrap();
    let usdt_wallet = &usdt_wallet[..(usdt_wallet.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {} (Press {} when finished)\n", usdt_wallet, EOF);

    println!("What is usdg_wallet?");
    let mut usdg_wallet = String::new();
    stdin().read_line(&mut usdg_wallet).unwrap();
    let usdg_wallet = &usdg_wallet[..(usdg_wallet.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {} (Press {} when finished)\n", usdg_wallet, EOF);

    println!("What is price USDT/USDG?");
    let mut price = String::new();
    stdin().read_line(&mut price).unwrap();
    let price = &price[..(price.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {} (Press {} when finished)\n", price, EOF);

    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let transaction = create_transaction(&connection, &usdt_wallet, &usdg_wallet, &price, &body);
    println!("\nSaved draft with id {}", transaction.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";