;; Input
(module
    (memory (;0;) 17)
)

;; EXPECTED-RESULT:
(module
    (type (;0;) (func))
    (import "leonardo" "yield_" (func (;0;) (type 0)))
    (import "leonardo" "memory" (memory (;0;) 17))
    (global (;0;) (mut i32) (i32.const 0))
)