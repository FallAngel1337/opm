use clap::{Arg, App, SubCommand};
use rpm::repos;
use std::process;

fn main() {
    let matches = App::new("Rusty Package Manager")
				.version("v0.1")
				.author("FallAngel <fallangel@protonmail.com>")
				.about("A fully package manager made in Rust")
				.subcommands( vec![
					SubCommand::with_name("install")
						.about("install a package")
						.arg(Arg::with_name("package")
						.takes_value(true)
						.index(1)
						.required(true))
						.help("Install a package by the name\nUsage: rpm install <package>"),
					SubCommand::with_name("update")
						.about("update rpm's packages cache")
						.help("Update's rpm cache\nUsage: rpm update"),
					SubCommand::with_name("remove")
						.about("remove a package")
						.help("Remove a package by the name\nUsage: rpm remove <package> ")
						.arg(Arg::with_name("package")
						.takes_value(true)
						.index(1)
						.required(true))
						.arg(Arg::with_name("purge")
							.required(false)
							.takes_value(false)
							.long("purge")
							.help("Remove every file related to the package"))
				])
				.get_matches();

    if let Some(package) = matches.subcommand_matches("install") {
        repos::install(package.value_of("package").unwrap()).unwrap_or_else(|err| {
            println!("Got an error during installation :: {}", err);
            process::exit(1);
        });
    }

    if let Some(_) = matches.subcommand_matches("update") {
        repos::update().unwrap_or_else(|err| {
			println!("Got and error during update :: {}", err);
			process::exit(1);
		})
    }

    if let Some(rm) = matches.subcommand_matches("remove") {
		println!("Removing ... {}", rm.value_of("package").unwrap());
        println!("Remove is not currently working");
		if rm.is_present("purge") {
			println!("Purgin 'em all HAHAHA");
		}
		process::exit(1);
    }
}