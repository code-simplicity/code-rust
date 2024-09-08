use std::io;

/// 摄氏度和华氏度的转变
pub fn main() {
    println!("请输入摄氏温度");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("数据无效");
    let celsius: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("输入无效，请重新输入");
            return;
        }
    };
    println!("摄氏度: {}", celsius);
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    println!("华氏度: {}", fahrenheit);
}
