use sqlx::MySqlPool;

pub async fn init_db(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    let pool = MySqlPool::connect(database_url).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS item (
            id INT AUTO_INCREMENT PRIMARY KEY,
            name VARCHAR(255) NOT NULL 
        )
        "#,
        
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}