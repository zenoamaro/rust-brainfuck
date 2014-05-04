Rust-brainfuck
==============

A Brainfuck interpreter made in Rust.

- Yes, it works, and it's fully compliant.
- No, it is not very fast (or at all (yet)).
- Yes, it is overkill.
- Yes, it is useless. Man, did you play with receipts as a kid or something?


Usage
-----

Make sure to have _Rust 0.10_ and _Make_ available:

```bash
$ make
mkdir -p dist
rustc -O --out-dir dist src/lib.rs
rustc -O -L dist -o dist/bf src/main.rs

$ echo "Lbh unir n ehfgl yrnx, znqnz." | dist/bf examples/rot13.bf
You have a rusty pipe, madam.
```

### Notes

- `EOF` is represented as zero. Be sure to take that into account when running your programs. For example, the _rot13_ program described on Wikipedia won't work here.

- I/O operators `,` and `.` read from `STDIN` and write to `STDOUT` respectively.


Motivation
----------

I am learning Rust, and Brainfuck happens to be a fun, simple project, which satisfies the following requisites:

- Small: it is compact enough to be kept entirely in the head at once.
- Flexible: it can be tackled in many different ways.
- Comprehensive: it requires learning many different things.


Plans and considerations
------------------------

- Test coverage is practically non-existent.

- There is some unnecessary cloning of data, and many instantiation and borrows are probably more costly or less strict than they should. I was kinda expecting this to happen, though.

- Parsing is not very efficient at the moment. Besides, non-orthogonal operators like `Incr` and `Decr`, or `Prev` and `Next` could be compacted to `Mutate` and `Seek`. It makes sense to lose the 1:1 compliance to the source to enable some low-hanging optimizations like operator condensation.

- There are many interesting variations on the base language, which could be put behind some flag.

- It should be easy enough to add a debugger (a rewindable one would be nice).

- Performance is laughable, especially when compared to stuff like `bff4`. Rust's integrated micro-benchmarking may help with that. Right now I believe the culprits could be the many unneeded string manipulations, copying of data, and the unoptimized parsed tree.

- It shouldn't be too hard to produce a compiled binary, or some custom byte-code. By the way, I saw some nice LLVM helpers hidden inside Rustc.

- It should be embeddable. And read mail.

