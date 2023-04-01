mod read_list;

use read_list::Person;
use std::fs;
use std::env;

fn check_list(name: &str) -> bool {
    for file in fs::read_dir(env::current_dir().expect("错误! 无法获取工作目录。").as_path()).expect("错误! 无法读取当前目录的文件名。") {  //遍历目录
        if file.expect("错误! 无法获取某个文件名。").file_name().to_string_lossy().contains(name) {return true;}
    }
    false
}

fn main() {
    let mut badguys = Vec::new();
    println!("当前目录是：{}", env::current_dir().expect("错误! 无法获取工作目录。").display());
    let names = Person::from_file("names.json").expect("错误！无法正确读取 JSON 文件。");
    for name in names.get_names() { //遍历名单
        if check_list(&name) {println!("{}\t已提交",name)}
        else {badguys.push(name)}
    }
    println!("未提交：{:?}",badguys);
}