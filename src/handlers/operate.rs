use std::io; //3
use std::fs::{read_dir};
use std::path::Path;

pub fn read_delete_dir(p: &Path, n: &str) -> io::Result<()> {
  // 准备删除文件夹
  // let path = Path::new(p);
  println!("path = {:?}", p);
  let result = read_dir(p)?;
  println!("result = {:?}", result);
  for entry in result {
    let entry = entry?;
    let path = entry.path();
    println!("path = {:?}", path);
    if path.is_dir() {
      println!("is_dir = {:?}", path.is_dir());
      // let deep_path = path.into_os_string();
      println!("deep_path = {:?}", path);
      read_delete_dir(&path, n);
    } 
  }
  Ok(())
}
