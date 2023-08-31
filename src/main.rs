mod model;

use std::{
    fmt::Display,
    io::{self, stdin, stdout, Write},
};

use serde::Deserialize;
use thiserror::Error;
use tokio::fs;
use toml::de;

#[derive(Deserialize, Debug)]
pub struct Config {}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("General error ")]
    GeneralError,
    #[error("Io error {0}")]
    IoError(#[from] io::Error),
    #[error("Toml error {0}")]
    TomlError(#[from] de::Error),

    #[error("Serde error {0}")]
    SerdeError(#[from] serde_json::Error),
}

pub type MyResult<T> = Result<T, MyError>;

pub enum Choice {
    Quit,
}

impl Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Choice::Quit => write!(f, "Quit"),
        }
    }
}

pub async fn run() -> MyResult<()> {
    let s = fs::read_to_string("Config.toml").await?;
    let _config: Config = toml::from_str(&s)?;

    let mut quit = false;

    let choices = vec![Choice::Quit];

    while !quit {
        println!("Available choices :");
        for (id, choice) in choices.iter().enumerate() {
            println!("{id} - {choice}");
        }

        if let Some(Choice::Quit) = choices.get(loop_until_valid_input(choices.len())) {
            quit = true;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> MyResult<()> {
    run().await
}

fn loop_until_valid_input(len: usize) -> usize {
    let mut s = String::new();

    loop {
        s.clear();
        print!("Your choice [0-{}]: ", len - 1);

        let _ = stdout().flush();
        match stdin().read_line(&mut s) {
            Ok(_) => match s.trim().parse::<usize>() {
                Ok(num) => {
                    if num < len {
                        return num;
                    }
                }
                Err(_) => {
                    continue;
                }
            },
            Err(_) => continue,
        };
    }
}
