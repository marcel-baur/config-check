# Config Check

A tool to check configuration files (`.properties` files only for now) for keys that do not appear in all files.
Needs a `config_checker.toml` file with a `files` array that contains the paths of the files that are to be verified:
```
files = ["one.properties", "two.properties"]
```

## Roadmap
* Improved print
* Add `.yaml` file support
