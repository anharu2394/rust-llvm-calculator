# The calculator using Inkwell and Rust Peg

This is my first llvm project. 🎉

This project uses Rust Peg to parse, Inkwell to compile.

Using Failure, This project handle errors effectively.

You can run:

```shell
$ cargo run '5 * 4 + 20 * 4 - 50'
problem is 5 * 4 + 20 * 4 - 50
The answer is 50
```

## Dependencies

-   llvm 7.0.x
