// 引入 lib.rs 中的内容  rust_learning 是 Cargo.toml配置文件中的 name 属性
use rust_learning::Person;
use rust_learning::attributes::{Female, Male};            

fn main(){
    // 创建 Female 和 Male 实例
    let female = Female::new("Alice", 30);
    let male = Male::new("Bob", 35);

    // 创建 Person 实例
    let person = Person { female, male };

    // 打印信息
    println!("Female: {}, Age: {}", person.female.name, person.female.age);
    println!("Male: {}, Age: {}", person.male.name, person.male.age);
}