## Writing a wordcount utility

Finished example: https://github.com/steveklabnik/rwc

Steps:
- try out the built-in wordcount utility your system has: `echo "hello world" | wc -c`
- create a new project using `cargo new my_wc`
- you'll need the [getopts](http://doc.rust-lang.org/getopts/getopts/index.html) crate for this project. Read about [crates](http://doc.rust-lang.org/book/crates-and-modules.html)
- write a program that reads in options from the command line (like -c, -l, -h) and prints them out again
- modify your program to also read a command-line argument for the filename (of the file with the text to count)
- then make your program reject options that do not match the `wc` functionality (see `man wc` if you want to check)
- write a function that counts the words in the file
- write a function that counts the characters in the file
- return the number of words or characters in the file (as specified by the option flag)
- add help and version option handling
- handle the situation where no file is provided