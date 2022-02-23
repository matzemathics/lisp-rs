
(set-global lambda 
    (lambda-eval 
        $(args stats) 
        (lambda-eval (resolve args) (eval stats)))) 
        
(set-global define
    (lambda 
        (name args expr) 
        (set-global 
            (resolve name) 
            (lambda-eval (resolve args) (eval expr))))) 
            
(define if (expr stat) 
    (get-property
        (eq expr true)
        if-true)
    stat false)

(define inc (x) (+ x 1))

(define progn x 
    (if (not (empty? x)) 
    (discard-first 
        (car x) 
        (progn (cdr x)))))

(inc 1)

(define for (start end body)
    (if (< start end)
    (progn
        (funcall body start)
        (for (inc start) end body))))

(for 1 5 (lambda (x) (print x)))