# **rust-xs-evaluation**
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[中文](./README.md) [English](./README_en.md)  

## Instruction
香山处理核性能测试 `Rust` 语言实现  
兼香山处理核 `Rust` 嵌入式支持库实现   

## Installation
```bash
git clone https://github.com/RISCVERS/rust-xs-evaluation
cd rust-xs-evaluation
git submodule update --init --recursive
```

## Build
环境配置请看这里：[build](./doc/build.md)   

## Quick Start
运行 `Rust` 版本的 `cputest`：  
```bash
cd rust-xs-evaluation
cd tests/am_cputests
just run
```

## Test XiangShan
+ 重写 `AM` 中的 `cputest`：[am_cputests](./tests/am_cputests)  
+ 重写 `AM` 中的 `cachetest`：[am_cachetests](./tests/am_cachetests)  
+ 基准测试：`TODO`

## XianShan Rust Runtime
现在已经将香山的微架构运行时抽出来成为一个单独的 crate，详细请看[xs-rt](./xs-rt)  
通过这个包现在只需要很少量的代码就可以搭建香山的 Rust 运行时环境。  

## Rust Embedded-Hal Protocol Implementation
`Rust` 嵌入式生态有个 `embedded-hal` 标准，考虑遵循这个标准来写香山的 `Rust` 嵌入式支持库。  
该项目下的 `xs-hal` 目录就是对此的尝试。  

## Embedded Rust
有关 `Rust` 嵌入式生态和 `embedded-hal` 标准可以参考这里：  
+ [Rust嵌入式生态](./img/rust_embedded_zoology.png)  
+ [embedded-hal](https://github.com/rust-embedded/embedded-hal)  



