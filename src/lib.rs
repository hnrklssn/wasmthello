#![feature(generic_const_exprs)]
mod wasmplayer;
mod game;
pub use crate::wasmplayer::WasmPlayer;
pub use crate::game::{Game, Pos, Player, PlayerController};

use std::error::Error;

pub fn play_game<const N: usize>(white_player: &mut dyn PlayerController<N>, black_player: &mut dyn PlayerController<N>) -> Result<Game<N>, Box<dyn Error>> {
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
