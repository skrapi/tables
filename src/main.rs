use clap::Parser;
use sqlx::postgres::PgPoolOptions;

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

        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&pool)
            .await?;

        assert_eq!(row.0, 150);
    }
    Ok(())
}
