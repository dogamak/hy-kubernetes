use crate::config::DatabaseConfig;
use deadpool::managed::{Pool, PoolError};
use deadpool_postgres::Manager;
use refinery::Report;

mod migrations {
    use refinery::embed_migrations;
    embed_migrations!("../migrations");
}

#[derive(Clone)]
pub struct DbConnection {
    pool: Pool<Manager>,
}

impl DbConnection {
    pub async fn connect(config: &DatabaseConfig) -> anyhow::Result<DbConnection> {
        let mut cfg = deadpool_postgres::Config::new();

        cfg.host = Some(config.host.clone());
        cfg.port = Some(config.port);
        cfg.dbname = Some(config.database.clone());
        cfg.user = Some(config.user.clone());
        cfg.password = Some(config.password.clone());

        let pool = cfg.create_pool(
            Some(deadpool_postgres::Runtime::Tokio1),
            tokio_postgres::NoTls,
        )?;

        Ok(DbConnection { pool })
    }

    pub async fn run_migrations(&self) -> anyhow::Result<Report> {
        let mut client = self.get_client().await?;

        let report = migrations::migrations::runner()
            .run_async(&mut **client)
            .await?;

        Ok(report)
    }

    pub async fn get_client(
        &self,
    ) -> Result<deadpool::managed::Object<Manager>, PoolError<tokio_postgres::Error>> {
        self.pool.get().await
    }
}
