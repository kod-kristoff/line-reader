use tokio::prelude::*;
use tokio::io::AsyncBufReadExt;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let stdin = io::stdin();
    let stdin = io::BufReader::new(stdin);
    let mut count = 0u32;
    let mut lines = stdin.lines();
    while let Some(_) = lines.next_line().await? {
        count += 1;
    }
    println!("Lines on stdin: {}", count);
    Ok(())
}
