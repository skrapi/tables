use clap::Parser;
use tables::communication::{DbMessage, TuiMessage};
use tables::{db::Db, tui::App};
use tokio::{sync::mpsc, task::JoinSet};

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
        let (tui_tx, mut tui_rx) = mpsc::channel::<TuiMessage>(100);
        let (db_tx, mut db_rx) = mpsc::channel::<DbMessage>(100);

        let mut set = JoinSet::new();

        set.spawn(async move {
            let mut app = App::new(tui_rx, db_tx);
            app.run(terminal).await.unwrap()
        });

        set.spawn(async move {
            let mut db = Db::new(url.clone(), db_rx, tui_tx);
            db.run().await.unwrap()
        });

        set.join_all().await;
        ratatui::restore();
    }
}
