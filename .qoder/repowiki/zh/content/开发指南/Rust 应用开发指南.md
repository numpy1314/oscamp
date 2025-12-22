# Rust 应用开发指南

<cite>
**本文档中引用的文件**   
- [helloworld/Cargo.toml](file://arceos/examples/helloworld/Cargo.toml)
- [helloworld/src/main.rs](file://arceos/examples/helloworld/src/main.rs)
- [u_1_0/Cargo.toml](file://arceos/tour/u_1_0/Cargo.toml)
- [u_1_0/src/main.rs](file://arceos/tour/u_1_0/src/main.rs)
- [axstd/Cargo.toml](file://arceos/ulib/axstd/Cargo.toml)
- [httpclient/Cargo.toml](file://arceos/examples/httpclient/Cargo.toml)
- [httpclient/src/main.rs](file://arceos/examples/httpclient/src/main.rs)
- [httpserver/Cargo.toml](file://arceos/examples/httpserver/Cargo.toml)
- [httpserver/src/main.rs](file://arceos/examples/httpserver/src/main.rs)
- [httpserver-c/features.txt](file://arceos/examples/httpserver-c/features.txt)
- [httpclient-c/features.txt](file://arceos/examples/httpclient-c/features.txt)
- [shell/Cargo.toml](file://arceos/examples/shell/Cargo.toml)
- [shell/src/main.rs](file://arceos/examples/shell/src/main.rs)
- [Makefile](file://arceos/Makefile)
</cite>

## 目录
1. [项目初始化与配置](#项目初始化与配置)
2. [核心属性与入口点](#核心属性与入口点)
3. [axstd 功能特性](#axstd-功能特性)
4. [代码示例分析](#代码示例分析)
5. [构建与运行](#构建与运行)
6. [常见错误与调试](#常见错误与调试)

## 项目初始化与配置

在 oscamp 平台上创建基于 `no_std` 的 Rust 用户程序，首先需要正确配置 `Cargo.toml` 文件。项目应依赖 `axstd` 库，该库为 ArceOS 提供了类似 Rust 标准库的接口。在 `Cargo.toml` 中，`axstd` 依赖被声明为可选（`optional = true`），以便在不同环境下灵活启用。

```toml
[dependencies]
axstd = { workspace = true, optional = true }
```

对于需要特定功能的应用程序，如网络或文件系统，可以在依赖中直接指定所需功能。例如，HTTP 客户端示例明确启用了 `net` 功能：

```toml
[dependencies]
axstd = { workspace = true, features = ["net"], optional = true }
```

而更复杂的 HTTP 服务器则需要内存分配、多任务和网络功能：

```toml
[dependencies]
axstd = { workspace = true, features = ["alloc", "multitask", "net"], optional = true }
```

**本节来源**
- [helloworld/Cargo.toml](file://arceos/examples/helloworld/Cargo.toml#L1-L11)
- [httpclient/Cargo.toml](file://arceos/examples/httpclient/Cargo.toml#L1-L15)
- [httpserver/Cargo.toml](file://arceos/examples/httpserver/Cargo.toml#L1-L11)

## 核心属性与入口点

在 ArceOS 的 `no_std` 环境中，Rust 程序必须使用特定的属性来适配操作系统。`#![cfg_attr(feature = "axstd", no_std)]` 属性确保当启用 `axstd` 功能时，程序不链接 Rust 标准库，而是使用 ArceOS 提供的精简运行时。`#![cfg_attr(feature = "axstd", no_main)]` 属性告诉编译器不要寻找标准的 `main` 函数入口，因为操作系统的启动流程会有所不同。

为了确保链接器能正确识别程序的入口函数，必须使用 `#[cfg_attr(feature = "axstd", no_mangle)]` 属性。这会阻止编译器对 `main` 函数进行名称修饰（name mangling），使其在编译后的二进制文件中保持为简单的 `main` 符号，从而被操作系统的加载器找到。

```rust
#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    // 程序逻辑
}
```

**本节来源**
- [helloworld/src/main.rs](file://arceos/examples/helloworld/src/main.rs#L1-L10)
- [u_1_0/src/main.rs](file://arceos/tour/u_1_0/src/main.rs#L1-L10)

## axstd 功能特性

`axstd` 库通过 Cargo 功能（features）机制提供模块化的功能支持。开发者可以根据应用需求选择性地启用特定功能，以保持程序的轻量化。这些功能在 `axstd/Cargo.toml` 文件中定义，主要分为以下几类：

- **内存管理**：`alloc` 功能启用动态内存分配，`paging` 功能启用分页内存管理。
- **多任务**：`multitask` 功能启用多线程和任务调度支持。
- **文件系统**：`fs` 功能启用文件 I/O 操作。
- **网络**：`net` 功能启用 TCP/UDP 套接字等网络编程接口。
- **设备驱动**：如 `driver-ixgbe` 用于特定网卡驱动。

启用这些功能不仅会编译相应的 `axstd` 代码，还会激活底层 ArceOS 模块（如 `arceos_api`）的对应功能。例如，启用 `net` 功能会同时激活 `arceos_api/net` 和 `axfeat/net`。

**本节来源**
- [axstd/Cargo.toml](file://arceos/ulib/axstd/Cargo.toml#L19-L75)

## 代码示例分析

### Hello World 示例

最简单的 `helloworld` 示例展示了基本的配置和输出。它通过条件编译使用 `axstd::println!` 宏来输出文本。`#[cfg(feature = "axstd")]` 确保只有在启用 `axstd` 时才导入此模块。

```rust
#[cfg(feature = "axstd")]
use axstd::println;

fn main() {
    println!("Hello, world!");
}
```

### 网络应用示例

`httpclient` 示例展示了如何使用 `axstd` 的网络功能。它通过 `TcpStream::connect()` 建立连接，并使用 `write_all()` 和 `read()` 进行数据交换。代码中还使用了 `ToSocketAddrs` trait 来解析地址。

```rust
use std::net::{TcpStream, ToSocketAddrs};

fn main() {
    let mut stream = TcpStream::connect(DEST)?;
    stream.write_all(REQUEST.as_bytes())?;
    let mut buf = [0; 2048];
    let n = stream.read(&mut buf)?;
    // 处理响应
}
```

`httpserver` 示例则更复杂，它使用 `TcpListener` 监听端口，并利用 `std::thread::spawn()` 为每个连接创建新线程，这要求同时启用 `alloc`、`multitask` 和 `net` 功能。

```rust
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind((LOCAL_IP, LOCAL_PORT))?;
    for stream in listener.incoming() {
        thread::spawn(move || {
            // 处理单个连接
        });
    }
}
```

### 文件系统与 Shell 示例

`shell` 示例通过 `use-ramfs` 功能启用了文件系统支持。它使用 `std::env::current_dir()` 获取当前目录，并通过 `std::io::stdin()` 和 `stdout()` 实现交互式命令行。

**本节来源**
- [helloworld/src/main.rs](file://arceos/examples/helloworld/src/main.rs#L4-L10)
- [httpclient/src/main.rs](file://arceos/examples/httpclient/src/main.rs#L8-L41)
- [httpserver/src/main.rs](file://arceos/examples/httpserver/src/main.rs#L16-L97)
- [shell/src/main.rs](file://arceos/examples/shell/src/main.rs#L26-L86)

## 构建与运行

构建和运行 ArceOS 用户程序主要通过顶层的 `Makefile` 完成。核心命令是：

```bash
make -C /path/to/arceos A=$(pwd) ARCH=aarch64 run
```

- `-C /path/to/arceos`: 切换到 ArceOS 项目根目录执行 Makefile。
- `A=$(pwd)`: 指定当前目录为要构建的应用程序路径。
- `ARCH=aarch64`: 指定目标架构（可选 `x86_64`, `riscv64`, `aarch64`）。
- `run`: 执行构建并启动 QEMU 模拟器。

其他常用参数包括：
- `MODE=debug`: 指定构建模式（`release` 或 `debug`）。
- `FEATURES="net fs"`: 为 ArceOS 内核启用特定功能。
- `APP_FEATURES="dns"`: 为应用程序本身启用 Cargo 功能。

对于 C 语言编写的程序，可以通过 `features.txt` 文件来指定所需的功能，例如 `httpserver-c` 示例中的 `features.txt` 包含 `alloc`、`paging` 和 `net`。

**本节来源**
- [Makefile](file://arceos/Makefile#L1-L239)
- [httpserver-c/features.txt](file://arceos/examples/httpserver-c/features.txt#L1-L4)
- [httpclient-c/features.txt](file://arceos/examples/httpclient-c/features.txt#L1-L4)

## 常见错误与调试

最常见的编译错误是由于未启用必要的功能。例如，如果在未启用 `net` 功能的情况下调用 `TcpStream::connect()`，编译器将无法找到该方法的实现。解决方法是在 `Cargo.toml` 的 `axstd` 依赖中添加 `features = ["net"]`，或在 `Makefile` 调用中通过 `APP_FEATURES` 参数指定。

另一个常见问题是链接错误，通常是因为 `main` 函数没有正确标记 `#[no_mangle]`，导致链接器找不到入口点。确保在 `main` 函数上使用 `#[cfg_attr(feature = "axstd", no_mangle)]` 是解决此问题的关键。

调试时，可以利用 `Makefile` 提供的 `debug` 目标，它会启动 QEMU 并监听 GDB 调试连接，便于进行断点调试和内存检查。

**本节来源**
- [httpserver/Cargo.toml](file://arceos/examples/httpserver/Cargo.toml#L10)
- [helloworld/src/main.rs](file://arceos/examples/helloworld/src/main.rs#L7)