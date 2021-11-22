# RPM (Rusty Package Manager)

A `apt`-like package manager written in Rust for many Linux distributions (Check [supported distros](#supported-distros) section)
It provides either a higher level interface and a low-level one for those who want to build and inspect the packages.

### Features:
- [ ] IT DOES NOT BREAK THE SYSTEM (yet)
- [ ] Customizable
- [ ] Debian's packages compatibility
- [ ] RPM's packages compatibility

### Todo:
- [ ] Organize the project files

#### Debian(-based) System(s)
- [ ] Install debian's binary packages
    - [ ] Handle dependencies
    - [ ] Handle package versioning dependencies
    - [ ] Handle install/removal errors
    - [ ] Execute pre/pos installing/removing scripts
    - [ ] Hability to create(build) packages
    - [ ] Inspect packages 

- [ ] Install debian's source packages
    - [ ]…
  
  //  sudo apt install libsqlite3-dev
### Usage
```smalltalk
$ rpm --help
Usage: rmp <[install|update|remove]> [options] <package_name>
```
```smalltalk
# E.g.: Installing python3 (default way to use)
$ sudo rpm install python3

# Installing for YOUR current user
$ rmp install python3
Do you want to install only for your user? [Yes/No]
-> Yes
...

# If you try to install a package that breaks the system:
$ sudo rpm install steam
Do you REALLY want to continue? If so, type "Do as I say!" to confirm
-> Do as I say!
Your by your own now... good luck
```
