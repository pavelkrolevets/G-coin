#[derive(Queryable)]
pub struct Transaction {
    pub id: i32,
    pub usdt_wallet: String,
    pub usdg_wallet: String,
    pub price: String,
    pub body: String,
    pub published: bool,
}

use super::schema::user_transactions;

#[derive(Insertable)]
#[table_name="user_transactions"]
pub struct NewTransaction<'a> {
    pub usdt_wallet: &'a str,
    pub usdg_wallet: &'a str,
    pub price: &'a str,
    pub body: &'a str,
}