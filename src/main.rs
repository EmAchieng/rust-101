mod branching;
mod functions;
mod structs;
mod traits;

use structs::Person;
use traits::{Cat, Dog, ItalianPerson, MakesSound, things_that_make_sound_dynamic};
use functions::{add, subtract, hello, another_hello};

fn main() {
    let bigger = branching::get_bigger(10, 20);
    println!("The bigger number is {}", bigger);

    let result = branching::picky_eater("apple");
    println!("Picky eater says: {}", result);

    let mut person = Person {
        age: 25,
        name: "John".to_string(),
    };
    println!("Person's name is {}", person.name);
    println!("Person's age is {}", person.get_age());
    person.birthday();
    println!("After birthday, person's age is {}", person.age);

    let age = person.get_age();
    println!("John's age after calling get_age: {}", age);
    person.birthday();
    println!("John's age after another birthday: {}", person.age);

    let cat = Cat;
    let dog = Dog;
    let italian_person = ItalianPerson;

    let animals: Vec<Box<dyn MakesSound>> = vec![Box::new(cat), Box::new(dog), Box::new(italian_person)];
    let sounds_dynamic = things_that_make_sound_dynamic(animals);
    for sound in sounds_dynamic {
        println!("Animal makes sound: {}", sound);
    }

    let animals_static: Vec<Box<dyn MakesSound>> = vec![Box::new(Cat), Box::new(Dog), Box::new(ItalianPerson)];
    let sounds_static = things_that_make_sound_dynamic(animals_static);
    for sound in sounds_static {
        println!("Animal makes sound: {}", sound);
    }

    // Using functions from functions.rs
    let sum = add(5, 3);
    println!("Sum: {}", sum);

    let difference = subtract(10, 4);
    println!("Difference: {}", difference);

    let greeting = hello("Alice".to_string());
    println!("{}", greeting);

    let another_greeting = another_hello("Bob".to_string());
    println!("{}", another_greeting);
}