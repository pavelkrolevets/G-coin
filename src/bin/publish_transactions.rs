extern crate usdg;
extern crate diesel;

use self::diesel::prelude::*;
use self::usdg::*;
use self::models::Transaction;
use std::env::args;

fn main() {
    use usdg::schema::user_transactions::dsl::{user_transactions, published};

    let id = args().nth(1).expect("publish_transaction requires a post id")
        .parse::<i32>().expect("Invalid ID");
    let connection = establish_connection();

    let transaction = diesel::update(user_transactions.find(id))
        .set(published.eq(true))
        .get_result::<Transaction>(&connection)
        .expect(&format!("Unable to find post {}", id));
    println!("Published post {}", transaction.usdt_wallet);
}