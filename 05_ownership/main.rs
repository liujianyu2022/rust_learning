// 在 rust 中，内存是通过所有权系统来管理的，该系统包含了一组在编译的时候就会检查的规则
// 也就是说，rust 把内存管理的工作放在编译的时候就做完了，因此所有权特性不会影响运行的效率

// 数据存储的位置：stack or heap
// 所有存储在stack上数据的大小必须是已知的、固定的
// 编译时大小未知的、运行时大小可能变化的数据必须放在heap上
// 把数据存储到stack上的速度远快于heap
// heap中存放的数据，其组织性能会差一些。操作系统在heap中找到一块足够大的空间，将其标注为在用，然后返回一个指针，后续访问的时候需要通过指针进行跳转

// 存放在 stack 中的数据：
// 1. 静态大小的类型：静态大小的类型通常可以直接存储在栈上。这些类型包括但不限于整数（如i32、u64等）、浮点数（如f32、f64）、布尔值（bool）、字符（char）以及固定大小的数组等
// 2. 简单的数据结构：一些简单的数据结构，如元组（Tuple），如果其包含的元素都是静态大小的类型，那么整个元组也可以存储在栈上

// 存放在 heap 中的数据：
// 1. 动态大小的类型：对于动态大小的类型，如动态数组（Vec<T>）、哈希表（HashMap<K, V>）等，由于其大小在编译时未知，因此它们通常被分配在堆上


// 所有权解决的问题：主要是为了管理heap中的数据
// 1. 跟踪哪些代码在使用heap中的哪些数据
// 2. 最小化heap上的重复数据量
// 3. 清理heap上未使用的数据以避免空间不足

// 所有权的规则：
// 1. 每个值都有一个变量，这个变量就是这个值的所有者
// 2. 每个值只能有一个所有者
// 3. 当所有者超出作用域的时候，该值也会被删除

// 字符串：
// 字符串字面值 &str：由于在编译的时候就知道它的内容了，因此会被硬编码到可执行文件中，因此字符串字面值具有不可变性，高效等特点
// String类型：为了支持String的可变性，需要在heap中存储该值


// 移动（Move）

fn main(){
    let mut str1: String = String::from("hello");        // 从字符串字面值创建String
    str1.push_str(" world");

    println!("{}", str1);

    let str2: String = str1.clone();                    // 由于使用了clone，相当于把 stack 和 heap 中的数据都复制了一份
    println!("{}", str1);                               // 此时 str1 和 str2 相当于两个独立的变量，互不影响
    println!("{}", str2);                               

    let str3: String = str1;                            // str1已经被移动到了str3，此时str1已经失效了
    // println!("{}", str1);                            // borrow of moved value: `str1`

    
    let num: i32 = 10;
    test1(str3);                                    // 对于没有实现 Copy 的，将发生移动，此后 str3 将无法访问了
    test2(num);                                         // 如果实现了 Copy 的，将发生复制

    // println!("{}", str3);                            // borrow of moved value: `str3`
}

// 将值传递给函数将发生移动或者复制
fn test1(str: String) {

}

fn test2(num: i32){

}