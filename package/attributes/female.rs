pub struct Female {
    pub name: String,
    pub age: u8,
}

impl Female {
    // 定义一个关联函数
    pub fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}
