# CLAY
CLAY (Common Lisp According to Yogurt) is a version of common lisp with an
interpreter written in rust. Development is tested against SBCL.


### whats working:

* lexer from [this github](https://github.com/samrat/rusl/blob/master/src/lexer.rs "samrat/rusl")
* parser successfully give a syntax tree.

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
  (if (< k 2)
      1
    (* k (fact (- k 1)))
    )
  )

(write (fact 5))
(terpri)
```

This code is stored in the test.lisp file. To run it with sbcl one would type:
```
$ sbcl --script test.lisp
```
to run with clay:
```
$ clay test.lisp
```


### TODO:

* write the bytecode compiler.
* write the bytecode interpreter.
* enable saving of byte code to a file.
* add a token counter to the lexer (keeps track of how many tokens it has found.
  to make dealing with the parser easier.)
* optimize the parser. or maybe even write a newer faster one.
