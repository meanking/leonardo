;; Input
(module
    (type $test (func))
    (import "env" "test" (func $test_import (type $test)))
    
    (func $local_test)
    (func (export "hello") (result i32)
        i32.const 45
        (loop
            ;; Tight
            (loop
             (br 0))

            ;; With local call
            (loop
                (call $local_test))
            
            ;; With import call
            (loop
                (call $test_import))
            
            ;; Loop inside condition
            (i32.lt_s
                (i32.const 5)
                (i32.const 10))
            (if (then
                (loop
                    (br 0))))
            
            ;; Loop with calls in both if branches
            (loop
                (i32.lt_s
                    (i32.const 5)
                    (i32.const 10))
                (if (then
                    (call $local_test))
                (else
                    (call $local_test))))
            
            ;; Complex loop scenario
            (loop
                (i32.lt_s
                    (i32.const 5)
                    (i32.const 10))
                (if (then
                    (loop
                        (br 0)))
                (else
                    (call $test_import))))
        )
    )
)

;; EXPECTED-RESULT:
(module
    (global (;0 reduction counter ;) (mut i32) (i32.const 0))
    (type (;0 yield type ;) (func))
    (type (;1;) (func (result i32)))
    (import "env" "test" (func $test_import (type 0)))
    (import "leonardo" "yield_" (func (;0;) (type 0)))

    (func (;2;) (type 1) (result i32)
        block  ;; Reduction counter logic
            global.get 0
            i32.const 1
            i32.add
            global.set 0
            global.get 0
            i32.const 10000
            i32.gt_s
            if
                call 1
                i32.const 0
                global.set 0
            else
            end
        end

        i32.const 45

        (loop
            ;; Tight
            (loop
                block  ;; Reduction counter logic
                    global.get 0
                    i32.const 1
                    i32.add
                    global.set 0
                    global.get 0
                    i32.const 10000
                    i32.gt_s
                    if
                        call 1
                        i32.const 0
                        global.set 0
                    else
                    end
                end
                (br 0)
            )

            ;; With local call
            (loop
                (call $local_test))
            
            ;; With import call
            (loop
                block  ;; Reduction counter logic
                    global.get 0
                    i32.const 1
                    i32.add
                    global.set 0
                    global.get 0
                    i32.const 10000
                    i32.gt_s
                    if
                        call 1
                        i32.const 0
                        global.set 0
                    else
                    end
                end
                (call $test_import))
            
            ;; Loop inside condition
            (i32.lt_s
                (i32.const 5)
                (i32.const 10))
            if
                (loop
                    block  ;; Reduction counter logic
                        global.get 0
                        i32.const 1
                        i32.add
                        global.set 0
                        global.get 0
                        i32.const 10000
                        i32.gt_s
                        if
                            call 1
                            i32.const 0
                            global.set 0
                        else
                        end
                    end
                    (br 0))
            else
            end

            ;; Loop with calls in both if branches
            (loop
                (i32.lt_s
                    (i32.const 5)
                    (i32.const 10))
                (if (then
                    (call $local_test))
                (else
                    (call $local_test))))
            

            ;; Complex loop scenario
            (loop
                block  ;; Reduction counter logic
                    global.get 0
                    i32.const 1
                    i32.add
                    global.set 0
                    global.get 0
                    i32.const 10000
                    i32.gt_s
                    if
                        call 1
                        i32.const 0
                        global.set 0
                    else
                    end
                end
                (i32.lt_s
                    (i32.const 5)
                    (i32.const 10))
                (if (then
                    (loop
                        block  ;; Reduction counter logic
                            global.get 0
                            i32.const 1
                            i32.add
                            global.set 0
                            global.get 0
                            i32.const 10000
                            i32.gt_s
                            if
                                call 1
                                i32.const 0
                                global.set 0
                            else
                            end
                        end
                        (br 0)))
                (else
                    (call $test_import))))
        )
    )

    (func $local_test (type 0)
        block  ;; Reduction counter logic
            global.get 0
            i32.const 1
            i32.add
            global.set 0
            global.get 0
            i32.const 10000
            i32.gt_s
            if
                call 1
                i32.const 0
                global.set 0
            else
            end
        end
    )

    (export "hello" (func 2))
)