use std::{env, process, time::Duration};

use arboard::Clipboard;
use clap::Parser;
use colored::Colorize;
use crossterm::{
    cursor::MoveToColumn,
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use openai::Message;

mod openai;

/// Program that takes help message and creates markdown table with help of AI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Command that returns help message
    command: String,

    /// Model to use
    #[arg(short, long, value_enum, default_value = "gpt35-turbo")]
    model: openai::Model,

    /// Print only the table output
    #[arg(short, long, default_value = "false")]
    pipable: bool,

    /// Put the table output in clipboard
    #[arg(short, long, default_value = "false")]
    clipboard: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let Ok(api_key) = env::var("OPENAI_API_KEY") else {
        println!("{} {}", "OPENAI_API_KEY not set.".red(), "Refer to step 3 here: https://help.openai.com/en/articles/5112595-best-practices-for-api-key-safety".bright_black());
        process::exit(1)
    };

    let command = args.command.clone();
    let loading_help_animation = tokio::spawn(async move {
        if args.pipable {
            return;
        }
        let emoji_support =
            terminal_supports_emoji::supports_emoji(terminal_supports_emoji::Stream::Stdout);
        let frames = if emoji_support {
            vec![
                "🕛", "🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚",
            ]
        } else {
            vec!["/", "-", "\\", "|"]
        };
        let mut current_frame = 0;
        let mut stdout = std::io::stdout();
        loop {
            current_frame = (current_frame + 1) % frames.len();
            match execute!(
                stdout,
                Clear(ClearType::CurrentLine),
                MoveToColumn(0),
                SetForegroundColor(Color::Yellow),
                Print(frames[current_frame]),
                Print(" Executing command '".bright_black()),
                Print(command.purple()),
                Print("'".bright_black()),
                ResetColor
            ) {
                Ok(_) => {}
                Err(_) => {
                    break;
                }
            }
            tokio::time::sleep(Duration::from_millis(150)).await;
        }
    });

    let command_parts = args.command.split(" ").collect::<Vec<&str>>();
    let output_raw = std::process::Command::new(command_parts[0])
        .args(&command_parts[1..])
        .output()?
        .stdout;

    if !loading_help_animation.is_finished() {
        loading_help_animation.abort();
        execute!(
            std::io::stdout(),
            Clear(ClearType::CurrentLine),
            MoveToColumn(0),
        )?;
    }

    let output = String::from_utf8(output_raw)?;

    let messages = vec![
        Message::system(String::from(openai::SYSTEM_MSG)),
        Message::user(output),
    ];

    let req = openai::Request::new(args.model.to_string(), messages);

    let json = match serde_json::to_string(&req) {
        Ok(json) => json,
        Err(e) => {
            println!("{e}");
            process::exit(1);
        }
    };

    let loading_ai_animation = tokio::spawn(async move {
        if args.pipable {
            return;
        }
        let emoji_support =
            terminal_supports_emoji::supports_emoji(terminal_supports_emoji::Stream::Stdout);
        let frames = if emoji_support {
            vec![
                "🕛", "🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚",
            ]
        } else {
            vec!["/", "-", "\\", "|"]
        };
        let mut current_frame = 0;
        let mut stdout = std::io::stdout();
        loop {
            current_frame = (current_frame + 1) % frames.len();
            match execute!(
                stdout,
                Clear(ClearType::CurrentLine),
                MoveToColumn(0),
                SetForegroundColor(Color::Yellow),
                Print(frames[current_frame]),
                Print(" Asking AI".bright_black()),
                ResetColor
            ) {
                Ok(_) => {}
                Err(_) => {
                    break;
                }
            }
            tokio::time::sleep(Duration::from_millis(150)).await;
        }
    });

    let request_builder = reqwest::Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .bearer_auth(api_key)
        .body(json);

    let response = match request_builder.send().await {
        Ok(response) => {
            let body = response.text().await?;
            let response: openai::Response = serde_json::from_str(&body)?;
            response
        }
        Err(e) => {
            println!("{e}");
            process::exit(1);
        }
    };

    if !loading_ai_animation.is_finished() {
        loading_ai_animation.abort();
        execute!(
            std::io::stdout(),
            Clear(ClearType::CurrentLine),
            MoveToColumn(0),
        )?;
    }

    println!("{}", response.choices[0].message.content);

    if args.clipboard {
        if let Ok(mut clip) = Clipboard::new() {
            clip.set_text(&response.choices[0].message.content).unwrap();
        }
    }

    Ok(())
}
