// 声明 female 和 male 子模块
// pub mod 或 mod 是子模块文件的入口，不能省略
pub mod female;
pub mod male;

// 在模块层次上公开 Female 和 Male 结构体
pub use female::Female;
pub use male::Male;
