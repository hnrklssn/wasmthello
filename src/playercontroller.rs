use wasmthello::Game;
pub trait PlayerController<const N: usize> {
    fn make_play(&mut self, game: &Game<N>) -> Result<Pos, Box<dyn Error>>;
}

