## rust 安装
教程视频  
离线安装  https://forge.rust-lang.org/infra/other-installation-methods.html#standalone  
Standalone installers --> x86_64-pc-windows-gnu --> msi  
ource code --> tar.xz

双击 rust-1.83.0-x86_64-pc-windows-gnu.msi --> Adcanced --> 设置安装路径

## 创建新项目
cargo new project01

## 构建和运行cargo项目
cargo build
cargo build --release       为了发布而构建，编译时会进行优化以确保代码在运行时更快。 可执行文件目录：target/release/
cargo run                   编译代码生成可执行文件 + 执行可执行文件
cargo check                 检查代码，确保可以通过编译，但是不会生成可执行文件


## 安装依赖
网址：https://crates.io/crates
示例：cargo add rand