/// 流程控制
pub fn main() {
    let t = true;

    // 条件必须是一个bool类型的，要不然编译直接报错，这就是静态类型的优势
    if t {
        println!("我的 true");
    } else {
        println!("我是 false");
    }

    // 条件控制语句
    let number = 6;

    if number != 0 {
        println!("number 不是 0");
        if number % 4 == 0 {
            println!("number 是 4 的倍数");
        } else if number % 2 == 0 {
            println!("number 是偶数");
        }
    } else if number % 2 == 0 {
        println!("number 是偶数");
    } else {
        println!("number 是奇数");
    }

    // 在let中使用条件控制语句
    let condition = !true;
    let number = if condition { 5 } else { 6 };
    println!("number ==> {number}");

    // 循环的妙用
    let mut number = 0;
    loop {
        number += 1;
        println!("again! {number}");
        // 满足条件推出循环
        if number == 1 {
            break;
        }
    }

    // 循环嵌套
    /// 内部循环通过执行break 'counting_up退出循环
    let mut count = 0;

    'counting_up: loop {
        println!(" count ==> {count}");

        let mut remaining = 10;

        loop {
            println!("remaining ==> {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    };

    println!("end loop {count}");

    // while循环
    let mut number = 3;
    while number != 0 {
        println!("while循环 {number}");
        number -= 1;
    }

    // 数组的for循环
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("循环出的数组元素: {}", a[index]);
        index += 1;
    }

    // while循环去循环数组要考虑到数组的长度，所以我们直接采用for选好看看
    for el in a {
        println!("数组 el {}", el)
    }

    // 数组反转，看起来代码更帅气，哈哈哈
    for el in (1..4).rev() {
        println!("反转数组 el {}", el);
    }
}
