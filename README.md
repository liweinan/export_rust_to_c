# Example of exporting RUST function to C code.

## USAGE

```bash
$ make                                                                                                                                                                                                                          11:31:47
rustc rust_print.rs --crate-type staticlib
clang main.c librust_print.a -lpthread -ldl
$ ./a.out                                                                                                                                                                                                                       11:31:49
Hello World!
$ git commit -a -m 'add clean target'
```
