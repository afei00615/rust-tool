use std::io::{self, Write};
use std::fs::{self,File};
use std::path::{Path,PathBuf};

use serde::{Serialize, Deserialize};

use fs_extra::dir::{self, CopyOptions};

use chrono::Local;

#[derive(Serialize, Deserialize)]
struct BackConfig{
    black_myth_path : String,
    back_path : String,
}
fn main() -> io::Result<()>{

    let file_path = "black_myth_config.toml";
    // let mut  black_myth_path = String::new();
    // let mut back_path = String::new();
    let mut black_config = BackConfig{
        black_myth_path : String::new(),
        back_path : String::new(),
    };
    if Path::new(file_path).exists() {
       let config_content = std::fs::read_to_string(file_path)?;
        black_config = toml::from_str(&config_content).expect("解析配置文件失败");
    //    black_myth_path = config.black_myth_path;
    //    back_path = config.back_path;
    }
    // if(black_myth_path.is_empty() || back_path.is_empty()){
    if black_config.black_myth_path.is_empty() || black_config.back_path.is_empty() {
        match create_config(){
            Ok(config) => black_config = config,
            Err(error) => return Err(error),
        }
        
    }
    println!("配置文件读取完成，开始备份");
    back_file(black_config);
    Ok(())
    
}

fn back_file(config: BackConfig){
    let mut options = CopyOptions::new();
    options.overwrite = true;
    
    let back_path = create_dated_folder(&config.back_path).unwrap();
    
    dir::copy(config.black_myth_path, &back_path, &options).expect("备份失败");
    println!("备份完成，备份目录{}",back_path);
}

fn create_dated_folder(base_dir: &str) -> std::io::Result<String> {
    // 获取当前日期
    let date = Local::now().format("%Y-%m-%d").to_string();
    let mut folder_name = date.clone();
    let mut counter = 0;

    loop {
        let mut folder_path = PathBuf::from(base_dir);
        folder_path.push(&folder_name);

        if !folder_path.exists() {
            // 如果路径不存在，则创建文件夹
            fs::create_dir_all(&folder_path)?;
            return Ok(folder_path.to_str().unwrap().to_string());
        }

        // 如果路径已存在，增加后缀
        counter += 1;
        folder_name = format!("{}-{}", date, counter);
    }
}

fn create_config() -> io::Result<BackConfig>{
    let path = loop {
        let mut tmp_path = String::new();
        println!("输入黑猴的安装路径：");
        io::stdin().read_line(&mut tmp_path)?;

        tmp_path = tmp_path.trim().to_string();
        
        if !Path::new(&tmp_path).is_dir() || tmp_path.is_empty() {
            println!("输入的路径不存在，请重新输入");
        }else {
            break tmp_path;
        }
    };
    let back_path = loop {
        println!("输入备份路径：");
        let mut back_path = String::new();
        io::stdin().read_line(&mut back_path)?;
        back_path = back_path.trim().to_string();    
        if !Path::new(&back_path).is_dir() || back_path.is_empty() {
            println!("输入的路径不存在，请重新输入");
        }else {
            break back_path;
        }
    };
    
    let config = BackConfig{
        black_myth_path : path,
        back_path : back_path,
    };
    let toml_string = toml::to_string(&config).expect("序列化失败");
    let mut file = File::create("black_myth_config.toml").expect("生成配置文件失败");
    file.write_all(toml_string.as_bytes()).expect("写入配置文件失败");
    println!("配置文件已生成");
    return Ok(config);
}
