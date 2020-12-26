#![allow(dead_code)]
use clap::ArgMatches;

pub struct Flag {
    pub all_most: bool,
    pub inode: bool,
    pub human_readable: bool,
    pub long: bool,
    pub ignore_pattern: String,
    pub size: bool,
}

impl Flag {
    pub fn parse_args(arg_matches: &ArgMatches) -> Flag {
        let mut flag = Flag {
            all_most: arg_matches.is_present("all_most"),
            inode: arg_matches.is_present("inode"),
            human_readable: arg_matches.is_present("human_readable"),
            long: arg_matches.is_present("long"),
            ignore_pattern: arg_matches.value_of("ignore").unwrap_or(" ").to_string(),
            size: arg_matches.is_present("size"),
        };
        if flag.long {
            flag.inode = true;
            flag.size = true;
        }
        flag
    }
}
