# dsc - DragonOS Raw Syscall Binding

This is a raw syscall binding for DragonOS. It is not meant to be used directly, but rather as a dependency for other crates.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
dsc = { git = "https://github.com/DragonOS-Community/dsc.git" }
```

## 其他

如果您正在开发dsc，请您在引入dsc的库的`Cargo.toml`中添加如下内容，而不是使用上述的代码：

```toml
[dependencies]
dsc = { path = "您本地存放dsc的源代码的路径" }
```
