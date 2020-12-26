use clap::Arg;

pub fn get_my_args() -> Vec<Arg<'static, 'static>> {
    vec![
        Arg::with_name("all_most")
            .short("a")
            .long("almost-all")
            .takes_value(false)
            .help("显示所有文件，除了 . 和 .."),
        Arg::with_name("inode")
            .short("i")
            .long("inode")
            .takes_value(false)
            .help("显示 inode"),
        Arg::with_name("human_readable")
            .short("h")
            .long("human-readable")
            .takes_value(false)
            .help("以合适单位显示文件大小"),
        Arg::with_name("long")
            .short("l")
            .takes_value(false)
            .help("显示详细信息"),
        Arg::with_name("ignore")
            .short("I")
            .long("ignore")
            .takes_value(true)
            .value_name("ignore_pattern")
            .help("忽略匹配的文件"),
        Arg::with_name("size")
            .short("s")
            .long("size")
            .takes_value(false)
            .help("显示文件字节数"),
        Arg::with_name("filename")
            .takes_value(true)
            .default_value(".")
            .value_name("filename"),
    ]
}
