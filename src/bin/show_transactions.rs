extern crate usdg;
extern crate diesel;

use self::usdg::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use usdg::schema::user_transactions::dsl::*;

    let connection = establish_connection();
    let results = user_transactions.filter(published.eq(true))
        .limit(5)
        .load::<Transaction>(&connection)
        .expect("Error loading transactions");

    println!("Displaying {} transactions", results.len());
    for transaction in results {
        println!("{}", transaction.usdt_wallet);
        println!("----------\n");
        println!("{}", transaction.usdg_wallet);
    }
}