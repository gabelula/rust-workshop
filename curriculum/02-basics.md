# First steps: the Rust programming environment

## Exercise 1
### Hello World

[Documentation](http://doc.rust-lang.org/book/getting-started.html#hello-world)

Goals: to verify that Rust is installed correctly, learn how to compile a program, learn how to create a function and use the println! macro

    fn main() {
        println!("hello, world");
    }

## Exercise 2
### Cargo

[Documentation](http://doc.rust-lang.org/book/getting-started.html#hello-cargo)

Goals: verify that Cargo is installed correctly, learn how to build and package Rust code using the preferred system

## Exercise 3
### Build for release

[Documentation](http://doc.rust-lang.org/book/getting-started.html#building-for-release)

Goals: learn how to build with release optimizations, and why we don’t need to do that during development

## Exercise 4
### Setting up a new project with Cargo

[Documentation](http://doc.rust-lang.org/book/getting-started.html#making-a-new-cargo-project-the-easy-way)

Goals: learn how to speed up development setup by using Cargo to create a project

`cargo new my_project` sets up the directory structure, a configuration template, and initializes git

## Exercise 5
### Compile errors

[Documentation](https://doc.rust-lang.org/error-index.html)

Goals: see a compile error in Rust, and talk about how to debug and resolve it

add `return x;` to the end of the hello world program