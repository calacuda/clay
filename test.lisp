(defun fact (k)
  (if (< k 2)
      1
    (* k (fact (- k 1)))
    )
  )

(write (fact 5))
(terpri)
