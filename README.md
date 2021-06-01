# CLAY
CLAY (Common Lisp According to Yogurt) is a version of common lisp with an
interpreter written in rust. Development is tested against SBCL.


### building clay:

First clone this repo. Then cd into the repo directory. Then build with the
release flag (cargo build --release). Then copy the binary to a somewhere in
your path variable.

on linux:
```
$ > git clone https://github.com/calacuda/clay.git
$ > cd clay
$ > cargo build --release
$ > cp /target/release/clay ~/.local/bin
```


### usage:

Consider this lisp function that calculates factorials recursively.
```
(defun fact (k)
  ;(write-line "k is " k)
  (if (< k 2)
      1
    (* k (fact (- k 1)))
    )
  )

(write (fact 5))
(terpri)
```

the line that is commented out is commented out because, it will not work in SBCL.

This code is stored in the "test.lisp" file. To run it with sbcl one would type:
```
$ sbcl --script test.lisp
```
to run with clay:
```
$ clay test.lisp
```


## benchmarks:

SBCL:
```
$ > hyperfine -r 1000 "sbcl --script test.lisp"
Benchmark #1: sbcl --script test.lisp
  Time (mean ± σ):       8.8 ms ±   2.3 ms    [User: 4.2 ms, System: 4.9 ms]
  Range (min … max):     6.9 ms …  17.9 ms    1000 runs
```

clay:

```
$ > hyperfine -r 1000 "target/release/clay test.lisp"
Benchmark #1: target/release/clay test.lisp
  Time (mean ± σ):       1.6 ms ±   1.3 ms    [User: 1.3 ms, System: 1.3 ms]
  Range (min … max):     0.5 ms …   9.2 ms    1000 runs
```

In other words my implementation is faster then SBCL. However SBCL is much, and
I mean much, better at dealing with large numbers.


### TODO:

* add explicit return statements (currently functions "return" the last variable
                                  that was mentioned or the last value that
                                  computed. one can work around the lack of
                                  return statements through conditionals and
                                  variables)
* add boolean detection capabilities to the lexer.
* make the rust part of the standard lib able to return values.
* add unsigned numbers in the background to make math more accurate.
* clean up the code (delete unneeded commented lines, fix cargo warnings, etc).
* enable saving of byte code to a file.
* add a token counter to the lexer (keeps track of how many tokens it has found.
  to make dealing with the parser easier.)
* optimize the parser. or maybe even write a newer faster one.


### Acknowledgement:

I got a good start on this project, and not to mention a better grasp of rust
from looking at another similar GitHub project [(this github)](https://github.com/samrat/rusl/blob/master/src/lexer.rs "samrat/rusl").
this is the same place I got the Lexer from.
