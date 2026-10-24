# 8GodOS-rCore

个人 OS 训练营（LearningOS **2026a**）学习记录。

课程作业以**独立 clone** 的形式放在本机工作区，本仓库按分支保存各项目的代码快照；提交与推送在各自的课程仓库目录里进行。

> ⚠️ 本仓库是**个人学习工作区**，不是课程官方仓库。评分与成绩以上游 LearningOS 仓库为准（见下方来源）。

## 分支

| 分支 | 项目 | 上游 |
| --- | --- | --- |
| [`rustling`](https://github.com/laoba-01/8GodOS-rCore/tree/rustling) | Rustlings 语言练习 | [LearningOS/2026a-rustling-laoba-01](https://github.com/LearningOS/2026a-rustling-laoba-01) |
| [`oscamp-base`](https://github.com/laoba-01/8GodOS-rCore/tree/oscamp-base) | 训练营基础练习（6 个模块） | [LearningOS/2026a-oscamp-base-laoba-01](https://github.com/LearningOS/2026a-oscamp-base-laoba-01) |
| [`rcore-ch3`](https://github.com/laoba-01/8GodOS-rCore/tree/rcore-ch3) | rCore 实验 lab1（ch3） | [LearningOS/2026a-rcore-laoba-01](https://github.com/LearningOS/2026a-rcore-laoba-01) |
| [`rcore-ch4`](https://github.com/laoba-01/8GodOS-rCore/tree/rcore-ch4) | rCore 实验 lab2（ch4） | 同上 |

本机工作区结构：

```
8godOS-rCore/
├── 2026a-rustling-laoba-01/     Rust 语言基础练习
├── 2026a-oscamp-base-laoba-01/  训练营基础练习
└── 2026a-rcore-laoba-01/        rCore 操作系统实验（按章节切分支）
```

这三个目录各自是独立的课程仓库 clone，不被本仓库跟踪（见 `.gitignore`）。

## 项目说明

### Rustlings（`rustling` 分支）

Rustlings 形式的语言基础练习，覆盖变量、所有权、错误处理、泛型、trait、生命周期等，110 道练习全部完成。

### 训练营基础练习（`oscamp-base` 分支）

共 6 个模块：

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

### rCore 实验（`rcore-ch3` / `rcore-ch4` 分支）

在 [rCore-Tutorial](https://github.com/LearningOS/2026a-rcore-laoba-01) 基础上按章节完成的实验，一个章节一个分支：

| 分支 | 章节 | 内容 |
| --- | --- | --- |
| `rcore-ch3` | ch3 | 系统调用计数 + `sys_trace`（lab1） |
| `rcore-ch4` | ch4 | 虚存版 `get_time`、`trace`、`mmap`、`munmap`（lab2） |

- 实验报告：`reports/lab1.md`、`reports/lab2.md`，分别在对应分支内。
- 两个章节均已通过课程仓库的 GitHub Actions 评测。

## 环境要求

### Rustlings / 训练营基础练习

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

### rCore 实验

- Rust `nightly-2024-05-02` + `riscv64gc-unknown-none-elf`（由实验仓库的 `rust-toolchain.toml` 固定）
- 需要 `qemu-system-riscv64` 与 `make`，建议在 WSL2 / Linux 下构建
- 构建并运行：

```bash
cd 2026a-rcore-laoba-01
make -C os run BASE=0
```

## 来源与许可

本仓库保存的是以下上游项目的代码快照，各自保留其原始 LICENSE：

| 分支 | 上游仓库 | 许可 |
| --- | --- | --- |
| `rustling` | [LearningOS/2026a-rustling-laoba-01](https://github.com/LearningOS/2026a-rustling-laoba-01) | MIT (© 2016 Carol Nichols / Goulding) |
| `oscamp-base` | [LearningOS/2026a-oscamp-base-laoba-01](https://github.com/LearningOS/2026a-oscamp-base-laoba-01) | Apache-2.0 |
| `rcore-ch3` / `rcore-ch4` | [LearningOS/2026a-rcore-laoba-01](https://github.com/LearningOS/2026a-rcore-laoba-01) | GPL-3.0 |

这些分支中的内容版权归原作者所有，此处仅作个人学习用途。
