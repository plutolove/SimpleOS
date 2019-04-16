## SimpleOS
根据blog_os实现一个简单的系统

#### 依赖
```bash
curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly # 安装nightly工具链
cargo install cargo-xbuild
cargo install bootimage --version "^0.7.1"
rustup component add llvm-tools-preview
```

#### 编译运行
运行需要预先安装`qemu`
```bash
bootimage build # 编译
bootimage run   # 运行
```