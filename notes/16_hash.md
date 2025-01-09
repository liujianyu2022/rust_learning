solana 的账户地址，就是通过将账户的公钥进行 SHA-256 和 Keccak（SHA-3）哈希得到的，并最终用Base58编码（去除了Base64中比较容易混淆的字符，如0、o、1、l等）  

```rust
use std::collections::HashMap;                      // 由于 HashMap 并没有包含在 Rust 的 prelude 库中，所以需要手动引入
fn main() {

    // 创建一个HashMap，用于存储学生成绩
    let mut student_grades = HashMap::new();
    student_grades.insert("Alice", 100);
    
    // 创建指定大小的 HashMap，避免频繁的内存分配和拷贝，提升性能。
    let mut student_grades2 = HashMap::with_capacity(3);
    student_grades2.insert("Alice", 100);
    student_grades2.insert("Bob", 99);
    student_grades2.insert("Eve", 59);
}
```