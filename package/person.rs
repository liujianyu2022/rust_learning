// 引入 Female 和 Male 模块
pub use crate::attributes::{Female, Male};

// 定义 Person 结构体
pub struct Person {
    pub female: Female,
    pub male: Male,
}
