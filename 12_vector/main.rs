
fn main(){

    let vector1: Vec<i32> = vec![0, 1, 2];

    let item: &i32= &vector1[2];                    // 索引访问，如果索引超出范围，程序会报错

    match vector1.get(2) {                          // get方法访问，如果索引超出范围，程序会执行None分支
        Some(item) => println!("{}", item),
        None => println!("none")
    }

    for item in &vector1 {
        println!("{}", item);
    }

    println!("---------------------------------------");

    let mut vector2: Vec<i32> = Vec::new();
    vector2.push(0);
    vector2.push(1);
    vector2.push(2);

    // 修改可变的vector
    for item in &mut vector2 {
        *item += 10;
    }

    for (index, item) in vector2.iter().enumerate() {
        println!("index: {}, item: {}", index, item);
    }

    println!("---------------------------------------");

    let mut vector3: Vec<i32> = Vec::from([1, 2, 3]);

    println!("is empty = {}", vector3.is_empty());                                 // 检查是否为空

    vector3.insert(0, 0);                                           // 在指定索引插入数据，索引值不能大于 v 的长度

    for (index, item) in &mut vector3.iter().enumerate() {
        println!("index: {}, item: {}", index, item);
    }

    assert_eq!(vector3.remove(1), 1);                                              // 移除指定位置的元素并返回
    assert_eq!(vector3.pop(), Some(3));                                            // 删除并返回 v 尾部的元素
    
}