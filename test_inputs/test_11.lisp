(defun nothing ()
  (write-line "hello world!")
  )

(import `("test_inputs/test_01.lisp"))

(write-line (fact 5))
