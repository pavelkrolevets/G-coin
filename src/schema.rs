table! {
    user_transactions (id) {
        id -> Int4,
        usdt_wallet -> Text,
        usdg_wallet -> Text,
        price -> Text,
        body -> Text,
        published -> Bool,
    }
}
