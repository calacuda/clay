(defun nothing ()
  (write-line "hello world!")
  )

(import `("test.lisp"))

(write-line (fact 5))
