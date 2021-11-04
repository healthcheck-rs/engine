use std::{
    ops::Sub,
    thread::sleep,
    time::{Duration, Instant},
};

use chrono::{NaiveDate, NaiveDateTime};

#[tokio::main]
async fn main() {
    let sites = vec![
        "https://hermodapp.com",
        "https://api.hermodapp.com/health_check",
    ];

    let client = reqwest::Client::new();
    let pool = sqlx::PgPool::connect("postgres://postgres:password@localhost:5432/healthcheck")
        .await
        .unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();

    loop {
        let loop_begin = Instant::now();
        let mut tx = pool.begin().await.unwrap();
        for site in &sites {
            let begin = chrono::Utc::now();
            let begin2 = chrono::Utc::now().naive_utc();
            let response = client.get(format!("{}", site)).send().await;
            let duration: i32 = chrono::Utc::now()
                .signed_duration_since(begin)
                .num_milliseconds()
                .try_into()
                .unwrap();

            sqlx::query!(
                "INSERT INTO healthchecks (site, status, latency, query_time) VALUES ($1, $2, $3, $4)",
                site,
                response.is_ok(),
                duration,
                begin2
            )
            .execute(&mut tx).await.unwrap();
        }

        tokio::spawn(async move {
            tx.commit().await?;
            Ok::<(), anyhow::Error>(())
        });

        sleep(Duration::from_secs(60).sub(loop_begin.elapsed()));
    }
}
