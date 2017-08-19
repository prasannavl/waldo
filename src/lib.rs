#[macro_use] extern crate clap;

pub mod windows;

use clap::{ErrorKind, App, Arg, SubCommand, ArgMatches};
use std::vec::Vec;

mod cmd;
use cmd::time;
use cmd::test;
use cmd::misc;

pub fn run() {
    let matches = App::new(crate_name!())
        .about("Ask Waldo. Waldo Do!")
        .version(crate_version!())
        .author(crate_authors!())
        .args(&get_args())
        .subcommands(get_sub_commands())
        .get_matches_safe();

    match matches {
        Ok(m) => dispatch(&m),
        Err(ref e) if
        e.kind == ErrorKind::HelpDisplayed ||
            e.kind == ErrorKind::VersionDisplayed
        => println!("{}", e.message),
        Err(ref e) => eprintln!("{}", e.message)
    }
}

fn dispatch(matches: &ArgMatches) {
    match matches.subcommand_name() {
        Some("hello") => println!("Hello world!"),
        Some("say") => println!("say command"),
        Some("time") => time::run(),
        Some("test") => test::run(),
        Some("display-off") => misc::display_off(),
        Some(_) => unreachable!(),
        None => println!("{}", matches.usage())
    }
}

fn get_args<'a>() -> Vec<Arg<'a, 'a>> {
    vec![
        Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .help("sets the level of verbosity")
    ]
}

fn get_sub_commands<'a>() -> Vec<App<'a, 'a>> {
    let mut v = vec![
        SubCommand::with_name("say")
            .about("say the given text"),
        SubCommand::with_name("hello")
            .about("say hello")
            .alias("say-hello"),
        SubCommand::with_name("time")
            .about("tell the time"),
        SubCommand::with_name("test")
            .about("run test command"),

    ];
    v.extend(get_platform_commands());
    v
}


#[cfg(windows)]
fn get_platform_commands<'a>() -> Vec<App<'a, 'a>> {
    vec![
        SubCommand::with_name("display-off")
            .about("turn the display off")
            .alias("doff")
    ]
}

#[cfg(linux)]
fn get_platform_commands<'a>() -> Vec<App<'a, 'a>>{
    vec![]
}
