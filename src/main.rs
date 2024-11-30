use chrono::Utc;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

use tables::app::App;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// URL to database connection
    #[arg(short, long)]
    url: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Set up terminal
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result.unwrap();

    if let Some(url) = args.url {
        println!("Connection URL: {url}");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await
            .unwrap();

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
            .await
            .unwrap();
        println!("{results:?}");
    }
}
