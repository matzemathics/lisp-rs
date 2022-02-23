
(set-global lambda 
    (lambda-eval 
        $(args stats) 
        (lambda-eval args stats))) 
        
(set-global define
    (lambda-eval
        $(name args expr)
        (set-global name
            (lambda-eval $args expr)
        )
    )
)

(define if (expr stat1 stat2) 
    ((get-property (eq expr true) if-true) (eval stat1) (eval stat2)))


(define progn x 
    (if (eq (cdr $x) $())
        $x
        (ignore-fst 
            (eval (car $x)) 
            (progn (resolve (cdr $x)))    
        )
    )
)

(progn (set-local a 2) a)