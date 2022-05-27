use std::error::Error;
use crate::game::Game;
use crate::game::Pos;
use crate::game::PlayerController;
use wasmtime::*;

pub struct WasmPlayer<const N: usize> where [(); N*N]: Sized {
    store: Store<()>,
    memory: Memory,
    func: TypedFunc<(i32, i32, i32), i32>,
    buf: [u8; N*N],
}

impl<const N: usize> WasmPlayer<N> where [(); N*N]: Sized {
    pub fn new(wasm: &[u8]) -> Result<Self, Box<dyn Error>> {
        let engine = Engine::default();

        // We start off by creating a `Module` which represents a compiled form
        // of our input wasm module. In this case it'll be JIT-compiled after
        // we parse the text format.
        let module = Module::new(&engine, wasm)?;

        // A `Store` is what will own instances, functions, globals, etc. All wasm
        // items are stored within a `Store`, and it's what we'll always be using to
        // interact with the wasm world. Custom data can be stored in stores but for
        // now we just use `()`.
        let mut store = Store::new(&engine, ());

        let memory_ty = MemoryType::new(1, None);
        let memory = Memory::new(&mut store, memory_ty)?;

        // With a compiled `Module` we can then instantiate it, creating
        // an `Instance` which we can actually poke at functions on.
        let instance = Instance::new(&mut store, &module, &[memory.into()])?;

        // The `Instance` gives us access to various exported functions and items,
        // which we access here to pull out our `answer` exported function and
        // run it.
        let answer = instance.get_func(&mut store, "answer")
            .expect("`answer` was not an exported function");

        // There's a few ways we can call the `answer` `Func` value. The easiest
        // is to statically assert its signature with `typed` (in this case
        // asserting it takes no arguments and returns one i32) and then call it.
        let answer = answer.typed::<(i32, i32, i32), i32, _>(&store)?;
        Ok(Self {
            store, memory, func: answer, buf: [0; N*N],
        })
    }
}

impl<const N: usize> PlayerController<N> for WasmPlayer<N> where [(); N*N]: Sized {

    fn make_play(&mut self, game: &Game<N>) -> Result<Pos, Box<dyn Error>> {
        game.serialize(&mut self.buf);
        game.print();
        self.memory.write(&mut self.store, 0, &self.buf)?;

        let legal_moves = game.legal_moves(game.current_player());
        let legal_move_count = legal_moves.len();
        assert!(legal_move_count > 0);
        for (i, pos) in legal_moves.into_iter().enumerate() {
            let offset = pos.to_offset(N);
            println!("legal move {}: {:?}", i, pos);
            self.buf[i] = offset;
        }
        self.memory.write(&mut self.store, self.buf.len(), &self.buf)?;

        // And finally we can call our function! Note that the error propagation
        // with `?` is done to handle the case where the wasm function traps.
        let ans = self.func.call(&mut self.store,
                                 (N as i32,
                                  legal_move_count as i32,
                                  game.current_player().serialize() as i32
                                  ))?;
        Ok(Pos::from_offset(ans as u8, N))
    }
}
