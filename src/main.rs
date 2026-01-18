use std::collections::HashMap;
use std::marker::PhantomData;
use std::fmt::Display;
use anyhow::Result;
use game::Common as _;

mod game;
mod tile;

#[tokio::main]
async fn main() -> Result<()> {
    if Box::new(game::Game::default())
        .move_left()?
        .move_left()?
        .move_left()?
        .move_left()?
        .move_up()?
        .move_up()?
        .move_up()?
        .move_up()?
        .move_right()?
        .move_right()?
        .move_right()?
        .move_right()?
        .move_down()?
        .move_left()?
        .move_left()?
        .move_left()?
        .move_left()?
        .move_down()?
        .move_right()?
        .move_right()?
        .move_right()?
        .move_right()?
        .move_down()?
        .move_left()?
        .move_left()?
        .complete()
        .is_some() {
        println!("You have won the game!");
    } else {
        println!("You have not captured the coin yet.");
    }
    Ok(())
}