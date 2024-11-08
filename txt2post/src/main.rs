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

async fn send_post_request(string: &str) -> Result<(), Error> {
    let body = format!(r#"{{"text": "{}"}}"#, string);
    match reqwest::Client::new()
        .post("http://localhost:3000")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
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
    disable_send: bool,
) {
    write!(stdout, "{}", termion::clear::All);

    write!(stdout, "{}", termion::cursor::Goto(1, 17));
    if (disable_send) {
        write!(stdout, "{}Output disabled", termion::color::Fg(termion::color::Red));
    } else {
        write!(stdout, "{}Output enabled", termion::color::Fg(termion::color::Green));
    }

    if line_number > 1 {
        match strings.get(line_number - 2) {
            Some(line) => {
                write!(stdout, "{}", termion::cursor::Goto(1, 1));
                write!(stdout, "{}", termion::color::Fg(termion::color::LightBlack));
                write!(stdout, "Previous:");
                print_line_left_aligned(stdout, line, 1);
            }
            None => {
                write!(stdout, "No previous line");
            }
        }
    }

    if line_number > 0 {
        match strings.get(line_number - 1) {
            Some(line) => {
                write!(stdout, "{}", termion::cursor::Goto(1, 2));
                write!(stdout, "{}", termion::color::Fg(termion::color::Blue));
                write!(stdout, "Current:");
                print_line_left_aligned(stdout, line, 2);
            }
            None => {
                write!(stdout, "No previous line\n");
            }
        }
    }

    write!(stdout, "{}", termion::color::Fg(termion::color::Reset));
    write!(stdout, "{}", termion::cursor::Goto(1, 3));
    write!(stdout, "Next:");

    if line_number < strings.len() {
        match strings.get(line_number) {
            Some(line) => {
                print_line_left_aligned(stdout, line, 3);
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
    for i in 0..10 {
        match strings.get(line_number + i) {
            Some(line) => print_line_left_aligned(stdout, line, (i + 3).try_into().unwrap()),
            None => {
                break;
            }
        }
    }
    write!(stdout, "{}", termion::cursor::Goto(1, 10));
    write!(
        stdout,
        "Press space to send next line, m to go to menu, b to toggle sending or q to quit"
    );
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
    for entry in std::fs::read_dir("./subs").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        files.push(path);
        files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    }

    write!(stdout(), "{}", termion::clear::All);
    for (i, file) in files.iter().enumerate() {
        write!(
            stdout(),
            "{}",
            termion::cursor::Goto(1, (i).try_into().unwrap()),
        )
        .unwrap();
        println!("{}: {}", i, file.display());
    }
    //write!(stdout(), "{}", termion::cursor::Goto(1, 15)).unwrap();
    write!(stdout(), "Choose a subtitle or press q to quit\n").unwrap();

    let strings = choose_file(files);
    strings
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut strings = read_subtitles();
    let mut line_number = 0;
    let stdout = stdout();
    let mut stdout = stdout.into_raw_mode().unwrap();
    let mut disable_send = false;

    write_lines(&mut stdout, &strings, line_number, disable_send);

    loop {
        let stdin = std::io::stdin();
        for c in stdin.events() {
            match c.unwrap() {
                Event::Key(Key::Char('q')) => {
                    write!(stdout, "{}", termion::clear::All).unwrap();
                    write!(stdout, "{}", termion::cursor::Show).unwrap();
                    return Ok(());
                }
                Event::Key(Key::Char(' ')) => {
                    if line_number < strings.len() {
                        match strings.get(line_number) {
                            Some(line) => {
                                line_number += 1;
                                write!(stdout, "{}", termion::color::Fg(termion::color::White))
                                    .unwrap();
                                write_lines(&mut stdout, &strings, line_number, disable_send);
                                if !disable_send {
                                    match send_post_request(line).await {
                                        Ok(_) => {}
                                        Err(e) => {
                                            write!(
                                                stdout,
                                                "{}",
                                                termion::color::Fg(termion::color::Red)
                                            )
                                            .unwrap();
                                            write!(stdout, "{}", termion::cursor::Goto(1, 20))
                                                .unwrap();
                                            write!(stdout, "Could not send request: {}", e)
                                                .unwrap();
                                        }
                                    }
                                }
                            }
                            None => {
                                write!(stdout, "{}", termion::cursor::Goto(1, 2)).unwrap();
                                write!(stdout, "No more lines").unwrap();
                            }
                        }
                    } else {
                        match send_post_request("").await {
                            Ok(_) => {
                                write!(stdout, "{}", termion::clear::All).unwrap();
                                write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
                                write!(stdout, "Sent empty line").unwrap();
                            }
                            Err(e) => {
                                write!(stdout, "{}", termion::cursor::Goto(1, 20)).unwrap();
                                write!(stdout, "Could not send request: {}", e).unwrap();
                            }
                        }
                        write!(stdout, "No more lines to send").unwrap();
                    }
                }
                // on m, go to menu
                Event::Key(Key::Char('m')) => {
                    write!(stdout, "{}", termion::clear::All).unwrap();
                    write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
                    write!(stdout, "Going back to menu").unwrap();
                    match send_post_request("").await {
                        Ok(_) => {
                            write!(stdout, "{}", termion::clear::All).unwrap();
                            write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
                            write!(stdout, "Sent empty line").unwrap();
                        }
                        Err(e) => {
                            write!(stdout, "{}", termion::cursor::Goto(1, 20)).unwrap();
                            write!(stdout, "Could not send request: {}", e).unwrap();
                        }
                    }
                    write!(stdout, "{}", termion::cursor::Show).unwrap();
                    stdout.suspend_raw_mode().unwrap();
                    strings = read_subtitles();
                    stdout.activate_raw_mode().unwrap();

                    line_number = 0;
                    write_lines(&mut stdout, &strings, line_number, disable_send);
                    match strings.get(line_number) {
                        Some(line) => {}
                        None => {
                            write!(stdout, "{}", termion::cursor::Goto(1, 2)).unwrap();
                            write!(stdout, "No more lines").unwrap();
                        }
                    }
                }
                Event::Key(Key::Char('b')) => {
                    if line_number > 1 {
                        match strings.get(line_number) {
                            Some(line) => {
                                line_number -= 1;
                                write_lines(&mut stdout, &strings, line_number, disable_send);
                                if !disable_send {
                                    match send_post_request(strings.get(line_number - 1).unwrap())
                                    .await
                                    {
                                        Ok(_) => {}
                                        Err(e) => {
                                            write!(stdout, "{}", termion::cursor::Goto(1, 20))
                                                .unwrap();
                                            write!(stdout, "Could not send request: {}", e)
                                                .unwrap();
                                        }
                                    }
                                }
                            }
                            None => {
                                write!(stdout, "{}", termion::cursor::Goto(1, 2)).unwrap();
                                write!(stdout, "No more lines").unwrap();
                            }
                        }
                    }
                }
                Event::Key(Key::Char('c')) => {
                    match send_post_request("").await {
                        Ok(_) => {
                            write!(stdout, "{}", termion::cursor::Goto(1, 20)).unwrap();
                            write!(stdout, "Cleared screen").unwrap();
                        }
                        Err(e) => {
                            write!(stdout, "{}", termion::cursor::Goto(1, 20)).unwrap();
                            write!(stdout, "Could not send request: {}", e).unwrap();
                        }
                    }
                    write!(stdout, "{}", termion::clear::All).unwrap();
                    write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
                    write!(stdout, "Press q to quit, space to continue").unwrap();
                }
                Event::Key(Key::Char('v')) => {
                    disable_send = !disable_send;
                    write!(stdout, "{}", termion::cursor::Goto(1, 20)).unwrap();
                    if disable_send {
                        write!(stdout, "Disabled sending").unwrap();
                        send_post_request("").await.unwrap();
                    } else {
                        write!(stdout, "Enabled sending").unwrap();
                        send_post_request(strings.get(line_number - 1).unwrap())
                            .await
                            .unwrap();
                    }
                    write_lines(&mut stdout, &strings, line_number, disable_send);
                }
                _ => {}
            }
        }
    }
    Ok(())
}
