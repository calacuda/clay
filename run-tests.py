"""
run tests.py

runs all the test files in the directory.

By: Calacuda (yogurt) | MIT Licence | Epoch: Aug 21, 2021
"""


import os


def run_test(fname, exe):
    """runs a single test"""
    run_mes = f"  running :  {fname}"
    breaker = "=" * (len(run_mes) + 3)
    # print start message
    print(breaker)
    print(run_mes)
    print(breaker)
    # run test
    os.system(f"./{exe} {fname}")
    # print footer
    print(breaker, end="\n\n")


def main():
    """main func"""
    if os.path.isfile("target/release/clay"):
        exe = "target/release/clay"
    elif os.path.isfile("target/debug/clay"):
        os.system("cargo build")
        exe = "target/debug/clay"
    else:
        print("ERROR: no executable found, please compile")
        return
    fname = "test.lisp"
    # run_test(fname, exe)
    i = 2
    while os.path.isfile(fname):
        run_test(fname, exe)
        fname = f"test{i}.lisp"
        i += 1


if __name__ == "__main__":
    main()
