use chrono::Utc;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// URL to database connection
    #[arg(short, long)]
    url: Option<String>,
}

// TODO Change the return sqlx::Error to a tables::Error, which can be returned gracefully to the
// user.
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let args = Args::parse();

    if let Some(url) = args.url {
        println!("Connection URL: {url}");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await?;

        println!("Connection successful!");

        //        let results = sqlx::query(
        //            r#"
        //INSERT INTO subscriptions (id, email, name, subscribed_at)
        //VALUES ($1, $2, $3, $4)
        //"#,
        //        )
        //        .bind(Uuid::new_v4())
        //        .bind("sylvan@hey.com")
        //        .bind("Sylvan Smit")
        //        .bind(Utc::now())
        //        .execute(&pool)
        //        .await?;
        //
        //        println!("{results:?}");

        let results = sqlx::query("SELECT * FROM subscriptions")
            .fetch_all(&pool)
            .await?;
        println!("{results:?}");
    }

    Ok(())
}
