# About the OPM's config file

When you run OPM for the firts time it'll create a configuration file at `/opt/opm/<pkg>/config.json`

**NOTE:** `pkg` on that case will be your system's default package format.

## Why do we moved from `$HOME` to `/opt/`
Well, thats kinda simple. When you run with `sudo` the `HOME` variable will change from the programs perspective, and may can cause unexpected results.
By moving to a fixed place, this will not happen anymore. The downside of it, is that you'll always need `sudo`. But we're working on the user-only installation instead of a system-wide installation

## config.json
```json
{
    "os_info":{
        "os":{
            "Linux":"Debian"
        },
        "arch":"Amd64",
        "previous_db":"/var/lib/dpkg/status",
        "default_package_format":"Deb",
        "install_dir":"/opt/opm/deb"
    },
    "cache":"/opt/opm/deb/cache/pkg",
    "rls":"/opt/opm/deb/cache/rls",
    "archive":"/opt/opm/deb/archive",
    "info":"/opt/opm/deb/info",
    "tmp":"/opt/opm/deb/tmp",
    "db":"/opt/opm/deb/db",
    "use_pre_existing_cache":false,
    "use_pre_existing_db":false
}
```
Here is an example of a configuration file on a Debian machine.
OPM will try to figure out the OS you're running (this need some improvements) and will write those to the file.
In case of a wrong guess, you should modify that by hand.
