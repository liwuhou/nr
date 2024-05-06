# Npm Runner

~~“喵的，我怎么又在 vue-cli 目录里跑 `npm start`!”~~
<br/>
使用 `nr`，让你无脑使用 `nr [script]`，快速执行 node 项目中的 npm scripts。

🌎[英文文档](./README.md)

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

## 功能 🚀

### Npm Run

只需在第一次执行的时候，在 npm 工程内，直接使用 `nr run [alias]`，会出现当前工程目录的脚本列表让你选择将要绑定的命令，工具会自动使用正确的包管理器去运行。

```bash
nr run [alias] [script]
```

后续执行同样的 `alias` 时，会自动选取上次选择的命令直接运行。

```bash
nr run [alias]
```

同时，`alias` 也可以为空，少输入一个参数，整个过程无须任何心智负担，可谓十分地无脑。

> `alias` 为空传入时，实际等价于 `-`，后续更改和删除可以传 `-` 操作 `alias`

### Npm Install

`nr` 将自动**选择当前目录里最正确的包管理器**去安装依赖（缺省是 `npm`）

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

## 安装 🏋️‍♂️

### Using Cargo 🦀 (Linux/MacOS/Windows)

```bash
cargo add nr
```

### Using a release binary (Linux/MacOS/Windows)

- 前往 [工程 release 页](https://github.com/liwuhou/nr/releases) 下载所属你系统的应用
- 确保将命令行工具存放在你 `PATH` 环境目录下
- 开始无脑使用

### Homebrew 🍺 (MacOS/Linux)

```bash
brew tap liwuhou/formulae

brew install liwuhou/formulae/nr
```

### Scoop (Windows)

Comming soon
