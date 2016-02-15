# Learning Rust's Syntax

## Setting up

You can modify your Hello World example or create a new project with `cargo new` for these exercises. We'll also be using an in-browser tool to evaluate code.

## Exercise 1
### Variable bindings

[Documentation](http://doc.rust-lang.org/book/variable-bindings.html)

Goals: explore how variables are defined in Rust and what a "variable binding" is

We can define a variable like this:
`let x = 5;`

Variable bindings can set more than a single variable potentially, for example this pattern: `let (x,y) = (3,4);`

Rust is statically typed, but it uses type inference, meaning it guesses what the variable’s type is. We can make this explicit: `let x: i32 = 5;`

Variables in Rust are not mutable by default! Try:
`let x = 5;`
`x = 10;`
What happens when you compile this?

How do we make a variable binding mutable? http://doc.rust-lang.org/book/mutability.html
`let mut x = 5;`
`x = 10;`

See what types are available in Rust: http://doc.rust-lang.org/book/primitive-types.html

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

We can use the enumerate function to handle this http://doc.rust-lang.org/book/loops.html#enumerate

Practice this: update the for loop exercise to use .enumerate() and change the output to say: “fizzbuzz on loop #” when it reaches that option

If we have time:

- Ownership
- References and borrowing
- Lifetimes