use clap::Parser;

use tables::db::Db;
use tables::tui::App;

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

    if let Some(url) = args.url {
        // Set up terminal
        let terminal = ratatui::init();

        let app_result = tokio::spawn(async move {
            let mut app = App::new();
            app.run(terminal).await
        });

        let db_result = tokio::spawn(async move {
            let mut db = Db::new(url.clone());
            db.run().await
        });

        let _ = app_result.await.unwrap();
        let _ = db_result.await.unwrap();

        ratatui::restore();
    }
}
