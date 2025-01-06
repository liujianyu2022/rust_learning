use std::fmt;

struct User {
    name: String,
    age: u8,
}

// 相当于定义 toString() 方法
impl fmt::Display for User {
    // 通过 &mut 可变引用。fmt::Formatter 可以修改其内部的状态，比如追加内容到最终的输出中
    // <'_> 是一个生命周期标注，表示这个引用的生命周期。'_ 是一种省略写法，表示编译器可以自动推断出生命周期
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User(name: {}, age: {})", self.name, self.age)
    }
}

// 如果不实现 Dispaly，可以为结构体派生 Debug 特性
#[derive(Debug)]
struct Admin {
    name: String,
    age: u8,
}

impl Admin {
    // 结构体实例的方法，第一个参数为 self，相当于this
    fn getName(&self) -> &str {
        return &self.name[..];
    }
}

impl Admin {
    // 关联函数，第一个参数不是self，相当于静态方法
    fn createAdmin(name: &str, age: u8) -> Admin {
        return Admin { 
            name: String::from(name), 
            age 
        };
    }
}

// Unit-Like Struct   没有任何字段的结构体
// 适用于需要在某个类型上实现某个 trait，但是又不想在里面存储无关的数据
struct Empty {}

fn main() {
    let mut user: User = User {
        name: String::from("liu"),
        age: 18
    };

    user.name = String::from("wang");              // 如果希望更改结构体实例中的某个字段，那么该实例必须是mut

    println!("{}", user);                          // 需要结构体实现了 Display 

    let admin1: Admin = Admin {
        name: String::from("liu"),
        age: 18
    };
    println!("{:#?}", admin1);
    println!("{}", admin1.getName());

    // 调用关联函数
    let admin2: Admin = Admin::createAdmin("wang", 18);
    println!("{:#?}", admin2);
}

fn test() -> User {
    return User {
        name: String::from("liu"),
        age: 18
    };
}