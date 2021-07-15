# CLAY
CLAY (Common Lisp According to Yogurt) is a version of common lisp with an
interpreter written in rust. Development is tested against SBCL.


### Development branch

this is for developmental purposes only. check the todos to see what being done.


### TODO:

* add in loadable .so lib files to add libraries written in other languages to
  clay. (probably with the [libloading crate](https://lib.rs/crates/libloading))
* add big nums (probably with the [num-bigint crate](https://lib.rs/crates/num-bigint))


### future TODOs:

* clean up the code (delete unneeded commented lines, fix cargo warnings, etc).
* enable saving of byte code to a file.
* add a token counter to the lexer (keeps track of how many tokens it has found.
  to make dealing with the parser easier.)
* optimize the parser. or maybe even write a newer faster one.


### Notes To Self:
* loadable .so external libraries: add the "`" (back tick char, just above the
  tilda) use this to signal compilar to not compile, or have a different list
  syntax. like square brackets or something. that way the lisp code will look
  like, "(from extern_lib `(extern_func arg_1 ... arg_n))" something like that.


### Acknowledgement:

I got a good start on this project, and not to mention a better grasp of rust
from looking at another similar GitHub project [(this github)](https://github.com/samrat/rusl/blob/master/src/lexer.rs "samrat/rusl").
this is the same place I got the Lexer from.
