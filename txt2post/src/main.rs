use reqwest::Error;
use std::fmt::write;
use std::fs::read_dir;
use std::fs::File;
use std::io::{stdout, BufRead, Read, Write};
use std::iter::Iterator;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

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
fn choose_file(files: Vec<std::path::PathBuf>) -> Vec<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    if input == "q" {
        std::process::exit(0);
    }
    let input = input.parse::<usize>().unwrap();
    let file = files[input].to_str().unwrap();
    match load_txt_file_safe(file) {
        Ok(string) => {
            let lines = string.lines();
            let mut lines_vec = Vec::new();
            for line in lines {
                lines_vec.push(line.to_string());
            }
            println!("You chose: {}", file);
            lines_vec
        }
        Err(e) => {
            println!("Could not load file: {}", e);
            std::process::exit(1);
        }
    }
}

fn write_lines(
    stdout: &mut RawTerminal<std::io::Stdout>,
    strings: &Vec<String>,
    line_number: usize,
) {
    write!(stdout, "{}", termion::clear::All);

    if line_number > 0 {
        match strings.get(line_number - 1) {
            Some(line) => {
                write!(stdout, "Previous line:");
                print_line_left_aligned(stdout, line, 1);
            }
            None => {
                write!(stdout, "No previous line\n");
            }
        }
    }

    write!(stdout, "{}", termion::cursor::Goto(1, 2));
    write!(stdout, "Sent:");

    if line_number < strings.len() {
        match strings.get(line_number) {
            Some(line) => {
                print_line_left_aligned(stdout, line, 2);
            }
            None => {
                write!(stdout, "No more lines\n");
            }
        }
    }
    write!(stdout, "{}", termion::cursor::Goto(1, 4));
    write!(stdout, "Next lines:");
    //move cursor to next line, 0 chars from left
    //show next 5 lines aligned to left side
    for i in 1..5 {
        match strings.get(line_number + i) {
            Some(line) => print_line_left_aligned(stdout, line, (i + 3).try_into().unwrap()),
            None => {
                break;
            }
        }
    }
    write!(stdout, "{}", termion::cursor::Goto(1, 10));
    write!(stdout, "Press space to send next line or q to quit");
}

fn print_line_left_aligned(
    stdout: &mut RawTerminal<std::io::Stdout>,
    line: &String,
    line_number: u16,
) {
    let OFFSET = 20;
    write!(stdout, "{}", termion::cursor::Goto(OFFSET, line_number));
    write!(stdout, "{}\n", line);
}

fn read_subtitles() -> Vec<String> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir("subtitles").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        files.push(path);
    }

    write!(stdout(), "{}", termion::clear::All);
    for (i, file) in files.iter().enumerate() {
        write!(
            stdout(),
            "{}",
            termion::cursor::Goto(1, (i + 1).try_into().unwrap()),
        )
        .unwrap();
        println!("{}: {}", i, file.display());
    }
    //Choose a subtitle or press q to quit
    write!(stdout(), "{}", termion::cursor::Goto(1, 10)).unwrap();
    write!(stdout(), "Choose a subtitle or press q to quit").unwrap();

    let strings = choose_file(files);
    strings
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let strings = read_subtitles();

    let stdout = stdout();
    let mut stdout = stdout.into_raw_mode().unwrap();

    write!(stdout, "{}", termion::cursor::Hide);
    let mut line_number = 0;
    write_lines(&mut stdout, &strings, line_number);
    send_post_request("http://localhost:3000", strings.get(line_number).unwrap()).await?;

    loop {
        write!(stdout, "{}", termion::cursor::Goto(1, 1));
        if line_number + 1 == strings.len() {
            let strings = read_subtitles();
            write!(stdout, "{}", termion::cursor::Hide);
            line_number = 0;
            write_lines(&mut stdout, &strings, line_number);
            send_post_request("http://localhost:3000", strings.get(line_number).unwrap()).await?;
        }

        match std::io::stdin().events().next().unwrap() {
            Ok(Event::Key(Key::Char(' '))) => {
                line_number += 1;
                let string = strings.get(line_number).unwrap();
                send_post_request("http://localhost:3000", string).await?;
                write_lines(&mut stdout, &strings, line_number);
            }
            Ok(Event::Key(Key::Left)) => {
                if line_number > 0 {
                    line_number -= 1;
                } else {
                    line_number = 0;
                }
                let string = strings.get(line_number).unwrap();
                send_post_request("http://localhost:3000", string).await?;
                write_lines(&mut stdout, &strings, line_number);
            }
            Ok(Event::Key(Key::Char('q'))) => {
                write!(stdout, "{}", termion::cursor::Show);
                //disable_raw_mode().unwrap();
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
