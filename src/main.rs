use chrono::Utc;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// URL to database connection
    #[arg(short, long)]
    url: Option<String>,
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .white()
                .on_blue();
            frame.render_widget(greeting, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}

// TODO Change the return sqlx::Error to a tables::Error, which can be returned gracefully to the
// user.
#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Set up terminal
    let mut terminal = ratatui::init();
    terminal.clear().unwrap();
    let app_result = run(terminal);
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
