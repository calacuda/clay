(do ((x 0 (+ 2 x))
     (y 20 (- y 2)))
    (= x y)
   (write-line "x = " x " y = " y)
)