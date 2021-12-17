# OPM (Oxidized Package Manager)

A `apt`-like system's package manager written in Rust for many Linux distributions packages 
(See [supported-packages](#supported-packages). For more details check [supported-package-features](#supported-package-features))
It provides either a higher level interface and a low-level one for those who want to build and inspect the packages.
This package manager aims to be universal/generic and simple. It's always good to remember that `IT IS NOT A WRAPPER`
for knwon package managers(apt, aptitude, dnf, yum, etc) neither a front-end for others (dpkg, rpm, etc).

### Notes:
Most of the features and functionalities aren't made and/or stable yet.
We're working to release a full working and satable version that can run on any operation system.
Also we're only installing for now binary packages, in the future will support source packages.

### BEFORE INSTALLING!!!
This can sound a little weird, but you need a package manager to install a package manager.
You need to install the `libsqlite3-dev` first. We'll gonna get rid of that in the future
because it needs be a standalone application.

## Instation
* From releases

  You can find a binary release for you architecture in the [releases](https://github.com) tab.

* From source

  You'll need to have [rust and cargo](https://www.rust-lang.org/tools/install) installed on your machine
  ```
  $ git clone git@github.com:0xc0ffeec0de/opm.git
  $ cd opm/
  $ cargo build --release
  ```

### Setup
Before using you need to setup the package manager only.
This step you'll need to it only once.
For setup use: `$ opm setup`

### Features:
- [ ] Customizable
- [ ] User-only installations

### Supported Packages:
  - [X] `.deb`
  - [ ] `.rpm`

### Usage
```
$ opm --help
Usage: opm <[install|update|remove]> [options] <package_name>
```

#### Installing a package
```
$ sudo opm install python3

# Installing for YOUR current user
$ opm install --user python3
```

Also you can install directly
E.g:
```
$ opm install <package>.deb
```

* Add more usages

### Suported Package Features
  #### Debian Packages:
    - [X] Binary package installation
    - [ ] Source package installation
    - [X] Handles dependencies
    - [ ] Running pre/post install/remove scripts