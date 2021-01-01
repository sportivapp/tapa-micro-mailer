#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod config;
mod messages;
mod utils;

use std::io::Result as IOResult;

#[tokio::main]
async fn main() -> IOResult<()> {
    println!("Hello, world!");

    Ok(())
}
