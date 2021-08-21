"""
run tests.py

runs all the test files in the directory.

By: Calacuda (yogurt) | MIT Licence | Epoch: Aug 21, 2021
"""


import os


MODE = "JIT"


def get_exe():
    """gets the correct compiled binary"""
    if os.path.isfile("target/release/clay"):
        exe = "target/release/clay"
    elif os.path.isfile("target/debug/clay"):
        exe = "target/debug/clay"
    else:
        print("compileing...")
        os.system("cargo build")
        exe = "target/debug/clay"
    return exe


def run_test(fname, exe):
    """runs a single test"""
    run_mes = f"  running :  {fname}"
    breaker = "=" * (len(run_mes) + 2)
    # print start message
    print(breaker)
    print(run_mes)
    print(breaker)
    # run test
    os.system(f"./{exe} {MODE} {fname}")
    # print footer
    print(breaker, end="\n\n")


def main():
    """main func"""
    exe = get_exe()

    test_files =  os.listdir("test_inputs")
    test_files.sort()

    for fname in test_files:
        run_test(f"test_inputs/{fname}", exe)


if __name__ == "__main__":
    main()
