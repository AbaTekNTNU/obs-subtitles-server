use reqwest::Error;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::iter::Iterator;

fn load_txt_file_safe(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

async fn send_post_request(url: &str, string: &str) -> Result<(), Error> {
    let body = format!(r#"{{"text": "{}"}}"#, string);
    let response = reqwest::Client::new()
        .post(url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;
    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    // let user choose which file in /subtitles to send
    let mut files = Vec::new();
    for entry in std::fs::read_dir("subtitles").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        files.push(path);
    }
    println!("Choose a subtitle or press q to quit:");
    for (i, file) in files.iter().enumerate() {
        println!("{}: {}", i, file.display());
    }
    let txtFile: String;
    let mut input = String::new();
    match io::stdin().lock().read_line(&mut input) {
        Ok(_) => {
            if input.trim() == "q" {
                return Ok(());
            }
            let index = input.trim().parse::<usize>().unwrap();
            txtFile = files[index].to_str().unwrap().to_string();
            println!("You chose: {}", txtFile);
            let lines = match load_txt_file_safe(&txtFile) {
                Ok(lines) => lines,
                Err(error) => panic!("Error: {}", error),
            };
            let url = std::env::args().nth(2).unwrap();

            let stdin = io::stdin();
            let mut stdin_lock = stdin.lock();
            print!("Press space to send the next line.");
            for (i, line) in lines.lines().enumerate() {
                println!("Next line: {}: {}", i, line);
                loop {
                    let mut buf = [0; 1];
                    stdin_lock.read_exact(&mut buf).unwrap();
                    if buf[0] == b' ' {
                        send_post_request(&url, line).await?;
                        println!("Current: {}\n", line);
                        break;
                    } else {
                        return Ok(());
                    }
                }
            }
        }
        Err(error) => println!("error: {}", error),
    }
    Ok(())
}
