# RPM Roadmap
   
### Tasks
- [ ] [Improve the UI/UX](#ui_ux)
- [ ] [Run the pre/post install/remove](#scripts)
- [ ] [Verify integrity](#integrity)
   - [ ] [Pacakages integrity](#packages-integrity)
   - [ ] [Repository integrity](#repository-integrity)
- [ ] [Dependencie Handling](#dependencie-handling)
- [ ] [Packages installation](#packages-installation)
   - [ ] User-only installtion
- [ ] Package versioning checking
- [x] [Packages update](#packages-update)
   - [ ] Improve the download speed (using async)
- [ ] [Packages removal](#packages-removal)
- [x] [Packages listing](#packages-listing)
   - [x] Make the querying faster
- [ ] Pacakage Source
   - [X] Sources formats are different on ubuntu-bases from debian-based
- [ ] [Handle Edge Cases](#Handle-Edge-Cases)
- [ ] Add a rollback function (in case of CTRL+C or package installtion/remove failure)

### UI-UX
For now it's showing too much messages and most of then doesn't have meaning
from the final-user perspective, so we need to make better output messages

### Scripts
If the packages does have some pre or post install script it should be executed.
Maybe we can have a feature for malicious scripts execution and prevent'em to harm the user.
- Follow: https://wiki.debian.org/MaintainerScripts

### Integrity
   ### Packages Intergrity
   All packages have a signature filed, it is a hash that signs the package.
   It' should be verified, after installation to see if nothing had been corrupted.

   ### Repository Intergrity
   Most repositopries have a signature, so we sould verify it before downloading from
   it. Of course, enable the possibilty to force the download from untrusted repositories.

### Dependencie Handling
Most of the packages depends on others, so we need to handle that dependencies.
But not all dependencies are required, they can be recommended to enable a new feature of the package.
By default, install just the essential packages.

### Handle Edge Cases
`OPM` should work on all use cases, it cannot break itself, not work or even worst...
break the whole system.
E.g.: Not having `/var/libg/dpkg/status` on debian based distributions.
I know it's very rare or thats near impossible, but it can happen... `OPM` need to handle that.

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