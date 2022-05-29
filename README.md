# Wasmthello
WebAssembly runtime for Othello bots. See [hello.wat](./hello.wat) for an example implementation, then build your own and compete against others!

## Protocol
Each bot needs to export a function named `answer` with the signature `(param i32) (param i32) (param i32) (param i32) (param i32) (result i32)`.
The parameters are:
1. A pointer to the board.
2. The size of the board. It is always an even number. Ex: for a standard 8x8 board the value will be 8.
3. A pointer to the list of legal moves available to the current player.
4. The number of currently legal moves. It is always greater than 0. If the player does not have any legal moves its round is automatically skipped.
5. The current player's identifier. It is either `1`, for the white player, or `2` for the black player.

Linear memory needs to be exposed by the module as an export, and can be used to access the board state and legal moves.

For a board size of `N`, the first `N*N` bytes following the board pointer contain the board state in column-major order (byte `N-1` is `(0,N-1)`, byte `N` is `(1,0)`). Each space is either `0`, if it is empty, `1` if it is occupied by a white tile, or `2` if it is occupied by a black tile.

For a number of legal moves `M`, the `M` bytes following the move list pointer contain the byte offsets of the spaces the current player is allowed to place their tile on. The value returned by the function needs to be a number contained in this list.

A function named `alloc_wasm_memory` also needs to be exported with the signature `(param i32) (return i32)`. The parameter is the number of bytes to allocate, the return value is the offset in linear memory (base pointer) to the allocated buffer. This buffer will be reused the entire game, so there is no need for a deallocation function to be exposed to the host.
