# https://stackoverflow.com/questions/16931770/makefile4-missing-separator-stop
default: rust_print main.c
	clang main.c librust_print.a -lpthread -ldl

rust_print: rust_print.rs
	rustc rust_print.rs --crate-type staticlib

clean:
	rm -rf librust_print.a a.out
