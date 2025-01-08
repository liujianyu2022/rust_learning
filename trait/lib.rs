pub trait Info {
    fn getName(&self) -> String;                    // 只定义方法签名

    fn getAge(&self) -> u8 {                        // 提供默认实现
        0
    }
}

pub struct Dog {
    pub name: String,
    pub age: u8
}

impl Info for Dog {
    fn getName(&self) -> String {                   // 具体实现该方法
        format!("dog: name = {}", self.name)
    }
}

pub struct Monkey {
    pub name: String,
    pub age: u8
}

impl Info for Monkey {
    fn getName(&self) -> String {                   // 具体实现该方法
        format!("monkey: name = {}", self.name)
    }

    fn getAge(&self) -> u8 {                        // 重写默认实现
        self.age
    }
}

// Trait 作为参数
pub fn get_info(item: impl Info){
    println!("the get info method is called, name = {}, age = {}", item.getName(), item.getAge());
}

// Trait 作为返回值
pub fn get_dog() -> impl Info {
    Dog {
        name: String::from("dog"),
        age: 1
    }
}