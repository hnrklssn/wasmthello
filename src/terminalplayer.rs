use std::error::Error;
use wasmthello::PlayerController;
use wasmthello::Pos;
use wasmthello::Game;
use wasmthello::Player;
use std::io;

pub struct TerminalPlayer<const N: usize> {
}

impl<const N: usize> PlayerController<N> for TerminalPlayer<N> {

    fn make_play(&mut self, game: &Game<N>) -> Result<Pos, Box<dyn Error>> {
        if game.current_player() == Player::White {
            println!("White (1) player's turn");
        } else {
            println!("Black (2) player's turn");
        }
        game.print();

        let legal_moves = game.legal_moves(game.current_player());
        let legal_move_count = legal_moves.len();
        assert!(legal_move_count > 0);
        for pos in legal_moves.into_iter() {
            let offset = pos.to_offset(N);
            println!("legal move {}: {:?}", offset, pos);
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let ans: i32 = input.trim().parse()?;
        let pos = Pos::from_offset(ans as u8, N);
        println!("placing {:?}", pos);
        Ok(pos)
    }
}
