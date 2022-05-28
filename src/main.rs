#![feature(generic_const_exprs)]
mod terminalplayer;
mod wasmplayer;
mod game;
extern crate wasmtime;
use std::error::Error;
use wasmthello::WasmPlayer;
use crate::terminalplayer::TerminalPlayer;

use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

fn pick_file() -> Option<File> {
    let mut input = String::new();
    println!("enter path to wasm (q to abort):");
    io::stdin().read_line(&mut input).ok()?;
    let ans = input.trim_end_matches('\n');
    if ans == "q" {
        return None;
    }

    match File::open(ans) {
        Ok(file) => Some(file),
        Err(err) => {
            println!("could not open file '{}': {}", ans, err);
            pick_file()
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    const SIZE: usize = 8;

    let f = pick_file();
    if f.is_none() {
        return Ok(());
    }

    let mut reader = BufReader::new(f.unwrap());
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let mut white_player = WasmPlayer::<SIZE>::new(&buffer)?;
    let mut black_player = TerminalPlayer::<SIZE> {};
    let game = wasmthello::play_game::<SIZE>(&mut white_player, &mut black_player);
    println!("Winner: {:?}", game.winner());
    Ok(())
}
