#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{Transaction, NewTransaction};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_transaction<'a>(conn: &PgConnection, usdt_wallet: &'a str, usdg_wallet: &'a str, price: &'a str, body: &'a str,) -> Transaction {
    use schema::user_transactions;

    let new_transaction = NewTransaction {
        usdt_wallet: usdt_wallet,
        usdg_wallet: usdg_wallet,
        price: price,
        body: body,
    };

    diesel::insert_into(user_transactions::table)
        .values(&new_transaction)
        .get_result(conn)
        .expect("Error saving new transaction")
}