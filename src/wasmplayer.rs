use std::error::Error;
use crate::game::Game;
use crate::game::Pos;
use crate::game::PlayerController;
use wasmtime::*;

pub struct WasmPlayer<const N: usize> where [(); N*N*2]: Sized {
    store: Store<()>,
    memory: Memory,
    func: TypedFunc<(i32, i32, i32, i32, i32), i32>,
    buf: [u8; N*N*2],
    wasm_memory_offset: i32,
}

impl<const N: usize> WasmPlayer<N> where [(); N*N*2]: Sized {
    pub fn new(wasm: &[u8]) -> Result<Self, Box<dyn Error>> {
        let engine = Engine::default();

        let module = Module::new(&engine, wasm)?;
        let mut store = Store::new(&engine, ());
        let instance = Instance::new(&mut store, &module, &[])?;

        let alloc = instance.get_func(&mut store, "alloc_wasm_memory")
            .expect("`alloc_wasm_memory` was not an exported function");
        let alloc = alloc.typed::<i32, i32, _>(&store)?;

        // Get linear memory. By using an export and letting the wasm module handle
        // allocations we avoid writing to memory from host side without guest
        // being aware of it. It also seems quite tricky to compile languages to
        // wasm and get it to use imported memory.
        let memory = instance
            .get_memory(&mut store, "memory")
            .expect("failed to find `memory` export");

        let answer = instance.get_func(&mut store, "answer")
            .expect("`answer` was not an exported function")
            .typed::<(i32, i32, i32, i32, i32), i32, _>(&store)?;

        // Board occupies N*N, legal moves never occupy more than N*N.
        // This is all the memory we'll use, so we don't need the alloc
        // function anymore. We'll use it for the entire duration of the
        // game, so no need for a dealloc function.
        let ptr = alloc.call(&mut store, (N*N*2) as i32)?;
        Ok(Self {
            store, memory, func: answer, buf: [0; N*N*2], wasm_memory_offset: ptr,
        })
    }
}

impl<const N: usize> PlayerController<N> for WasmPlayer<N> where [(); N*N*2]: Sized {

    fn make_play(&mut self, game: &Game<N>) -> Result<Pos, Box<dyn Error>> {
        game.serialize(&mut self.buf); // Write the first N*N bytes
        game.print();

        let legal_moves = game.legal_moves(game.current_player());
        let legal_move_count = legal_moves.len();
        assert!(legal_move_count > 0);
        for (i, pos) in legal_moves.into_iter().enumerate() {
            let offset = pos.to_offset(N);
            println!("legal move {}: {:?}", i, pos);
            self.buf[N*N+i] = offset;
        }
        self.memory.write(&mut self.store, self.wasm_memory_offset as usize, &self.buf)?;

        let ans = self.func.call(&mut self.store,
                                 (self.wasm_memory_offset as i32,
                                  N as i32,
                                  self.wasm_memory_offset + (N*N) as i32,
                                  legal_move_count as i32,
                                  game.current_player().serialize() as i32,
                                  ))?;
        Ok(Pos::from_offset(ans as u8, N))
    }
}
