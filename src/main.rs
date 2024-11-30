use clap::Parser;

use tables::db::Db;
use tables::tui::App;
use tokio::task::JoinSet;

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

        let mut set = JoinSet::new();

        set.spawn(async move {
            let mut app = App::new();
            app.run(terminal).await.unwrap()
        });

        set.spawn(async move {
            let mut db = Db::new(url.clone());
            db.run().await.unwrap()
        });

        while let Some(_) = set.join_next().await {}
        ratatui::restore();
    }
}
