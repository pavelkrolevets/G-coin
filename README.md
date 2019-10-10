# Memo

1. Create a databse
``` 
echo DATABASE_URL=postgres://username:password@localhost/usdg > .env
diesel setup
diesel migration generate user_transactions   
diesel migration run
```

2. Show transactions

``` 
cargo run --bin show_transactions
```