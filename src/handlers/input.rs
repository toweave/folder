use std::io::stdin; //3

pub fn read_line() -> String {
    let stdin = stdin();
    let mut input_str = String::new(); //2
    stdin.read_line(&mut input_str).unwrap().to_string(); //4
    println!("您的输入是：{}", input_str); //5
    input_str.trim().to_string()
}
