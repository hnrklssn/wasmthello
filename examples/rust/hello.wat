(module
  (type (;0;) (func (param i32 i32 i32)))
  (type (;1;) (func (param i32) (result i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (func (;0;) (type 0) (param i32 i32 i32)
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func (;1;) (type 1) (param i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    block  ;; label = @1
      local.get 0
      i32.eqz
      if  ;; label = @2
        i32.const 1
        local.set 0
        br 1 (;@1;)
      end
      local.get 1
      i32.const 1048576
      i32.load
      i32.store offset=12
      local.get 0
      i32.const 3
      i32.add
      local.tee 2
      i32.const 2
      i32.shr_u
      local.tee 3
      local.get 1
      i32.const 12
      i32.add
      call 2
      local.tee 0
      if  ;; label = @2
        i32.const 1048576
        local.get 1
        i32.load offset=12
        i32.store
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 2
        i32.const -4
        i32.and
        local.tee 0
        i32.const 520
        local.get 0
        i32.const 520
        i32.gt_u
        select
        i32.const 65543
        i32.add
        local.tee 2
        i32.const 16
        i32.shr_u
        memory.grow
        local.tee 0
        i32.const -1
        i32.ne
        if  ;; label = @3
          local.get 0
          i32.const 16
          i32.shl
          local.tee 0
          i32.const 0
          i32.store offset=4
          local.get 0
          local.get 1
          i32.load offset=12
          i32.store offset=8
          local.get 0
          local.get 0
          local.get 2
          i32.const -65536
          i32.and
          i32.add
          i32.const 2
          i32.or
          i32.store
          local.get 1
          local.get 0
          i32.store offset=12
          local.get 3
          local.get 1
          i32.const 12
          i32.add
          call 2
          local.set 0
          i32.const 1048576
          local.get 1
          i32.load offset=12
          i32.store
          local.get 0
          br_if 2 (;@1;)
          br 1 (;@2;)
        end
        i32.const 1048576
        local.get 1
        i32.load offset=12
        i32.store
      end
      unreachable
    end
    local.get 1
    i32.const 16
    i32.add
    global.set 0
    local.get 0)
  (func (;2;) (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 1
    i32.load
    local.tee 2
    if  ;; label = @1
      local.get 0
      i32.const 2
      i32.shl
      local.set 6
      loop  ;; label = @2
        local.get 2
        i32.const 8
        i32.add
        local.set 3
        block  ;; label = @3
          local.get 2
          i32.load offset=8
          local.tee 4
          i32.const 1
          i32.and
          i32.eqz
          if  ;; label = @4
            local.get 2
            local.set 0
            br 1 (;@3;)
          end
          loop  ;; label = @4
            local.get 3
            local.get 4
            i32.const -2
            i32.and
            i32.store
            local.get 2
            i32.load offset=4
            local.tee 4
            i32.const -4
            i32.and
            local.tee 3
            if (result i32)  ;; label = @5
              i32.const 0
              local.get 3
              local.get 3
              i32.load8_u
              i32.const 1
              i32.and
              select
            else
              i32.const 0
            end
            local.set 0
            block  ;; label = @5
              local.get 2
              i32.load
              local.tee 5
              i32.const -4
              i32.and
              local.tee 7
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              local.get 7
              local.get 5
              i32.const 2
              i32.and
              select
              local.tee 5
              i32.eqz
              br_if 0 (;@5;)
              local.get 5
              local.get 5
              i32.load offset=4
              i32.const 3
              i32.and
              local.get 3
              i32.or
              i32.store offset=4
              local.get 2
              i32.load offset=4
              local.tee 4
              i32.const -4
              i32.and
              local.set 3
            end
            local.get 2
            local.get 3
            if (result i32)  ;; label = @5
              local.get 3
              local.get 3
              i32.load
              i32.const 3
              i32.and
              local.get 2
              i32.load
              i32.const -4
              i32.and
              i32.or
              i32.store
              local.get 2
              i32.load offset=4
            else
              local.get 4
            end
            i32.const 3
            i32.and
            i32.store offset=4
            local.get 2
            local.get 2
            i32.load
            local.tee 2
            i32.const 3
            i32.and
            i32.store
            local.get 2
            i32.const 2
            i32.and
            if  ;; label = @5
              local.get 0
              local.get 0
              i32.load
              i32.const 2
              i32.or
              i32.store
            end
            local.get 1
            local.get 0
            i32.store
            local.get 0
            i32.const 8
            i32.add
            local.set 3
            local.get 0
            local.tee 2
            i32.load offset=8
            local.tee 4
            i32.const 1
            i32.and
            br_if 0 (;@4;)
          end
        end
        local.get 6
        local.get 0
        i32.load
        i32.const -4
        i32.and
        local.tee 2
        local.get 0
        i32.const 8
        i32.add
        local.tee 4
        i32.sub
        i32.le_u
        if  ;; label = @3
          block  ;; label = @4
            local.get 2
            local.get 6
            i32.sub
            local.tee 2
            local.get 4
            i32.const 72
            i32.add
            i32.ge_u
            if  ;; label = @5
              local.get 2
              i32.const 0
              i32.store
              local.get 2
              i32.const 8
              i32.sub
              local.tee 2
              i64.const 0
              i64.store align=4
              local.get 2
              local.get 0
              i32.load
              i32.const -4
              i32.and
              i32.store
              block  ;; label = @6
                local.get 0
                i32.load
                local.tee 1
                i32.const -4
                i32.and
                local.tee 4
                i32.eqz
                br_if 0 (;@6;)
                i32.const 0
                local.get 4
                local.get 1
                i32.const 2
                i32.and
                select
                local.tee 1
                i32.eqz
                br_if 0 (;@6;)
                local.get 1
                local.get 1
                i32.load offset=4
                i32.const 3
                i32.and
                local.get 2
                i32.or
                i32.store offset=4
              end
              local.get 2
              local.get 2
              i32.load offset=4
              i32.const 3
              i32.and
              local.get 0
              i32.or
              i32.store offset=4
              local.get 3
              local.get 3
              i32.load
              i32.const -2
              i32.and
              i32.store
              local.get 0
              local.get 0
              i32.load
              local.tee 1
              i32.const 3
              i32.and
              local.get 2
              i32.or
              local.tee 3
              i32.store
              block  ;; label = @6
                local.get 1
                i32.const 2
                i32.and
                i32.eqz
                if  ;; label = @7
                  local.get 2
                  i32.load
                  local.set 0
                  br 1 (;@6;)
                end
                local.get 0
                local.get 3
                i32.const -3
                i32.and
                i32.store
                local.get 2
                local.get 2
                i32.load
                i32.const 2
                i32.or
                local.tee 0
                i32.store
              end
              local.get 2
              local.get 0
              i32.const 1
              i32.or
              i32.store
              br 1 (;@4;)
            end
            local.get 1
            local.get 3
            i32.load
            i32.const -4
            i32.and
            i32.store
            local.get 0
            local.get 0
            i32.load
            i32.const 1
            i32.or
            i32.store
            local.get 0
            local.set 2
          end
          local.get 2
          i32.const 8
          i32.add
          return
        end
        local.get 1
        local.get 0
        i32.load offset=8
        local.tee 2
        i32.store
        local.get 2
        br_if 0 (;@2;)
      end
    end
    i32.const 0)
  (func (;3;) (type 3) (param i32 i32 i32 i32 i32) (result i32)
    local.get 2
    i32.load8_u)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048580))
  (global (;2;) i32 (i32.const 1048592))
  (export "memory" (memory 0))
  (export "read_wasm_memory" (func 0))
  (export "alloc_wasm_memory" (func 1))
  (export "answer" (func 3))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2)))
