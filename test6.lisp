(let (num 10)
     (foo "monty")
     (bar "python"))

(defun times-two (x)
  (let (product (* x 2)))
  (write-line product)
  (return 5)
  )

(write-line num " time 2 is: " (times-two num) " ain't that a shock!")
(write-line "but thats irrelevant, I like the movie '" foo " "
            bar " and the Holy Grail.' it was good")
