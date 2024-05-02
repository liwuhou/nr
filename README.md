# Npm Runner

Just `nr [script]`, fastly excute any node's project any script.

```bash
Usage: nr <COMMAND>

Commands:
  run      Run any alias script or default
  ls       Show alias script list
  alias    Set alias to map script
  delete   Delete alias
  install  Use the right package manager to install
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Usage

First run, you just need to `run`.

```bash
nr run [alias] [script]
```

After above, you can use this [alias] to run script directly.

```bash
nr run [alias]
```

Even if alias is empty, just excute it without any thinking!

## Just install

`nr` will **use right package manager** to instlal dependencies.

```bash
nr install
# or
nr install -D [npm_package_name...]
```
