pub fn main() {
    // 摄氏度和华氏度的转变
    let mut temp = 35.0;
    println!("摄氏度: {}", temp);
    temp = temp * 9.0 / 5.0 + 32.0;
    println!("华氏度: {}", temp);
}