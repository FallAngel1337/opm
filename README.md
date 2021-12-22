# OPM (Oxidized Package Manager)

A `apt`-like system's package manager written in Rust for many operation systems distributions packages 
(See [supported-packages](#supported-packages). For more details check [supported-package-features](#supported-package-features)).
It provides either a higher level interface and a low-level one for those who want to build and inspect the packages.
This package manager aims to be universal/generic and simple. 

It's always good to remember that `IT IS NOT A WRAPPER`
for known package managers(apt, aptitude, dnf, yum, etc) neither a front-end for others (dpkg, rpm, etc).

**Note**: Most of the features and functionalities aren't done and/or stable yet. We're working to release a full working and stable version that can run on any operation system, including Microsoft Windows. For now just for debian-based linux distributions.

## Installation
### From releases (not-recommended):

You can find a binary release for you architecture in the [releases](https://github.com/0xc0ffeec0de/opm/releases) tab.

### From source:

You'll need to have [rust and cargo](https://www.rust-lang.org/tools/install) installed on your machine
```
$ git clone git@github.com:0xc0ffeec0de/opm.git
$ cd opm/
```
If you have [docker](https://www.docker.com/) installed you can run `docker build -t opm .` to run it inside a container.

**NOTE**: You need to define a environment variable called `PKG_FMT` to define on which package format `opm` will be working with. For now just `deb` is supported.

## Features:
  - [ ] Customizable
  - [ ] User-only installations
  - [ ] ...

## Supported Packages:
  - [X] deb
  - [ ] rpm
  - [ ] ...

## Usage
```
$ opm --help
Oxidized Package Manager v0.1
FallAngel <fallangel@protonmail.com>
A fully package manager made in Rust

USAGE:
    opm [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -l, --list       List all installed packages
    -V, --version    Prints version information

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    install    install a package
    remove     remove a package
    search     search for a package
    update     update opm's packages cache
```

For more details about the usage, check the [docs](docs/USAGE.md)


## Contribution
Check the [CONTRIBUTING.md](CONTRIBUTING.md).

## Help the project to continue
Maybe when this project will be complete ;)
