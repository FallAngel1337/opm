</p>
<p align="center">
  </a>
    <img alt="Logo" src="docs/images/logo.png">
  </a>
</p>

# OPM (Oxidized Package Manager) 📦
</p>
<p align="center">
  </a>
    <img alt="GitHub" src="https://img.shields.io/github/license/0xc0ffeec0de/opm">
    <img alt="GitHub Workflow Status (branch)" src="https://img.shields.io/github/workflow/status/0xc0ffeec0de/opm/Rust/master">
    <img alt="GitHub issues" src="https://img.shields.io/github/issues/0xc0ffeec0de/opm">
    <img alt="GitHub release (latest SemVer)" src="https://img.shields.io/github/v/release/0xc0ffeec0de/opm">
  </a>
</p>


A `apt`-like systems package manager written in Rust for many operating systems packages.
(See [supported-packages](#supported-packages)).
It provides either a higher level interface and a low-level one for those who want to build and inspect the packages.
This package manager aims to be universal/generic and simple. 

It's always good to remember that `IT IS NOT A WRAPPER`
for known package managers(apt, aptitude, dnf, yum, etc) neither a front-end for others (dpkg, rpm, etc).

**Note**: Most of the features and functionalities aren't done and/or stable yet. We're working to release a full working and stable version that can run on any operation system, including Microsoft Windows. For now just for debian-based linux distributions.

## Installation
**NOTE:** Since OPM is not stable yet, it's not recommended to use it on your OS directly

### From releases:

You can find a binary release for you architecture in the [releases](https://github.com/0xc0ffeec0de/opm/releases) tab.

### From source:

You'll need to have [rust and cargo](https://www.rust-lang.org/tools/install) installed on your machine
```
$ git clone git@github.com:0xc0ffeec0de/opm.git
$ cd opm/
$ cargo build --release
$ sudo ./target/releases/opm
```
If you have [docker](https://www.docker.com/) installed you can build the image inside the repository.

## Supported Packages:
  - [X] deb
  - [ ] rpm
  - [ ] ...

## Usage
```
Oxidized Package Manager v1.5.1-beta
FallAngel <fallangel@protonmail.com>
A package manager fully written in Rust

USAGE:
    opm [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -l, --list       List all installed packages
    -V, --version    Prints version information

SUBCOMMANDS:
    clear      Clear OPM's cache
    help       Prints this message or the help of the given subcommand(s)
    install    Install a package
    remove     Remove a package
    search     Search for a package in the cache
    update     Update opm's packages cache
```

For more details about the usage, check the [docs](docs/USAGE.md)


## Contribution
Check the [CONTRIBUTING.md](CONTRIBUTING.md).

## Help the project to continue
Maybe when this project will be complete ;)
