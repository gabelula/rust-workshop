# Learning Rust's Syntax

## Setting up

You can modify your Hello World example or create a new project with `cargo new` for these exercises. We'll also be using an in-browser tool to evaluate code.

## Exercise 1
### Variable bindings

[Documentation](http://doc.rust-lang.org/book/variable-bindings.html)

Goals: explore how variables are defined in Rust and what a "variable binding" is

We can define a variable like this:
`let x = 5;`

Variable bindings can set more than a single variable, for example this pattern: `let (x,y) = (3,4);`

Rust is statically typed, but it uses type inference, meaning it guesses what the variable’s type is. We can make this explicit: `let x: i32 = 5;`

Variables in Rust are not mutable by default! Try:
`let x = 5;`
`x = 10;`
What happens when you compile this?

How do we make a variable binding mutable? http://doc.rust-lang.org/book/mutability.html
`let mut x = 5;`
`x = 10;`

See what types are available in Rust: http://doc.rust-lang.org/book/primitive-types.html

[Strings](http://doc.rust-lang.org/book/strings.html) in Rust have some particularly interesting properties: `&str` defines "string slices" which are static, and `String` is used for growable UTF-8 strings

## Exercise 2
### Functions

[Documentation](http://doc.rust-lang.org/book/functions.html)

Goals: to learn how function declarations work, and practice writing a few of them

We already wrote a function in our Hello World program: `main` (every Rust program must have this function)
Here’s another:

    fn print_number(x: i32) {
        println!("x is: {}", x);
    }

Based on our hello world example, can you figure out how to write a program that uses this function? (if you get stuck or don't know where to start, see the documentation and ask a TA for help)

What happens when you call the function with an input that isn't an integer?

Functions in Rust don't have to return anything. If we do want them to have a return value, we specify it like this:

    fn add_one(x: i32) -> i32 {
       x + 1
    }

Writing `-> i32` tells Rust what the type of the return value will be. Notice the missing semicolon! The return value is an [expression](http://doc.rust-lang.org/book/functions.html#expressions-vs-statements) -- as are most things in Rust. Semicolons are used to separate expressions. We don't need one at the end of the return value -- it would cause the function to return an empty expression.

If we wanted to return a value before the end of the program, we could use the `return` keyword. [Documentation](http://doc.rust-lang.org/book/functions.html#early-returns)

*Update your program to add one to x before printing it.*

## Exercise 3
### if/else

Goals: experiment with simple boolean branching

[open the example](http://rustbyexample.com/flow_control/if_else.html)

Things to try: read through the code, run it, modify the value of n to reach all possible results

## Exercise 4
### Loops

[Documentation](http://doc.rust-lang.org/book/loops.html)

Goals: explore the options for iteration: loop, while, and for

let’s try some examples:

- http://rustbyexample.com/flow_control/loop.html
- http://rustbyexample.com/flow_control/while.html
- http://rustbyexample.com/flow_control/for.html

If you’re used to C-style for loops, you might ask: “how do I know how many times we’ve already looped when our range doesn’t start from 0? there’s no `i` to track it."

We can use the enumerate function to handle this: http://doc.rust-lang.org/book/loops.html#enumerate

Practice this: update the for loop exercise to use .enumerate() and change the output to say: “fizzbuzz on loop #” when it reaches that option

If we have time:

## Exercise 5
### Ownership

Ownership is how Rust achieves memory safety. There are three parts to how this works: [ownership](http://doc.rust-lang.org/book/ownership.html), [borrowing](http://doc.rust-lang.org/book/references-and-borrowing.html), and [lifetimes](http://doc.rust-lang.org/book/lifetimes.html).

exercises:

- [ownership and moves](http://rustbyexample.com/scope/move.html)
- [borrowing](http://rustbyexample.com/scope/borrow.html)
- [lifetimes](http://rustbyexample.com/scope/lifetime.html)

These examples might be the first time you've encountered the terms "stack" and "heap". If so, [read more](http://doc.rust-lang.org/book/the-stack-and-the-heap.html) about what this means.

This is just a brief introduction to this topic! To explore it further, read the sections of the Rust Book linked above, and try the other examples in Rust By Example chapter 14.

## Exercise 6
### Testing

[Documentation](http://doc.rust-lang.org/book/testing.html)

Goals: learn the basics of how to test our Rust code

Let's have a look at the project we created using `cargo new`. At the end of *main.rs*, do you see this line? `#[test]`

There's a default test included in our project. We can run this test with the command `cargo test`. Right now the text passes.

We could make it fail instead:

    #[test]
    fn it_works() {
       assert!(false);
    }

We can tell Rust to invert the result of the test when evaluating whether it passed:

    #[test]
    #[should_panic]
      fn it_works() {
        assert!(false);
    }

We can make an equality assertion as well:

    fn it_works() {
        assert_eq!("Hello", "world");
    }

Rust tests are typically grouped inside a [tests module](http://doc.rust-lang.org/book/testing.html#the-tests-module):

  #[cfg(test)]
  mod tests {
      use super::add_two;

      #[test]
      fn it_works() {
          assert_eq!(4, add_two(2));
      }
  }
