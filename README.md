# Npm Runner

~~"sh\*t, I excue `npm start` script in vue-cli project again!"~~
<br/>
Just `nr [script]`, fastly excute any node's project any script.

🌏[中文文档](./README_zh.md)

![Example](./docs/example.svg)

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

## Feature 🚀

### Npm Run

Firstly run, you just need to `run`. `nr` **will use right package manage** to run the npm's script.

```bash
nr run [alias] [script]
```

After above, you can use this [alias] to run script directly.

```bash
nr run [alias]
```

Even if alias is empty, just excute it without any thinking!

> [alias] is empty or `-`, will run the default alias.

### Npm Install

`nr` will **use right package manager** to instlal dependencies.

```bash
nr install
# npm install
# yarn install
# pnpm install
```

```bash
nr install -D [npm_package_name...]
# npm install -D ...
# yarn add -D ...
# pnpm add -D ...
```

## Installtion 🏋️‍♂️

### Using Cargo 🦀 (Linux/MacOS/Windows)

```bash
cargo add nr
```

### Using a release binary (Linux/MacOS/Windows)

- Download the [latest release binary](https://github.com/liwuhou/nr/releases) for your system
- Make it available globally on PATH environment variable
- Enjoy!

### Homebrew 🍺 (MacOS/Linux)

```bash
brew tap liwuhou/formulae

brew install liwuhou/formulae/nr
```

### Scoop (Windows)

Comming soon
