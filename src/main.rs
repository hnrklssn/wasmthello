#![feature(generic_const_exprs)]
mod wasmplayer;
mod terminalplayer;
extern crate wasmtime;
use std::error::Error;
use wasmthello::Game;
use wasmthello::Player;
use wasmthello::Pos;
use crate::wasmplayer::WasmPlayer;
use crate::terminalplayer::TerminalPlayer;

use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

trait PlayerController<const N: usize> {
    fn make_play(&mut self, game: &Game<N>) -> Result<Pos, Box<dyn Error>>;
}

fn pick_file() -> Option<File> {
    let mut input = String::new();
    println!("enter path to wasm (q to abort):");
    io::stdin().read_line(&mut input).ok()?;
    let ans = input.trim_end_matches("\n");
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

fn play_game<const N: usize>(white_player: &mut dyn PlayerController<N>, black_player: &mut dyn PlayerController<N>) -> Result<Game<N>, Box<dyn Error>> {
    let mut game = Game::<N>::new();
    while !game.game_over() {
        let legal_moves = game.legal_moves(game.current_player());
        if legal_moves.is_empty() {
            game.skip();
            // Game is over if neither can play
            assert!(!game.legal_moves(game.current_player()).is_empty());
        }
        let pos = if game.current_player() == Player::White {
            white_player.make_play(&game)?
        } else {
            black_player.make_play(&game)?
        };
        println!("Answer: {:?}", pos);
        game.play(pos);
    }
    Ok(game)
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
    let game = play_game::<SIZE>(&mut white_player, &mut black_player)?;
    println!("Winner: {:?}", game.winner());
    Ok(())
}
