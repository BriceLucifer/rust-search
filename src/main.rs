use rocksdb::DB;
use std::path::Path;
use std::error::Error;
use walkdir::WalkDir;
use clap::Parser;
use colored::Colorize;


#[derive(Parser, Debug)]
#[command(author="fb8py", version, about, long_about = None)]

struct Args {
    /// 这是寻找文件名字
    #[arg(short, long)]
    name: String,

    /// 请输入路径
    #[arg(short, long)]
    path: String,

    /// 这是一个debug选项
    #[arg(short, long)]
    debug: bool,

    /// 这是一个带默认参数的count
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}
fn main() -> Result<(), Box<dyn Error>> {

    let args = Args::parse();
    // 打开一个 RocksDB 数据库，如果数据库不存在则创建它
    let path = Path::new("file_index_db");
    let dir = WalkDir::new(args.path);
    let db = DB::open_default(path)?;
    
    // 循环添加
    for entry in dir.into_iter().flat_map(|e| e.ok()){
        let filename = entry.file_name().to_str().unwrap(); 
        let filepath = entry.path().to_str().unwrap();
        
        // 测试打印 看看路径和文件是否准确
        //println!("{} -- {}",filename,filepath);    

        let _ = db.put(filename.as_bytes(),filepath.as_bytes());
    }

    //检索并打印一个文件路径 -- test 检查
    // let file_name = "Rocker_DB.exe";
    // match db.get(file_name.as_bytes())? {
    //     Some(value) => println!("Found path for {} -> {}", file_name, String::from_utf8(value)?),
    //     None => println!("File not found: {}", file_name),
    // }

    let file_name = args.name;

    match db.get(file_name.as_bytes())? {
        Some(value) => println!("Found path for {} -> {}", file_name.blue().bold(), String::from_utf8(value)?),
        None => println!("File not found: {}", file_name.red().bold()),
    }

    // 测试而已看看测试参数
    println!("{}",args.count);
    // 返回值
    Ok(())
}


