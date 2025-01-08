enum Gender {
    Female,
    Male
}

#[derive(Debug)]
enum Country {
    US,
    UK,
    JP
}
enum Computer {
    Dell,
    Lenovo,
    Asus(Country)                               // 绑定值模式
}


fn get_standard(gender: Gender) -> u8 {
    match gender {
        Gender::Female => {                     // 如果某个模式有多行代码，需要使用 {}
            println!("you are female");
            50
        },
        Gender::Male => 60                      // 如果某个模式只有单行代码，直接返回即可
    }
}

fn get_computer(computer: Computer) {
    match computer {
        Computer::Dell => {
            println!("dell");
        },
        Computer::Lenovo => {
            println!("lenovo");
        },
        Computer::Asus(country) => {
            println!("Asus is from {:#?}", country);
        }
    }
}

fn main(){
    let computer: Computer = Computer::Asus(
        Country::US
    );
    get_computer(computer);
}