# DASH-autobahn
An improved version of [dsh](https://github.com/leeian1011/dsh.git)!<br>
Built with the memory safe Rust :^)

## Improvements & Upgrades
- No hard limit of 5 lanes, unlimited lanes baby!
- Nickname-able lanes!
- Powerful list function that allows us to list the directory within a lane!

## Learning exercise
This project is just to familiarize myself with the intricacies and improve
my foundational and fundamental understanding of rust!<br>
I also moved to Linux and porting my MacOS build of the C built dsh has been very eh.
I am also not very happy with the functionality of the C dsh and would like
to add much more capabilities to it!

### Side notes & things I've learned

- Smart pointers such as Box that allows us to have unknown sized structs
at compile time.

- Very surface level Process and IO management in Rust.

- Using serde_json as I have to work with JSON files in real life, hoping
this would improve my fundamental understanding of using them! (JSON is
just object notation its not that deep i know :p)

- Taking advantage of powerful iterators and traits from the standard
library like `From<T>`.

