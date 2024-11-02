# Rust 101 at BaselHack 2024

This project is a simple Rust application that demonstrates basic Rust concepts such as modules, traits, structs, and functions. It includes examples of arithmetic operations, trait implementations, and struct methods.

## Project Structure

The project is organized into the following modules:

- `branching`: Contains functions for conditional logic.
- `functions`: Contains basic arithmetic and greeting functions.
- `structs`: Defines a `Person` struct with methods.
- `traits`: Defines a `MakesSound` trait and implements it for several types.

## Modules and Functions

### `branching`

- `get_bigger(a: i32, b: i32) -> i32`: Returns the bigger of two integers.
- `picky_eater(food: &str) -> &str`: Returns a response based on the input food.

### `functions`

- `add(a: i32, b: i32) -> i32`: Returns the sum of two integers.
- `subtract(a: i32, b: i32) -> i32`: Returns the difference between two integers.
- `hello(name: String) -> String`: Returns a greeting message.
- `another_hello(name: String) -> String`: Returns another greeting message.

### `structs`

- `Person`: A struct representing a person with `age` and `name` fields.
  - `get_age(&self) -> u32`: Returns the age of the person.
  - `birthday(&mut self)`: Increments the person's age by one.

### `traits`

- `MakesSound`: A trait with a `make_sound` method.
  - Implemented for `Cat`, `Dog`, and `ItalianPerson`.
- `things_that_make_sound_dynamic(list_of_things: Vec<Box<dyn MakesSound>>) -> Vec<String>`: Returns a vector of sounds made by the list of things.

## Usage
To build the project, use the following command:

```sh
cargo build
```

To run the project, use the following command:

```sh
cargo run
```