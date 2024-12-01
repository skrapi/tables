use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;
use tokio::sync::mpsc;

use crate::communication::{DbMessage, TuiMessage};

pub struct Db {
    connection_url: String,
    pool: Option<Pool<Postgres>>,
    db_rx: mpsc::Receiver<DbMessage>,
    tui_tx: mpsc::Sender<TuiMessage>,
    exit: bool,
}

impl Db {
    pub fn new(
        url: String,
        db_rx: mpsc::Receiver<DbMessage>,
        tui_tx: mpsc::Sender<TuiMessage>,
    ) -> Self {
        Self {
            connection_url: url,
            pool: None,
            db_rx,
            tui_tx,
            exit: false,
        }
    }

    pub async fn run(&mut self) -> Result<(), sqlx::Error> {
        self.connect().await?;

        while !self.exit {
            if let (Some(pool), Some(message)) = (&self.pool, self.db_rx.recv().await) {
                match message {
                    DbMessage::Query(query_string) => {
                        // "SELECT * FROM subscriptions"
                        match sqlx::query(&query_string).fetch_all(pool).await {
                            Ok(response) => {
                                let _ = self
                                    .tui_tx
                                    .send(TuiMessage::QueryResponse(
                                        format!("{response:?}").to_string(),
                                    ))
                                    .await;
                            }
                            Err(error) => {
                                let _ = self
                                    .tui_tx
                                    .send(TuiMessage::Failure(error.to_string()))
                                    .await;
                            }
                        }
                    }
                    DbMessage::Quit => self.exit = true,
                }
            }
        }
        Ok(())

        //eprintln!("Connection successful!");

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
        //eprintln!("{results:?}");
    }

    //async fn query(&mut self, query_string: String) -> Result<String, sqlx::Error> {}

    async fn connect(&mut self) -> Result<(), sqlx::Error> {
        self.pool = Some(
            PgPoolOptions::new()
                .max_connections(5)
                .connect(&self.connection_url)
                .await?,
        );
        Ok(())
    }
}
