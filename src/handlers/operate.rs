use std::fs::{read_dir, remove_dir_all, remove_file};
use std::io; //3
use std::path::Path;

pub fn read_delete_dir(p: &Path, n: &str) -> io::Result<()> {
    // 准备删除文件夹
    // let path = Path::new(p);
    let result = read_dir(p)?;
    for entry in result {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let deep_path = format!("{}", path.display());
            let folder_name = deep_path.split("\\").last().unwrap();
            if folder_name == n {
                println!("删除文件夹地址 = {:?}，删除文件夹名称 = {:?}", deep_path, n);
                remove_dir_all(path)?;
            } else {
                read_delete_dir(&path, n)?;
            }
        }
    }
    Ok(())
}

pub fn read_delete_file(p: &Path, n: &str) -> io::Result<()> {
    // 准备删除文件夹
    // let path = Path::new(p);
    let result = read_dir(p)?;
    for entry in result {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let deep_path = format!("{}", path.display());
            let file_name = deep_path.split("\\").last().unwrap();
            if file_name == n {
                println!("删除文件夹地址 = {:?}，删除文件名称 = {:?}", deep_path, n);
                remove_file(path)?;
            }
        } else {
            read_delete_file(&path, n)?;
        }
    }
    Ok(())
}
