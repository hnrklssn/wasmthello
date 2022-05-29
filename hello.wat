(module ;; minimal example bot
  (memory (export "memory") 1)
  (global $stack_ptr (mut i32) (i32.const 0))
  (func (export "answer") (param i32) (param i32) (param i32) (param i32) (param i32) (result i32)
     local.get 2
	  i32.load ;; return the first legal move (there is always at least 1)
  )
  (func (export "alloc_wasm_memory") (param i32) (result i32)
     global.get $stack_ptr ;; save original value
     global.get $stack_ptr
     local.get 0
	  i32.add
     global.set $stack_ptr ;; bump stack pointer
	  ;; leave previous pointer on stack as return value
  )
)

