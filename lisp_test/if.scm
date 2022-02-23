
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

(define if (expr stat) 
    ((get-property (eq expr true) if-true) stat false))

(if (eq 2 2) 2212)