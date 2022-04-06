# The Rust Programming Language
[The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html) is the official book on Rust: an open source systems programming language that helps you write faster, more reliable software. Rust offers control over low-level details (such as memory usage) in combination with high-level ergonomics, eliminating the hassle traditionally associated with low-level languages.

You'll begin with basics like creating functions, choosing data types, and binding variables and then move on to more advanced concepts, such as:

* Ownership and borrowing, lifetimes and traits
* Using Rust's memory safety guarantees to build fast, safe programs
* Testing, error handling and effective refactoring
* Generics, smart pointers, multithreading, trait objects and advanced pattern matching
* Using `Cargo`, Rust's built-in package manager, to build, test and document your code and manage dependencies
* How best to use Rust's advanced compiler with compiler-led programming techniques.

#### To install Rust
`curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

#### To update Rust
`rustup update`

#### To uninstall Rust
`rustup selft uninstall`

#### Local Documentation
The installer also includes a copy of the documentation locally, so you can read it offline. Run `rustup doc` to open the documentation in your browser.

###### NOTE:
Instructions for using a crate are in each crate's documentation. Another neat feature of Cargo
is that you can run the `cargo doc --open` command, which will build documentation provided by all of the dependencies locally and open it in your browser.