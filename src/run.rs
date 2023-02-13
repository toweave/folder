use crate::handlers::input::read_line;
use crate::handlers::operate::{read_delete_dir, read_delete_file};
use std::io::{Error, Result};
use std::path::Path;
use std::process;
// enum operations {
//   Some(String::from("0")),
//   Zero,
// }

pub fn start() -> Result<()> {
    // println!("Running");
    // read_line();
    loop {
        println!("请选择操作方式：");
        println!("1. 递归删除文件夹；");
        println!("2. 递归删除文件；");
        println!("0. 输入 0 退出程序！");
        println!();

        let line = read_line();

        let one = String::from("1");
        let two = String::from("2");
        let zero = String::from("0");

        // if line == zero {
        //     println!("退出程序！");
        //     process::exit(1)
        // }
        match line {
            _x2 if line == two => {
                println!("输入删除文件夹路径+文件的名称：（例如：D:/work test.txt");
                let line_str = read_line();
                let path_file_name: Vec<&str> = line_str.split(" ").collect();
                if path_file_name.len() != 2 {
                    println!("输入数据不正确");
                    return Ok(());
                }
                let path_name = &path_file_name[0];
                let file_name = &path_file_name[1];
                println!(
                    "目标文件夹地址 = {:?}，目标文件名称 = {:?}",
                    path_name, file_name
                );
                let path = Path::new(path_name);
                read_delete_file(path, file_name)?;
                println!("操作完成！");
            }
            _x1 if line == one => {
                println!("输入删除文件夹路径+文件夹的名称：（例如：D:/work test");
                let line_str = read_line();
                let path_file_name: Vec<&str> = line_str.split(" ").collect();
                if path_file_name.len() != 2 {
                    println!("输入数据不正确");
                    return Ok(());
                }
                let path_name = &path_file_name[0];
                let file_name = &path_file_name[1];
                println!(
                    "目标文件夹地址 = {:?}，目标文件夹名称 = {:?}",
                    path_name, file_name
                );
                let path = Path::new(path_name);
                read_delete_dir(path, file_name)?;
                println!("操作完成！");
            }
            _x0 if line == zero => {
                println!("退出程序！");
                return Ok(());
                // process::exit(1);
            }
            _ => {
                println!("请选择正确的操作方式！");
                println!("↓");
            }
        }
    }
}
