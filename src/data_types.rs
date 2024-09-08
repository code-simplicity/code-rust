use std::io;

// 数据类型
// rust是静态类型数据，所以我们在编写程序的时候，类型一定是知道的，不像js那种动态类型的
pub fn main() {
    let guess: u32 = "42".parse().expect("Not a number!!");
    println!("guess ==> {guess}");

    // 标量类型
    // 分为四部分：整数、浮点数、布尔值、字符类型
    // 整型，分为有符号和无符号，有符号使用i表示，无符号使用u表示，分为8，16，32，64，128位 isize和usize
    // 会根据硬件平台来决定
    let max_a: i8 = 127;

    // 浮点数采用IEEE-754标准表示
    let max_b = 2.0; // 64位
    let max_c: f32 = 3.0; // 32位

    // 这里浮点数相加会得到一个浮点数,如果走默认的f64，得到的是0.30000000000000004，如果是走f32，得到的是0.3，因为IEEE-754标准中会有这样的问题
    let total: f32 = 0.1 + 0.2;
    println!("total ==> {total}");

    // 下面的都是一些数学运算，常规的计算逻辑
    // 加
    let sum = 5 + 10; // 15

    // 减
    let difference = 95.5 - 4.3; // 91.2

    // 乘法
    let product = 4 * 30; // 120

    // 除法
    let quotient = 56.7 / 32.2; // 1.75

    // 取余
    let remainder = 43 % 5; // 3

    // 布尔型, 一般数据给定之后，类型就确定了，不需要再额外处理，但是有时候我们也是有场景需要进行类型标注的
    let t = true;

    let f: bool = false;

    // 字符串类型
    let c = 'z';

    let zh = '中';

    let emoji = '😻';
    println!("emoji ==> {emoji}");

    // end，以上都是标量类型

    // 下面的是符合复合类型，主要有元组（tuple）和数组（array）

    // 元组是将多个类型组合成一个整体，长度固定，不会改变
    let tup= (500, 6.4, 1);

    // 取位置，其实也允许解构
    let five_hundred = tup.0;
    let (x, y, z) = tup;
    println!("tup length ==> {five_hundred}");
    println!("x ==> {x}");
    println!("y ==> {y}");
    println!("z ==> {z}");

    // 数组，什么时候使用数组，这个数据长度不会变的时候，比如一个游戏，一个场景，背景图片，背景图片不会变，就使用数组
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    // 取数组元素
    println!("请输入一个数字.");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("输入失败！");

    let index: usize = index.trim().parse().expect("不是一个数字");

    let element = a[index];

    println!("取到了这个数组的值 {element}");
}
