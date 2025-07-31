use futures::future::join_all;
use tokio::fs;

pub async fn multi_file(){

      let files = vec![
        "./file/file1.txt",
        "./file/file2.txt",
        "./file/file3.txt",
    ];

    let read_futures = files.into_iter().map(|path| {
        async move {
            match fs::read_to_string(path).await {
                Ok(content) => println!("{}",content),
                Err(e) => {
                    eprintln!("Failed to read file: {}", e);
                    
                }

            }
        }
    });

    let results  = join_all(read_futures).await;
    println!("end of async code")
}