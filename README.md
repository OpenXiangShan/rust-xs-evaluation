# **rust-xs-evaluation**
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[中文](./README.md) [English](./README_en.md)  

## Instruction
香山处理核性能测试 Rust 语言实现  
兼香山处理核微架构运行时 Rust 语言实现  

## Build
### 安装 Rust 环境
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
一路回车就行。  

测试是否安装成功：  
```bash
rustc --version
```
输出版本信息则表示安装成功。  

### 安装 Rust 交叉编译链（RISC-V）
```bash
rustup target add riscv64gc-unknown-none-elf
```

### 安装 binutils 工具集
```bash
cargo install cargo-binutils
rustup component add llvm-tools-preview
```
测试是否安装成功：  
```bash
rust-objdump --version
```

### 安装 just 工具
```bash
cargo install just
```

测试是否安装成功：  
```bash
just --version
```

### 编译香山可运行的二进制文件
```bash
cd rust-xs-evaluation
cd tests/am_cputests
just build
```
生成的二进制文件在 `rust-xs-evaluation/target/riscv64gc-unknown-none-elf/release` 目录下。  

## Quick Start
```bash
cd rust-xs-evaluation
cd tests/am_cputests
just run
```

## Test XiangShan
