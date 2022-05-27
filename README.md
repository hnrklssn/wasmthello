# Wasmthello
WebAssembly runtime for Othello bots. See [hello.wat](./hello.wat) for an example implementation, then build your own and compete against others!

## Protocol
Each bot needs to export 1 function named `answer` with the signature `(param i32) (param i32) (param i32) (result i32)`.
The first parameter is the size of the board. It is always an even number. Ex: for a standard 8x8 board the value will be 8.
The second parameter is the number of currently legal moves. It is always greater than 0. If the player does not have any legal moves its round is automatically skipped.
The third parameter is the current players identifier. It is either `1`, for the white player, or `2` for the black player.

1 page of linear memory is exposed as an import, and can be used to access the board state and legal moves.
For a board size of `N`, the first `N*N` bytes contain the board state in column-major order (byte `N-1` is `(0,N-1)`, byte `N` is `(1,0)`). Each space is either `0`, if it is empty, `1` if it is occupied by a white tile, or `2` if it is occupied by a black tile.
For a number of legal moves `M`, the bytes `[N*N..N*N+M)` contain the byte offsets of the spaces the current player is allowed to place their tile on. The value returned by the function needs to be a number contained in this list.
