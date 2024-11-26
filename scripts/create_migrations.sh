# Assuming you used the default parameters from the script
export DATABASE_URL=postgres://app:secret@127.0.0.1:5432/newsletter
sqlx migrate add create_subscriptions_table
