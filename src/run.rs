use crate::handlers::input::read_line;
use crate::handlers::operate::read_delete_dir;
use std::io;
use std::process;
use std::fs::{read_dir};
use std::path::Path;
// enum operations {
//   Some(String::from("0")),
//   Zero,
// }

pub fn start() -> io::Result<()> {
    // println!("Running");
    // read_line();
    loop {
        println!("请选择操作方式：");
        println!("1. 递归删除文件夹；");
        println!("0. 输入 0 退出程序！");
        println!();

        let line = read_line();

        let one = String::from("1");
        let zero = String::from("0");

        // if line == zero {
        //     println!("退出程序！");
        //     process::exit(1)
        // }
        match line {
            // 1 => println!("one"),
            // 2 => println!("two"),
            // 3 => println!("three"),
            _x1 if line == one => {
              println!("输入删除文件夹路径+文件夹的名称：（例如：D:/work+node_modules）");
              let line_str = read_line();
              let path_file_name:Vec<&str> = line_str.split("+").collect();
              println!("path_file_name = {:#?}", path_file_name);
              let path_name = &path_file_name[0];
              let file_name = &path_file_name[1];
              println!("path_name = {:#?}, file_name = {:#?}", path_name, file_name);
              let path = Path::new(path_name);
              read_delete_dir(path, file_name);
            }
            _x0 if line == zero => {
                println!("退出程序！");
                process::exit(1);
            }
            _ => {
                println!("请选择正确的操作方式！");
                println!("↓");
            }
        }
    }
}
