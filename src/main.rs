use std::env;

const FULL:     &'static str = include_str!("../full.txt");

const SESSIONS: &'static str = include_str!("../sessions.txt");
const WINDOWS:  &'static str = include_str!("../windows.txt");
const PANES:    &'static str = include_str!("../panes.txt");
const COPYMODE: &'static str = include_str!("../copymode.txt");
const MISC:     &'static str = include_str!("../misc.txt");
const HELP:     &'static str = include_str!("../help.txt");

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => usage(),
        2 => match args[1].as_str() {
            "full"      => println!("{FULL}"),
            "sessions"  => println!("{SESSIONS}"),
            "windows"   => println!("{WINDOWS}"),
            "panes"     => println!("{PANES}"),
            "copymode"  => println!("{COPYMODE}"),
            "misc"      => println!("{MISC}"),
            "help"      => println!("{HELP}"),
            &_          => usage(),
        },
        _ => usage(),
    }
}

fn usage() {
    println!("\n\
            Usage:\n\
            \n\
            tmux_help <category>\n\
            \n\
            categories: sessions, windows, panes, copymode, misc, help.\n"
    );
}
