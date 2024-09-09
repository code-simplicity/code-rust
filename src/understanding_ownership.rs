/// 所有权（系统）是 Rust 最为与众不同的特性，对语言的其他部分有着深刻含义。
/// 它让 Rust 无需垃圾回收（garbage collector）即可保障内存安全，因此理解 Rust 中所有权如何工作是十分重要的
/// 所有权，去进行内存的管理，是 Rust 中最核心的特性之一，管理数据
///
/// 所有权规则
///     Rust 中的每一个值都有一个所有者
///     值在任意时刻有且只有一个所有者
///     当所有者（变量）离开作用域之后，这个值将被丢弃
/// 当变量离开作用域之后，会主动调用drop去清理内存中的数据
pub fn main() {
    {
        // 开始x是无效的
        let x = 5; // 现在是有效的

        // 使用x，使用之后，x就无效了，因为使用完之后作用域就结束了
    }

    // x无效了

    // String类型。字符串字面量 "::"是一个语法，为何这里不能直接使用“hello”，这样得到的类型是&str，是一个字面量值，是不可变的
    let mut s = String::from("hello");

    // s得是一个mut，可变的
    s.push_str(", world");

    println!("s ==> {}", s);

    // 下面我们来看一个例子 变量与数据交互的方式
    // 1: 移动
    let mut s1 = String::from("hello");

    // 这里你认为s2是重新分配了一块内存吗，其实不是，
    // 这个需要了解一下String的组成部分，主要由ptr（指针）、len（长度）、capacity（容量）组成
    // s2只是拷贝了一份s1的三个组成部分，但是s1和s2指向同一块内存，所以s1和s2是同一个对象，所以s2是s1的一个拷贝
    // 这里s1 已经无效了，不能再对s1做任何操作了
    // 这里可以看作是将s1的所有权移动到s2，s1已经无效了，这里再Rust称为移动（move）
    let mut s2 = s1;
    println!("s2 {}", s2);

    // 2: 克隆
    // 我们有时候我们需要深度的复制String的数据，给我们提供了一个clone函数调用，内部实现
    let mut s3 = s2.clone();
    s3.push_str(", s3");
    println!("s2 {} s3 ==> {}", s2, s3);

    // 所有权与函数
    ///  Copy trait特殊注解
    /// 所有整数类型，布尔类型，浮点数类型，字符串类型，元组（特别注意要所有的类型都满足才是复合Copy特殊注解的）
    let mut  s = String::from("hello");

    takes_ownership(s);

    // println!("s 所有权失效 ==> {}", s);

    let x = 5;

    makes_copy(x);

    println!("x ==> {x}")

}

fn takes_ownership(some_string: String) {
    println!("takes_ownership {}", some_string);
}

fn makes_copy(some_integer: u32) {
    println!("takes_ownership {}", some_integer);
}
