Don't be afraid. If you want to discuss about the design of any structure, feel
free to open an issue. If you have any doubt on how any structure works, you
can open an issue as well. I am open to discuss anything :D

# Documentation
If you write any method, iterator, data structure or whatever, you are not
required to write any documentation. However, writing documentation helps me
(or any other person) to understand what is your purpouse. But don't worry,
if you did not wrote any documentation and I need more info, I am going to
ask it.

# Testing
Besides writing unit tests, if you wrote anything sensitive or with `unsafe`
keyword, I would like you to write a new fuzz test. To add a new fuzz test,
just execute `cargo fuzz add <your_test_name>`, and to run
`./fuzz-run.sh <your_test_name> --sanitizer <choose-a-sanitizer>`. Be sure
to have installed `cargo-fuzz`. Write the test using the tiny "framework" I
wrote, named `fuzz_helper`, available as local package. You can look at
other fuzz tests as examples.

# FIXME
I don't know why, but memory sanitizer detects uninitialized value read when
initializing `fuzz_helper`. The weird is that I have no unsafe in it and no
reason to believe it is actually problem of my code. I believe it is an  issue
of libfuzzer itself.