use clap::{Arg, App, SubCommand};
use opm::repos;
use std::process;

fn main() {
	let mut config = repos::setup().unwrap_or_else(|err| {
		eprintln!("Could not setup the package manager due {}", err);
		repos::roll_back();
		process::exit(1);
	});

    let matches = App::new("Oxidized Package Manager")
				.version("v0.1")
				.author("FallAngel <fallangel@protonmail.com>")
				.about("A package manager fully written in Rust")
				.arg(Arg::with_name("list")
					.short("l")
					.long("list")
					.takes_value(false)
					.help("List all installed packages")
				)
				.subcommands( vec![
					SubCommand::with_name("install")
						.about("install a package")
						.arg(Arg::with_name("package")
						.takes_value(true)
						.index(1)
						.required(true))
						.help("Install a package by the name\nUsage: opm install <package>"),
					SubCommand::with_name("update")
						.about("update opm's packages cache")
						.help("Update's opm cache\nUsage: opm update"),
					SubCommand::with_name("remove")
						.about("remove a package")
						.help("Remove a package by the name\nUsage: opm remove <package> ")
						.arg(Arg::with_name("package")
						.takes_value(true)
						.index(1)
						.required(true))
						.arg(Arg::with_name("purge")
							.required(false)
							.takes_value(false)
							.long("purge")
							.help("Remove every file related to the package")),
					SubCommand::with_name("search")
						.about("search for a package")
						.help("Search for a package in the cache")
						.arg(Arg::with_name("package")
							.takes_value(true)
							.index(1)
							.required(true)),
					SubCommand::with_name("clear")
						.about("Clear OPM's cache")
						.help("Clear OPM's cache")
				])
				.get_matches();

	match matches.occurrences_of("list") {
		0 => (),
		1 => repos::list_installed(&config),
		_ => println!("Invalid argument")
	};

    if let Some(package) = matches.subcommand_matches("install") {
        repos::install(&mut config, package.value_of("package").unwrap()).unwrap_or_else(|err| {
            println!("Got an error during installation :: {}", err);
            process::exit(1);
        });
    }

    if matches.subcommand_matches("update").is_some() {
        repos::update(&mut config).unwrap_or_else(|err| {
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
    };
	
    if let Some(package) = matches.subcommand_matches("search") {
		let pkg =  package.value_of("package").unwrap();
		println!("Searching for {} ...", package.value_of("package").unwrap());
		repos::search(&mut config, pkg).unwrap_or_else(|err| {
			eprintln!("Failed to search for {} due {}", pkg, err);
			process::exit(1);
		});
    };

    if matches.subcommand_matches("clear").is_some() {
		repos::clear(&config).unwrap_or_else(|err| {
			eprintln!("Failed to clear cache due {}", err);
			process::exit(1);
		});
    };
}