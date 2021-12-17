# OPM (Oxidized Package Manager)

A `apt`-like system's package manager written in Rust for many operation systems distributions packages 
(See [supported-packages](#supported-packages). For more details check [supported-package-features](#supported-package-features)
It provides either a higher level interface and a low-level one for those who want to build and inspect the packages.
This package manager aims to be universal/generic and simple. 

It's always good to remember that `IT IS NOT A WRAPPER`
for known package managers(apt, aptitude, dnf, yum, etc) neither a front-end for others (dpkg, rpm, etc).

**Note**: Most of the features and functionalities aren't done and/or stable yet. We're working to release a full working and satable version that can run on any operation system, including Microsoft Windows. For now just for debian-based linux distributions.

### Before installing
This can sound a little weird, but you need a package manager to install a package manager.
You need to install the `libsqlite3-dev` first. We'll gonna get rid of that in the future
because it needs be a standalone application.

## Installation
### From releases:

You can find a binary release for you architecture in the [releases](https://github.com) tab.

### From source:

You'll need to have [rust and cargo](https://www.rust-lang.org/tools/install) installed on your machine
```
$ git clone git@github.com:0xc0ffeec0de/opm.git
$ cd opm/
$ cargo build --release
$ PKG_FMT=<fmt> target/release/opm setup
```
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
Usage: opm <[install|update|remove]> [options] <package_name>
```

### Installing a package
```
$ sudo opm install python3

# Installing for YOUR current user
$ opm install --user python3
```

Also you can install directly.
E.g:
```
$ opm install <package>.deb
```

### Supported Package Features
  #### Debian Packages:
  - [X] Binary package installation
  - [ ] Source package installation
  - [X] Handles dependencies
  - [ ] Running pre/post install/remove scripts
  - [ ] ...

## Contribution
Check the [contribution guidelines](CONTRIBUTING.md).

If you want to contribute with the project you can check [here](https://github.com/0xc0ffeec0de) for more details about the project structure, implementation details program logic, well-known bugs, etc to give you a better understanding of whats happening under the hood.

Also, fell free to send a pull request or open an issue. If you miss a feature that you want that opm would have, you wan open an issue and then we'll evaluate if it's possible or not.

## Help the project to continue
Maybe when this project will be complete ;)
