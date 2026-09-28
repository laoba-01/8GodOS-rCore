# 8GodOS-rCore

个人 OS 训练营（LearningOS **2026a**）工作区，存放两个课程项目的**工作副本**与做题记录。

> ⚠️ 本仓库是**个人学习工作区**，不是课程官方仓库。课程原件请以 LearningOS 上游为准（见下方来源）。

## 目录结构

```
8GodOS-rCore/
├── 2026a-rustling-laoba-01/       Rust 语言基础练习（25 个练习集，112 个 .rs）
└── 2026a-oscamp-base-laoba-01/    操作系统训练营基础练习（6 个模块）
```

### `2026a-rustling-laoba-01/`

Rustlings 形式的语言基础练习，覆盖变量、所有权、错误处理、泛型、trait、生命周期等。

### `2026a-oscamp-base-laoba-01/`

操作系统训练营基础练习，共 6 个模块：

| 模块 | 主题 |
| --- | --- |
| `01_concurrency_sync` | 并发同步：线程、互斥锁、channel、进程管道 |
| `02_no_std_dev` | no_std 开发：内存原语、bump/free-list 分配器、系统调用封装、fd 表 |
| `03_os_concurrency` | OS 并发：原子操作、内存序、自旋锁、读写锁 |
| `04_context_switch` | 上下文切换：栈式协程、绿色线程（**交叉编译到 RISC-V**） |
| `05_async_programming` | 异步编程：Future、tokio、异步 channel、select/timeout |
| `06_page_table` | 页表：PTE 标志、页表遍历、多级页表、TLB 模拟 |

自带命令行工具 `oscamp-cli`：

```bash
cargo run -p oscamp-cli -- watch    # 交互模式
```

## 环境要求

- **Rust 1.98.1**（由 `rust-toolchain.toml` 固定，profile `minimal`）
- 第 4 模块需要 RISC-V 交叉环境：

```bash
# Debian / Ubuntu
sudo apt-get install -y build-essential gcc-riscv64-linux-gnu qemu-user
```

- 目标平台：`riscv64gc-unknown-linux-gnu`
- 交叉 linker：`riscv64-linux-gnu-gcc`
- 运行器：`qemu-riscv64 -L /usr/riscv64-linux-gnu`

> **注意**：`qemu-user` 与 `qemu-user-static` 不是一回事。只有 `qemu-user` 提供 `/usr/bin/qemu-riscv64`（仓库 `.cargo/config.toml` 里的 runner 用的正是这个名字），`qemu-user-static` 只提供 `qemu-riscv64-static`。

> **建议在 WSL2 / Linux 原生文件系统上工作**。若仓库放在 Windows 盘的 `/mnt/` 挂载下（9p 文件系统），`notify`/inotify 收不到 Windows 侧的改动，`oscamp-cli watch` 会「改了文件但界面不刷新」，编译速度也明显更慢。

## 来源与许可

本仓库包含以下两个上游项目的副本，各自保留其原始 LICENSE：

| 目录 | 上游仓库 | 许可 |
| --- | --- | --- |
| `2026a-rustling-laoba-01/` | [LearningOS/2026a-rustling-laoba-01](https://github.com/LearningOS/2026a-rustling-laoba-01) | MIT (© 2016 Carol Nichols / Goulding) |
| `2026a-oscamp-base-laoba-01/` | [LearningOS/2026a-oscamp-base-laoba-01](https://github.com/LearningOS/2026a-oscamp-base-laoba-01) | Apache-2.0 |

这两个子目录中的内容版权归原作者所有，此处仅作个人学习用途。为保持本工作区整洁，两个子目录中面向课程基础设施的 `.github/` 目录已从本副本中移除（上游原件未受影响）。
