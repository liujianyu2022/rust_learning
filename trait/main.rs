use rust_learning::Info;
use rust_learning::Dog;
use rust_learning::Monkey;

use rust_learning::get_info;

fn main(){
    let dog: Dog = Dog {
        name: String::from("dog"),
        age: 1
    };
    let monkey: Monkey = Monkey { 
        name: String::from("monkey"), 
        age: 2
    };

    println!("dog name: {}", dog.getName());
    println!("dog age: {}", dog.getAge());
    println!("monkey name: {}", monkey.getName());
    println!("monkey age: {}", monkey.getAge());

    get_info(dog);
}