struct Foo {
    x: i32,
}

fn test(f: Foo) {
    println!("{}", f.x);
    // f 在这里被 dropped 释放
}

fn main() {
    let mut foo = Foo { x: 42 };
    let f = &mut foo;                       // 可变借用

    // test1(foo);                          // foo 已经被可变借用而无法取得其所有权
    // foo.x = 13;                          // foo 已经被可变借用而无法被修改
    
    f.x = 13;                               // f 会因为此后不再被使用而被 dropped 释放
    
    println!("{}", foo.x);
    
    foo.x = 7;                              // 现在修改可以正常进行，因为其所有可变引用 f 已经被 dropped 释放
    
    test(foo);                             // 移动 foo 的所有权到一个函数中
}