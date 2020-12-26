use super::Flag;
use chrono::offset::TimeZone;
use chrono::Utc;
use chrono::{DateTime, Local};
use std::fs;
use std::os::linux::fs::MetadataExt;

pub fn show(meta: &fs::Metadata, flag: &Flag) {
    let mut ret = String::new();
    // show inode
    if flag.inode {
        ret.push_str(&format!("inode: {}\n", meta.st_ino()));
    }
    // show size
    if flag.size {
        if flag.human_readable {
            format_size(meta.st_size() as f64, &mut ret);
        } else {
            ret.push_str(&format!("大小：{} 字节\n", meta.st_size()));
        }
    }
    // show datetime and permissions
    if flag.long {
        format_datetime(meta, &mut ret);
        ret.push_str("权限：");
        ret.push_str(match meta.permissions().readonly() {
            true => "只读\n",
            false => "可写\n",
        });
    }

    print!("{}\n", ret);
}

fn format_size(size: f64, ret: &mut String) {
    let k = 1024.0;
    let m = 1024f64.powi(2);
    let g = 1024f64.powi(3);
    let t = 1024f64.powi(4);
    let p = 1024f64.powi(5);

    if size < k {
        ret.push_str(&format!("大小：{} 字节 \n", size));
    } else if k <= size && size < m {
        ret.push_str(&format!("大小： {:.2} Kib \n", size / k));
    } else if m <= size && size < g {
        ret.push_str(&format!("大小： {:.2} Mib \n", size / m));
    } else if g <= size && size < t {
        ret.push_str(&format!("大小： {:.2} Gib \n", size / g));
    } else if t <= size && size < p {
        ret.push_str(&format!("大小： {:.2} Tib \n", size / t));
    }
}

fn format_datetime(meta: &fs::Metadata, ret: &mut String) {
    let fmt = "%F %T %:z";
    ret.push_str(&format!(
        "最近访问：{}\n",
        &parse_timestamp(meta.st_atime(), fmt)
    ));
    ret.push_str(&format!(
        "最近更改：{}\n",
        &parse_timestamp(meta.st_mtime(), fmt)
    ));
    ret.push_str(&format!(
        "最近改动：{}\n",
        &parse_timestamp(meta.st_ctime(), fmt)
    ));
    let time = meta.created().unwrap();
    let datetime: DateTime<Local> = time.into();
    ret.push_str(&format!("创建时间：{}\n", datetime.format(fmt)));
}

fn parse_timestamp(time: i64, fmt: &str) -> String {
    let time: DateTime<Utc> = Utc.timestamp(time, 0);
    DateTime::<Local>::from(time).format(fmt).to_string()
}
