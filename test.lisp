(defun fact (k)
  ;(write-line "k is " k)
  (if (< k 2)
      1
    (* k (fact (- k 1)))
    )
  )

(write (fact 65))
(terpri)
