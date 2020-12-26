use super::my_args;
use clap::App;

pub fn get_my_app() -> App<'static, 'static> {
    let mut app = App::new("lsdir")
        .author("崔书豪 <568126480@qq.com>")
        .version("0.1.0")
        .about("lsdir 是一个命令行软件，用来列举目录内容或文件信息");

    for arg in my_args::get_my_args() {
        app = app.arg(arg);
    }

    app
}
