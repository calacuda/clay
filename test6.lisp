(let (num 10)
     (foo "monty")
     (bar "python"))

(defun times-two (x)
  (let (product (* x 2)))
  ;(write-line product)
  product
  )

(write-line num " time 2 is: " (times-two num) " ain't that a shock!")
(write-line product)
(write-line "but thats irrelevant, I like the movie '" foo " "
            bar " and the Holy Grail.' it was good")
