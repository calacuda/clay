# CLAY
CLAY (Common Lisp According to Yogurt) is a version of common lisp with an interpreter written in rust. Developement is tested against SBCL.


### usage:

Consider this lisp function that calculates factorials recursively.

```
(defun fact (k)
  (if (< k 2)
      1
    (* k (fact (- k 1)))
    )
  )

(write (fact 5))
(terpri)
```