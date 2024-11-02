pub trait MakesSound {
    fn make_sound(&self) -> String;
}

pub struct Cat;
pub struct Dog;
pub struct ItalianPerson;

impl MakesSound for Cat {
    fn make_sound(&self) -> String {
        "Meow".to_string()
    }
}

impl MakesSound for Dog {
    fn make_sound(&self) -> String {
        "Woof".to_string()
    }
}

impl MakesSound for ItalianPerson {
    fn make_sound(&self) -> String {
        "Ciao".to_string()
    }
}

pub fn things_that_make_sound_dynamic(list_of_things: Vec<Box<dyn MakesSound>>) -> Vec<String> {
    list_of_things
        .iter()
        .map(|thing| thing.make_sound())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{things_that_make_sound, things_that_make_sound_dynamic, Cat, Dog, ItalianPerson, MakesSound};

    #[test]
    fn a_cat_makes_a_miau_sound() {
        let cat = Cat;
        assert_eq!(cat.make_sound(), "Miau".to_string());
    }

    #[test]
    fn a_dog_makes_woof_sound() {
        let dog = Dog;
        assert_eq!(dog.make_sound(), "Woof".to_string());
    }

    #[test]
    fn an_italian_shouts_mammamia() {
        let person = ItalianPerson;
        assert_eq!(person.make_sound(), "MammaMia".to_string());
    }

    #[test]
    fn a_zoo_of_animals_make_funny_sounds() {
        let animals: Vec<Box<dyn MakesSound>> = vec![Box::new(Cat), Box::new(Dog), Box::new(ItalianPerson)];
        let sounds = things_that_make_sound_dynamic(animals);
        assert_eq!(sounds, vec!["Miau".to_string(), "Woof".to_string(), "MammaMia".to_string()]);
    }
}