// 标准库支持的比较少，没有内置的宏来创建 HashMap
// 数据存储在heap上

// 在 Rust 中，HashMap<K, V> 要求键（K）和值（V）是拥有所有权的类型
// 对于实现了 Copy Trait 的类型（比如：i32），其值会复制到HashMap
// 对于拥有所有权的值（比如：String），其值会移动，所有权会转移到HashMap
// 如果只是将值的引用插入到HashMap，值本身不会移动

// 访问 HashMap 中的值，采用 get 方法，返回值为 Option<&V>

use std::collections::HashMap;

fn main(){
    let mut map1: HashMap<String, i32> = HashMap::new();
    map1.insert(String::from("a"), 10);

    let coins: Vec<String> = vec![String::from("btc"), String::from("eth")];
    let mut prices: Vec<i32> = Vec::new();
    prices.push(10000);
    prices.push(4000);

    // coins.iter() 会生成 &String 类型的引用，而 prices.iter() 会生成 &i32 类型的引用
    let map2: HashMap<&String, &i32> = coins.iter().zip(prices.iter()).collect();

    let key_btc = "btc".to_string();
    let price_btc = map2.get(&key_btc);                 // get 方法，返回值为 Option<&V>

    match price_btc {
        Some(value) => println!("{}", value),
        None => println!("None")
    }

    for (k, v) in &map2 {
        println!("{}: {}", k, v)
    }

    let key: String = String::from("key");
    let value: String = String::from("value");
    let mut map3: HashMap<String, String> = HashMap::new();
    map3.insert(key, value);
    // println!("{}: {}", key, value);                      // 对于拥有所有权的值（比如：String），其值会移动，所有权会转移到HashMap


}