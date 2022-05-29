#![feature(generic_const_exprs)]
#[cfg(not(target_arch = "wasm32"))]
mod wasmplayer;
mod game;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::wasmplayer::WasmPlayer;
pub use crate::game::{Game, Pos, Player, PlayerController};

#[cfg(not(target_arch = "wasm32"))]
pub fn play_game<const N: usize>(white_player: &mut dyn PlayerController<N>, black_player: &mut dyn PlayerController<N>) -> Game<N> {
    let mut game = Game::<N>::new();
    while !game.game_over() {
        let legal_moves = game.legal_moves(game.current_player());
        if legal_moves.is_empty() {
            game.skip();
            // Game is over if neither can play
            assert!(!game.legal_moves(game.current_player()).is_empty());
        }
        let play = if game.current_player() == Player::White {
            white_player.make_play(&game).map_err(|_| Player::White)
        } else {
            black_player.make_play(&game).map_err(|_| Player::Black)
        };
        match play {
            Ok(pos) => {
                println!("Answer: {:?}", pos);
                game.play(pos);
            },
            Err(player) => game.misplay(player)
        };
    }
    game
}
