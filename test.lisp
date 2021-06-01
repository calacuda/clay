(defun fact (k)
  (write-line "k is " k)
  (if (< k 2)
      1
    (* k 5 (fact (- k 1)))
    )
  )

(write (fact 5))
(terpri)
;(fact 5)
