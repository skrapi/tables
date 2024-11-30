use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;

pub struct Db {
    connection_url: String,
    pool: Option<Pool<Postgres>>,
}

impl Db {
    pub fn new(url: String) -> Self {
        Self {
            connection_url: url,
            pool: None,
        }
    }

    pub async fn run(&mut self) -> Result<(), sqlx::Error> {
        self.connect().await?;
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
        if let Some(pool) = &self.pool {
            let _results = sqlx::query("SELECT * FROM subscriptions")
                .fetch_all(pool)
                .await
                .unwrap();
            //eprintln!("{results:?}");
        }

        Ok(())
    }

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
