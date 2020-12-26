use super::Flag;
use fs::DirEntry;
use regex::Regex;
use std::fs;

pub fn show(dirname: &str, flag: &Flag) {
    let re = Regex::new(&flag.ignore_pattern).unwrap();
    for entry in fs::read_dir(dirname).unwrap() {
        let entry = entry.unwrap();
        let filename = &*entry.file_name().into_string().unwrap();

        if filename.starts_with('.') && !flag.all_most {
            continue;
        }
        if re.is_match(filename) {
            continue;
        }
        priln_entry(&entry);
        super::file_info::show(&entry.metadata().unwrap(), flag);
    }
}

fn priln_entry(entry: &DirEntry) {
    let mut s = String::new();
    let file_type = entry.file_type().unwrap();
    if file_type.is_dir() {
        s.push_str(&format!(
            "目录：{}",
            entry.file_name().into_string().unwrap()
        ));
    } else if file_type.is_file() {
        s.push_str(&format!(
            "文件：{}",
            entry.file_name().into_string().unwrap()
        ));
    } else if file_type.is_symlink() {
        s.push_str(&format!(
            "链接：{}",
            entry.file_name().into_string().unwrap()
        ));
    }
    println!("{}", &s);
}
