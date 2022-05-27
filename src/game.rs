use std::error::Error;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    White,
    Black
}

impl Player {
    fn flip(&self) -> Player {
        match *self {
            Player::White => Player::Black,
            Player::Black => Player::White
        }
    }
    pub fn serialize(&self) -> u8 {
        match *self {
            Player::White => 1,
            Player::Black => 2
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pos(usize,usize);

impl Pos {
    pub fn to_offset(&self, board_size: usize) -> u8 {
        (self.0 + self.1 * board_size) as u8
    }

    pub fn from_offset(offset: u8, board_size: usize) -> Self {
        Pos(offset as usize % board_size, offset as usize / board_size)
    }
}

pub trait PlayerController<const N: usize> {
    fn make_play(&mut self, game: &Game<N>) -> Result<Pos, Box<dyn Error>>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

impl Dir {
    fn delta_pos(&self) -> (isize, isize) {
        match *self {
            Dir::Up    => (0, -1),
            Dir::Down  => (0,  1),
            Dir::Left  => (-1, 0),
            Dir::Right => (1,  0),
        }
    }

    fn add_to_pos(&self, pos: Pos) -> Pos {
        let delta_pos = self.delta_pos();
        let x = (pos.0 as isize + delta_pos.0) as usize;
        let y = (pos.1 as isize + delta_pos.1) as usize;
        Pos(x, y)
    }
}

pub struct Game<const N: usize> {
    board: [[Option<Player>; N]; N],
    turn: Player,
}

impl <const N: usize> Game<N> {
    pub fn new() -> Self {
        assert!(N % 2 == 0);
        let mut g = Self {
            board: [[None; N]; N],
            turn: Player::Black,
        };
        g.board[N / 2 - 1][N / 2 - 1] = Some(Player::White);
        g.board[N / 2][N / 2 - 1] = Some(Player::Black);
        g.board[N / 2 - 1][N / 2] = Some(Player::Black);
        g.board[N / 2][N / 2] = Some(Player::White);
        g
    }

    pub fn current_player(&self) -> Player {
        self.turn
    }

    pub fn print(&self) {
        for i in 0..N {
            for j in 0..N {
                let pos = Pos(j,i);
                match self.space(pos) {
                    Some(Player::White) => print!("1"),
                    Some(Player::Black) => print!("2"),
                    None => if self.is_legal_move(pos, self.turn) {
                        print!("+")
                    } else {
                        print!("-")
                    }
                }
            }
            println!("");
        }
    }

    pub fn play(&mut self, pos: Pos) {
        assert!(self.legal_moves(self.turn).contains(&pos));
        assert!(self.is_space(pos));
        assert!(self.board[pos.0][pos.1].is_none());
        for flip_pos in self.flipped_if_placed(pos, self.turn) {
            self.set(flip_pos, self.turn);
        }
        self.set(pos, self.turn);
        self.turn = self.turn.flip();
    }

    pub fn serialize(&self, buf: &mut [u8]) {
        let values = self.positions().into_iter().map(|pos| match self.space(pos) {
            None => 0,
            Some(Player::White) => 1,
            Some(Player::Black) => 2
        });
        for (i,v) in values.enumerate() {
            assert!(i < N * N);
            buf[i] = v;
        }
    }

    pub fn skip(&mut self) {
        assert!(self.legal_moves(self.turn).is_empty());
        self.turn = self.turn.flip();
    }

    pub fn game_over(&self) -> bool {
        self.legal_moves(self.turn).is_empty() &&
        self.legal_moves(self.turn.flip()).is_empty()
    }

    pub fn winner(&self) -> Option<Player> {
        assert!(self.game_over());
        let non_empty_spaces = self.positions().into_iter().filter_map(|pos| self.space(pos));
        let (white_spaces, black_spaces): (Vec<Player>, Vec<Player>) = non_empty_spaces.partition(|&space| space == Player::White);
        if white_spaces.len() == black_spaces.len() {
            None
        } else if white_spaces.len() > black_spaces.len() {
            Some(Player::White)
        } else {
            Some(Player::Black)
        }
    }

    fn positions(&self) -> Vec<Pos> {
        (0..N).flat_map(|x| (0..N).map(move |y| Pos(x,y))).collect()
    }


    pub fn legal_moves(&self, player: Player) -> Vec<Pos> {
        self.positions().into_iter()
            .filter(|&pos| self.is_legal_move(pos, player))
            .collect()
    }

    fn is_legal_move(&self, pos: Pos, player: Player) -> bool {
        !self.flipped_if_placed(pos, player).is_empty()
    }

    fn flipped_if_placed(&self, pos: Pos, player: Player) -> Vec<Pos> {
        [self.flipped_if_placed_dir(pos, Dir::Up, player),
         self.flipped_if_placed_dir(pos, Dir::Down, player),
         self.flipped_if_placed_dir(pos, Dir::Left, player),
         self.flipped_if_placed_dir(pos, Dir::Right, player)].concat()
    }

    fn flipped_if_placed_dir(&self, pos: Pos, dir: Dir, player: Player) -> Vec<Pos> {
        if self.space(pos).is_some() {
            return Vec::new()
        }
        let positions_in_dir: Vec<Pos> = match dir {
            Dir::Up => (0..pos.1).rev().map(|y| Pos(pos.0,y)).collect(),
            Dir::Down => ((pos.1 + 1)..N).map(|y| Pos(pos.0,y)).collect(),
            Dir::Left => (0..pos.0).rev().map(|x| Pos(x, pos.1)).collect(),
            Dir::Right => ((pos.0 + 1)..N).map(|x| Pos(x, pos.1)).collect()
        };
        let res: Vec<Pos> = positions_in_dir.into_iter().take_while(|&p| match self.space(p) {
            Some(s) => s == player.flip(),
            _ => false
        }).collect();
        match res.last() {
            None => Vec::new(),
            Some(&last_pos) => {
                let next_pos = dir.add_to_pos(last_pos);
                // Check that the line ends with tile of player colour
                if self.is_space(next_pos) && self.space(next_pos).map_or_else(|| false, |p| p == player) {
                    res
                } else {
                    Vec::new()
                }
            }
        }
    }

    fn is_space(&self, pos: Pos) -> bool {
        pos.0 < N && pos.1 < N
    }

    fn space(&self, pos: Pos) -> Option<Player> {
        assert!(self.is_space(pos));
        self.board[pos.0][pos.1]
    }

    fn set(&mut self, pos: Pos, player: Player) {
        assert!(self.is_space(pos));
        self.board[pos.0][pos.1] = Some(player);
    }
}
