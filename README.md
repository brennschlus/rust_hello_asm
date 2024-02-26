# Hello World in Rust and Assembly

This is a simple Rust program that prints “Hello, world!” to the terminal using inline assembly and system calls. It does not use the standard library or the main function, and it compiles to a very small binary of only 648 bytes.

### How it works
The program uses the asm! macro to write inline assembly code that invokes the write and exit system calls. The write system call takes three arguments: the file descriptor to write to (1 for standard output), the pointer to the buffer to write from, and the number of bytes to write. The exit system call takes one argument: the exit code of the program (0 for success). The program passes these arguments using the registers rax, rdi, rsi, and rdx, according to the System V AMD64 ABI calling convention.

The program defines a constant byte array b"Hello, world!\n" as the message to print, and uses the as_ptr method to get a pointer to its first element. The program also defines a #[no_mangle] attribute for the _start function, which is the entry point of the program. The program uses the unsafe keyword to indicate that the inline assembly code may have undefined behavior or violate memory safety.

The program also defines a #[panic_handler] attribute for the my_panic function, which is called when the program panics. The function simply loops forever, as there is no way to handle a panic in this minimal program.

### How to compile and run
To compile this program, you need Rust installed in your system. You also need to pass some flags to the compiler and the linker to optimize the binary size and avoid linking to the standard library or the dynamic linker. The command to compile the program is:

```bash
RUSTFLAGS="-Ctarget-cpu=native -Clink-args=-nostartfiles -Clink-args=-Wl,-n,-N,--no-dynamic-linker,--no-pie,--build-id=none" cargo +nightly build --release
```

This will create a binary file named hello_world_rust in the target/release directory. To run the program, simply execute the binary file:

```bash
./target/release/hello_world_rust
```

You should see the output “Hello, world!” in the terminal.

### How to learn more
If you are interested in learning more about Rust and system programming, you can check out some of these resources:

- [The Rust Programming Language](https://doc.rust-lang.org/book/) - The official book on Rust, covering the basics, syntax, features, and idioms of the language.
- [Writing an OS in Rust](https://os.phil-opp.com/) - A blog series that teaches how to create a minimal operating system kernel in Rust, from scratch.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - A collection of runnable examples that illustrate various Rust concepts and techniques.
- [A very small Rust binary indeed](https://darkcoding.net/software/a-very-small-rust-binary-indeed/) - Main source of inspiration for this repository.
