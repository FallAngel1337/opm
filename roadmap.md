# RPM Roadmap
   
### Tasks
- [ ] [Improve the UI/UX](#ui_ux)
- [ ] [Run the pre/post install/remove](#scripts)
- [ ] [Verify package's integrity](#integrity)
- [ ] Check the package priority
- [ ] [Packages installation](#packages-installation)
- [x] [Packages update](#packages-update)
   - [ ] Improve the download speed (using async)
- [ ] [Packages removal](#packages-removal)
- [x] [Packages listing](#packages-listing)
   - [x] Make the querying faster


### Packages installation
```
$ rpm install [options] <package_name>
```
Note: It requires sudo to install for the system or `--user` option for just the user.

About how we should install a (debian) package.

* Installation process

1.  Check if the package is already installed
2.  Query for the package in the cache (**~/.rpm/cache**)
3.  If the package is in the cache install it
4.  Install the dependencies if so
5.  Run the **pre/post** install scripts if so

### Packages update
```
$ rpm update
```
Will update the packages cache entry at `~/.opm/cache` by querying the default distro sources (`/etc/apt/sources.list`)

#### Packages removal
```
$ rpm remove [options] <package_name>
```

Query the database for the package and then remove it. With `-p / --purge` should remove all the files related to the package by using the **pre/post** remove scripts . If does not, remove manually. Also delete the database entry for that package.

### Packages listing
```
$ rpm --list
```

Query the database for all installed packages and list to the user. You can use the shortcut `-l`too.
