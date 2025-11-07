use std::sync::Arc;
use grammers_client::Client;
use grammers_mtsender::SenderPool;
use grammers_session::storages::SqliteSession;
use grammers_tl_types as tl;
use tokio::runtime;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

async fn async_main() -> Result {
    println!("=== Grammers Telegram Client Demo ===\n");

    let session = Arc::new(SqliteSession::open("demo.session")?);
    let pool = SenderPool::new(Arc::clone(&session), 1);
    let client = Client::new(&pool);
    let SenderPool { runner, handle, .. } = pool;
    let pool_task = tokio::spawn(runner.run());

    println!("Testing Telegram connection with ping...");
    match client.invoke(&tl::functions::Ping { ping_id: 12345 }).await {
        Ok(response) => {
            println!("✓ Ping successful!");
            println!("  Response: {:?}\n", response);
        }
        Err(e) => {
            println!("✗ Ping failed: {}\n", e);
        }
    }

    println!("Connection test completed!");
    println!("\nNote: This demo shows basic connectivity.");
    println!("For full functionality (login, messages), you need Telegram API credentials.");

    drop(handle);
    drop(client);
    let _ = pool_task.await;

    Ok(())
}

fn main() -> Result {
    runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}
