
(defun inc (x) (+ x 1))

(defun for (start end body)
    (if (< start end)
    (progn
        (funcall body start)
        (for (inc start) end body))))

(for 1 5 (lambda (x) (print x)))