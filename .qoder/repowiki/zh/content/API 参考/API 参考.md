# API 参考

<cite>
**本文档中引用的文件**  
- [arceos_api/lib.rs](file://arceos/api/arceos_api/src/lib.rs)
- [arceos_posix_api/lib.rs](file://arceos/api/arceos_posix_api/src/lib.rs)
- [axstd/lib.rs](file://arceos/ulib/axstd/src/lib.rs)
- [axlibc/lib.rs](file://arceos/ulib/axlibc/src/lib.rs)
- [arceos_api/macros.rs](file://arceos/api/arceos_api/src/macros.rs)
- [arceos_posix_api/imp/mod.rs](file://arceos/api/arceos_posix_api/src/imp/mod.rs)
- [axstd/fs/mod.rs](file://arceos/ulib/axstd/src/fs/mod.rs)
- [axstd/net/mod.rs](file://arceos/ulib/axstd/src/net/mod.rs)
- [arceos_api/imp/mod.rs](file://arceos/api/arceos_api/src/imp/mod.rs)
- [arceos_posix_api/imp/io.rs](file://arceos/api/arceos_posix_api/src/imp/io.rs)
- [arceos_posix_api/imp/fs.rs](file://arceos/api/arceos_posix_api/src/imp/fs.rs)
- [arceos_posix_api/imp/net.rs](file://arceos/api/arceos_posix_api/src/imp/net.rs)
- [axlibc/io.rs](file://arceos/ulib/axlibc/src/io.rs)
- [axlibc/unistd.rs](file://arceos/ulib/axlibc/src/unistd.rs)
- [axlibc/net.rs](file://arceos/ulib/axlibc/src/net.rs)
- [axlibc/fs.rs](file://arceos/ulib/axlibc/src/fs.rs)
</cite>

## 目录
1. [ArceOS 原生 API (arceos_api)](#arceos-原生-api-arceos_api)
2. [POSIX 兼容 API (arceos_posix_api)](#posix-兼容-api-arceos_posix_api)
3. [Rust 用户库 (axstd)](#rust-用户库-axstd)
4. [C 用户库 (axlibc)](#c-用户库-axlibc)

## ArceOS 原生 API (arceos_api)

ArceOS 原生 API（arceos_api）为操作系统内部服务提供了直接的接口，允许内核模块和系统组件直接调用底层功能，而无需通过传统的系统调用机制。这些 API 是 ArceOS 操作系统架构的核心，提供了对系统资源的直接访问。

### 系统操作
`sys` 模块提供系统级操作功能。
- `ax_terminate()`: 关闭整个系统和所有 CPU。

### 时间操作
`time` 模块提供与时间相关的操作。
- `ax_monotonic_time()`: 返回自系统启动以来经过的时间。
- `ax_wall_time()`: 返回自纪元以来经过的时间（也称为实时）。

### 内存管理
`mem` 模块提供内存管理功能。
- `ax_alloc(layout)`: 在全局分配器中分配具有给定布局的连续内存块。如果分配失败，则返回 `None`。此函数是不安全的，需要用户手动管理缓冲区生命周期。
- `ax_dealloc(ptr, layout)`: 释放由 `ax_alloc` 分配的内存块。此函数是不安全的，需要用户手动管理缓冲区生命周期。
- `ax_alloc_coherent(layout)`: 分配满足直接内存访问（DMA）要求的**相干**内存。如果分配失败，则返回 `None`。此函数是不安全的。
- `ax_dealloc_coherent(dma, layout)`: 释放先前分配的相干内存。此函数是不安全的。

### 标准输入输出
`stdio` 模块提供标准输入和输出功能。
- `ax_console_read_byte()`: 从控制台读取一个字节，如果没有输入可用，则返回 `None`。
- `ax_console_write_bytes(buf)`: 将字节切片写入控制台，返回写入的字节数。
- `ax_console_write_fmt(args)`: 将格式化字符串写入控制台。

### 多线程管理
`task` 模块提供多线程管理功能。
- `ax_sleep_until(deadline)`: 当前任务将休眠，直到给定的截止时间。
- `ax_yield_now()`: 当前任务自愿放弃 CPU 时间，并切换到另一个就绪任务。
- `ax_exit(exit_code)`: 以给定的退出代码退出当前任务。
- `ax_current_task_id()`: 返回当前任务的 ID。
- `ax_spawn(f, name, stack_size)`: 使用给定的入口点和其他参数生成一个新任务。
- `ax_wait_for_exit(task)`: 等待给定任务退出，并返回其退出代码。
- `ax_set_current_priority(prio)`: 设置当前任务的优先级。
- `ax_wait_queue_wait(wq, until_condition, timeout)`: 阻塞当前任务并将其放入等待队列，直到给定条件变为真或给定的持续时间已过（如果指定）。
- `ax_wait_queue_wake(wq, count)`: 唤醒等待队列中的一个或多个任务。

### 文件系统操作
`fs` 模块提供文件系统操作功能。
- `ax_open_file(path, opts)`: 使用指定的选项打开相对于当前目录的路径处的文件。
- `ax_open_dir(path, opts)`: 使用指定的选项打开相对于当前目录的路径处的目录。
- `ax_read_file(file, buf)`: 从文件的当前位置读取，返回读取的字节数。
- `ax_read_file_at(file, offset, buf)`: 从文件的给定位置读取，返回读取的字节数。
- `ax_write_file(file, buf)`: 在文件的当前位置写入，返回写入的字节数。
- `ax_write_file_at(file, offset, buf)`: 在文件的给定位置写入，返回写入的字节数。
- `ax_truncate_file(file, size)`: 将文件截断到指定大小。
- `ax_flush_file(file)`: 刷新文件，将所有缓冲的数据写入底层设备。
- `ax_seek_file(file, pos)`: 将文件的游标设置为指定的偏移量。返回寻址后的新位置。
- `ax_file_attr(file)`: 返回文件的属性。
- `ax_read_dir(dir, dirents)`: 从当前位置开始读取目录条目到给定的缓冲区中，返回读取的条目数。
- `ax_create_dir(path)`: 在提供的路径处创建一个新的空目录。
- `ax_remove_dir(path)`: 删除一个空目录。
- `ax_remove_file(path)`: 从文件系统中删除一个文件。
- `ax_rename(old, new)`: 将文件或目录重命名为新名称。
- `ax_current_dir()`: 返回当前工作目录。
- `ax_set_current_dir(path)`: 将当前工作目录更改为指定路径。

### 网络操作
`net` 模块提供 TCP/UDP 通信的网络原语。
- `ax_tcp_socket()`: 创建一个新的 TCP 套接字。
- `ax_tcp_socket_addr(socket)`: 返回 TCP 套接字的本地地址和端口。
- `ax_tcp_peer_addr(socket)`: 返回 TCP 套接字的远程地址和端口。
- `ax_tcp_set_nonblocking(socket, nonblocking)`: 将此 TCP 套接字移入或移出非阻塞模式。
- `ax_tcp_connect(handle, addr)`: 将 TCP 套接字连接到给定的地址和端口。
- `ax_tcp_bind(socket, addr)`: 将 TCP 套接字绑定到给定的地址和端口。
- `ax_tcp_listen(socket, _backlog)`: 在绑定的地址和端口上开始监听。
- `ax_tcp_accept(socket)`: 接受 TCP 套接字上的新连接。
- `ax_tcp_send(socket, buf)`: 在 TCP 套接字上传输给定缓冲区中的数据。
- `ax_tcp_recv(socket, buf)`: 在 TCP 套接字上接收数据，并将其存储在给定的缓冲区中。
- `ax_tcp_poll(socket)`: 返回 TCP 套接字是否可读或可写。
- `ax_tcp_shutdown(socket)`: 关闭 TCP 套接字上的连接。
- `ax_udp_socket()`: 创建一个新的 UDP 套接字。
- `ax_udp_socket_addr(socket)`: 返回 UDP 套接字的本地地址和端口。
- `ax_udp_peer_addr(socket)`: 返回 UDP 套接字的远程地址和端口。
- `ax_udp_set_nonblocking(socket, nonblocking)`: 将此 UDP 套接字移入或移出非阻塞模式。
- `ax_udp_bind(socket, addr)`: 将 UDP 套接字绑定到给定的地址和端口。
- `ax_udp_recv_from(socket, buf)`: 在 UDP 套接字上接收单个数据报消息。
- `ax_udp_peek_from(socket, buf)`: 在 UDP 套接字上接收单个数据报消息，而不将其从队列中移除。
- `ax_udp_send_to(socket, buf, addr)`: 在 UDP 套接字上向给定地址发送数据。
- `ax_udp_connect(socket, addr)`: 将此 UDP 套接字连接到远程地址。
- `ax_udp_send(socket, buf)`: 在 UDP 套接字上向其连接的远程地址发送数据。
- `ax_udp_recv(socket, buf)`: 在 UDP 套接字上从其连接的远程地址接收单个数据报消息。
- `ax_udp_poll(socket)`: 返回 UDP 套接字是否可读或可写。
- `ax_dns_query(domain_name)`: 将主机名解析为 IP 地址列表。
- `ax_poll_interfaces()`: 轮询网络堆栈。

### 图形操作
`display` 模块提供图形操作功能。
- `ax_framebuffer_info()`: 获取帧缓冲区信息。
- `ax_framebuffer_flush()`: 刷新帧缓冲区，即在屏幕上显示。

### 输入/输出操作
`io` 模块定义了 I/O 操作类型。
- `AxPollState`: 表示 I/O 资源的轮询状态。

**Section sources**
- [arceos_api/lib.rs](file://arceos/api/arceos_api/src/lib.rs#L1-L406)
- [arceos_api/macros.rs](file://arceos/api/arceos_api/src/macros.rs#L1-L111)
- [arceos_api/imp/mod.rs](file://arceos/api/arceos_api/src/imp/mod.rs#L1-L49)

## POSIX 兼容 API (arceos_posix_api)

POSIX 兼容 API（arceos_posix_api）为应用程序提供了与标准 POSIX 系统调用兼容的接口。这些 API 允许使用标准 C 库函数的应用程序在 ArceOS 上运行，同时在底层调用 ArceOS 原生 API。

### 支持的系统调用
该 API 支持广泛的 POSIX 系统调用，涵盖文件操作、进程控制和网络通信。

#### 文件操作
- `sys_open`: 打开一个文件。
- `sys_lseek`: 设置文件的位置。
- `sys_stat`: 获取文件的元数据。
- `sys_fstat`: 通过文件描述符获取文件元数据。
- `sys_lstat`: 获取符号链接的元数据。
- `sys_getcwd`: 获取当前工作目录的路径。
- `sys_rename`: 重命名文件或目录。

#### 进程控制
- `sys_exit`: 退出当前进程。
- `sys_getpid`: 获取当前进程 ID。
- `sys_sched_yield`: 让出 CPU 时间片。

#### 时间操作
- `sys_clock_gettime`: 获取指定时钟的时间。
- `sys_nanosleep`: 使调用线程休眠指定的时间。

#### 网络通信
- `sys_socket`: 创建一个用于通信的套接字。
- `sys_bind`: 将地址绑定到套接字。
- `sys_connect`: 将套接字连接到指定的地址。
- `sys_sendto`: 向指定地址发送消息。
- `sys_send`: 向已连接的地址发送消息。
- `sys_recvfrom`: 接收消息并获取其源地址。
- `sys_recv`: 接收消息。
- `sys_listen`: 在套接字上监听连接。
- `sys_accept`: 接受连接。
- `sys_shutdown`: 关闭全双工连接。
- `sys_getaddrinfo`: 查询域名的地址。
- `sys_freeaddrinfo`: 释放查询到的 `addrinfo` 结构。
- `sys_getsockname`: 获取套接字绑定的地址。
- `sys_getpeername`: 获取套接字连接的对等地址。

#### I/O 多路复用
- `sys_select`: 同步 I/O 多路复用。
- `sys_epoll_create`: 创建一个 epoll 实例。
- `sys_epoll_ctl`: 控制 epoll 实例。
- `sys_epoll_wait`: 等待 epoll 事件。

#### 线程管理
- `sys_pthread_create`: 创建一个新线程。
- `sys_pthread_exit`: 终止调用线程。
- `sys_pthread_join`: 等待线程终止。
- `sys_pthread_self`: 返回调用线程的句柄。
- `sys_pthread_mutex_init`: 初始化互斥锁。
- `sys_pthread_mutex_lock`: 锁定互斥锁。
- `sys_pthread_mutex_unlock`: 解锁互斥锁。

### 与标准 POSIX 的差异
- **IPv6 支持**: 当前实现不支持 IPv6，`sys_getaddrinfo` 和 `from_sockaddr` 函数在遇到 IPv6 地址时会 panic。
- **`sys_lstat` 实现**: `sys_lstat` 的实现是临时的（TODO），目前不支持符号链接的实际元数据查询。
- **`sys_getaddrinfo` 实现**: `sys_getaddrinfo` 的实现是简化的，仅返回 TCP 参数，并且忽略 `hints` 参数。
- **`sys_writev` 实现**: `sys_writev` 通过循环调用 `sys_write` 来实现，而不是原子操作。
- **`sys_read` 和 `sys_write` 行为**: 当 `fd` 特性未启用时，`sys_read` 和 `sys_write` 的行为受到限制，例如，不能向标准输出/错误写入或从标准输入以外的文件描述符读取。

**Section sources**
- [arceos_posix_api/lib.rs](file://arceos/api/arceos_posix_api/src/lib.rs#L1-L61)
- [arceos_posix_api/imp/mod.rs](file://arceos/api/arceos_posix_api/src/imp/mod.rs#L1-L21)
- [arceos_posix_api/imp/io.rs](file://arceos/api/arceos_posix_api/src/imp/io.rs#L1-L73)
- [arceos_posix_api/imp/fs.rs](file://arceos/api/arceos_posix_api/src/imp/fs.rs#L1-L218)
- [arceos_posix_api/imp/net.rs](file://arceos/api/arceos_posix_api/src/imp/net.rs#L1-L582)

## Rust 用户库 (axstd)

Rust 用户库（axstd）旨在模仿 Rust 标准库（std），为 `no_std` 环境提供功能。它为 Rust 应用程序提供了一个熟悉的接口，同时在底层调用 ArceOS 原生 API，而不是依赖 libc 和系统调用。

### 功能说明
axstd 通过重新导出 `alloc` 和 `core` crate 的模块，为用户提供了与标准库相似的 API。当启用特定功能时，它会提供更高级的功能。

- **内存分配**: 当启用 `alloc` 功能时，`axstd` 会重新导出 `alloc` crate 的模块，如 `boxed`、`collections`、`string` 和 `vec`。
- **核心类型**: `axstd` 始终重新导出 `core` crate 的核心模块，如 `arch`、`cell`、`cmp`、`hint`、`marker`、`mem`、`ops`、`ptr`、`slice` 和 `str`。
- **文件系统**: 当启用 `fs` 功能时，`axstd` 会导出 `fs` 模块，提供文件和目录操作。
- **网络**: 当启用 `net` 功能时，`axstd` 会导出 `net` 模块，提供 TCP/UDP 通信功能。
- **其他模块**: `axstd` 还提供了 `env`、`io`、`os`、`process`、`sync`、`thread` 和 `time` 等模块。

### 与标准库的差异
- **底层实现**: axstd 的功能直接调用 ArceOS 原生 API，而不是通过系统调用，这可以提高性能并减少开销。
- **功能子集**: axstd 是一个精简的标准库，可能不包含标准库中的所有功能和方法。
- **特性驱动**: axstd 的功能是通过 Cargo 特性（如 `fs`、`net`、`multitask`）启用的，这允许用户根据需要选择功能，从而优化二进制大小。

**Section sources**
- [axstd/lib.rs](file://arceos/ulib/axstd/src/lib.rs#L1-L78)
- [axstd/fs/mod.rs](file://arceos/ulib/axstd/src/fs/mod.rs#L1-L78)
- [axstd/net/mod.rs](file://arceos/ulib/axstd/src/net/mod.rs#L1-L47)

## C 用户库 (axlibc)

C 用户库（axlibc）为 C 应用程序提供类 libc 的功能。它实现了标准 C 库函数，这些函数在底层调用 ArceOS POSIX 兼容 API，使 C 程序能够在 ArceOS 上运行。

### 功能说明
axlibc 通过将 C 函数（使用 `#[no_mangle]` 和 `extern "C"` 标记）绑定到 ArceOS POSIX API 的相应函数来工作。它还处理错误码的转换。

- **I/O 操作**: `read`、`write`、`writev` 函数分别绑定到 `sys_read`、`sys_write` 和 `sys_writev`。
- **进程控制**: `getpid`、`exit` 和 `abort` 函数分别绑定到 `sys_getpid`、`sys_exit`。
- **文件系统**: `ax_open`、`lseek`、`stat`、`fstat`、`lstat`、`getcwd` 和 `rename` 函数分别绑定到相应的 POSIX 系统调用。
- **网络**: `socket`、`bind`、`connect`、`sendto`、`send`、`recvfrom`、`recv`、`listen`、`accept`、`shutdown`、`getaddrinfo`、`freeaddrinfo`、`getsockname` 和 `getpeername` 函数分别绑定到相应的网络系统调用。
- **线程**: `pthread_create`、`pthread_exit`、`pthread_join`、`pthread_self`、`pthread_mutex_init`、`pthread_mutex_lock` 和 `pthread_mutex_unlock` 函数分别绑定到相应的 pthread 系统调用。
- **其他**: `malloc`、`free`、`clock_gettime`、`nanosleep` 等函数也提供了相应的实现。

### 错误处理
axlibc 使用 `utils::e` 函数将 ArceOS POSIX API 返回的错误码转换为符合 C 标准的负数错误码。例如，在 `getaddrinfo` 函数中，它将返回值映射到 `EAI_FAIL` 或 `EAI_NONAME` 等常量。

**Section sources**
- [axlibc/lib.rs](file://arceos/ulib/axlibc/src/lib.rs#L1-L126)
- [axlibc/io.rs](file://arceos/ulib/axlibc/src/io.rs#L1-L33)
- [axlibc/unistd.rs](file://arceos/ulib/axlibc/src/unistd.rs#L1-L21)
- [axlibc/fs.rs](file://arceos/ulib/axlibc/src/fs.rs#L1-L68)
- [axlibc/net.rs](file://arceos/ulib/axlibc/src/net.rs#L1-L181)