# dsc - DragonOS Raw Syscall Binding

This is a raw syscall binding for DragonOS. It is not meant to be used directly, but rather as a dependency for other crates.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
dsc = { git = "https://github.com/DragonOS-Community/dsc.git" }
```

## Development

如果您正在开发dsc，请您在引入dsc的库的`Cargo.toml`中添加如下内容，而不是使用上述的代码：

```toml
[dependencies]
dsc = { path = "您本地存放dsc的源代码的路径" }
```

## How to build

```bash
ARCH=x86_64 && cargo build -Zbuild-std --release --target src/platform/$ARCH/target.json 
```

## How to build docs

```bash
ARCH=x86_64 && cargo doc -Zbuild-std --release --target src/platform/$ARCH/target.json 
```

## What is DragonOS?

DragonOS is an opensource operating system developed for the server field. 
Its kernel and user mode environment are developed from scratch, and provides Linux compatibility.

- [DragonOS Website](https://dragonos.org)
- [DragonOS Github](https://github.com/DragonOS-Community)

## License

Licensed under 
  * MIT license ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT)
