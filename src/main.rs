use tokio::prelude::*;
use tokio::io::AsyncBufReadExt;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut args = std::env::args();
    let _me = args.next();
    let mut tasks = vec![];

    for filename in args {
        tasks.push(tokio::spawn(async {
            let file = tokio::fs::File::open(filename).await?;
            let file = io::BufReader::new(file);
            let mut lines = file.lines();
            let mut count = 0u32;
            while let Some(_) = lines.next_line().await? {
                count += 1;
            }
            Ok(count) as Result<u32, std::io::Error>
        }));
    }

    let mut count = 0u32;
    for task in tasks {
        count += task.await??;
    }
    println!("Total lines: {}", count);
    Ok(())
}
