(module ;; example bot
  (memory (import "js" "mem") 0)
  (func (export "answer") (param i32) (param i32) (param i32) (result i32)
     local.get 0
     local.get 0
	  i32.mul ;; board_size * board_size skips the board and indexes into the list of legal moves
	  i32.load ;; return the first legal move (there is always at least 1)
  )
)

