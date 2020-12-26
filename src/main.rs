mod app;
mod dir_info;
mod file_info;
mod flag;

use app::my_app;
use flag::Flag;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let arg_matches = my_app::get_my_app().get_matches();
    let filename = arg_matches.value_of("filename").unwrap();
    let meta = fs::metadata(filename)?;
    let flag = Flag::parse_args(&arg_matches);
    if meta.is_dir() {
        dir_info::show(&filename, &flag);
    } else {
        println!("文件名：{}", filename);
        file_info::show(&meta, &flag);
    }
    Ok(())
}
