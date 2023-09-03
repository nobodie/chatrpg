mod error;
mod model;

use crate::error::MyError;
use error::GeneralResult;
use model::{Game, NodeId};
use serde::Deserialize;
use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
};
use tokio::fs;

#[derive(Deserialize, Debug)]
pub struct Config {}

pub enum PlayerChoice {
    Quit,
    DiscoverNode(NodeId),
    VisitNode(NodeId),
    TalkTo(String),
}

impl Display for PlayerChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerChoice::Quit => write!(f, "Quit game"),
            PlayerChoice::DiscoverNode(id) => write!(f, "Discover new node {id}"),
            PlayerChoice::VisitNode(id) => write!(f, "Visit previous node {id}"),
            PlayerChoice::TalkTo(name) => write!(f, "Talk to {name}"),
        }
    }
}

pub async fn run() -> GeneralResult<()> {
    let s = fs::read_to_string("Config.toml").await?;
    let _config: Config = toml::from_str(&s)?;

    let mut quit = false;

    let mut game = Game::new().map_err(MyError::ChatRpg)?;

    while !quit {
        let (id, _node) = game.get_current_node();

        println!("You currently are at node {id}");

        let choices = game.generate_choices();

        println!("Available choices :");
        choices
            .iter()
            .enumerate()
            .for_each(|(id, choice)| println!("{id} - {choice}"));

        let choice = choices
            .get(loop_until_valid_input(choices.len()))
            .expect("Choice must be valid.");

        if game.handle_choice(choice).map_err(MyError::ChatRpg)? {
            println!("Sure ? : [0 - no, 1 - yes]");
            if loop_until_valid_input(2) == 1 {
                quit = true;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> GeneralResult<()> {
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
