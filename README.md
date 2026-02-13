# NoteSanitizer

NoteSanitizer is a simple Rust command-line tool that cleans and formats text files.

## Features

- Removes blank lines
- Removes duplicate lines
- Trims unnecessary spaces
- Converts text to lowercase
- Numbers each cleaned line

## How to Run

Make sure Rust is installed.

Then run:

cargo run example.txt

## Example

Input file:
Rust is amazing

RUST is amazing
   Rust is amazing   

I love programming

I love programming

Output:
1: rust is amazing
2: i love programming

## Author
Doreen Lomoteley Lomotey
