# English 英语

RustIranta is a memory-safe programming language powered by a novel borrow-type system (ownership model) written in Rust; it uses LLVM as its backend.

At present the RustIranta compiler only handles basic expressions and translates source code into LLVM IR. Producing an executable requires setting up the LLVM backend yourself.

When compiling the LLVM IR you must link against the Iranta standard library, [RustIrantaSTD](https://github.com/CleanIce-BlueSnowy/RustIrantaSTD).

To build this project, you need to install Inkwell and the LLVM development components.

# Chinese 中文

RustIranta 是一个用 Rust 编写的新型借用类型系统（所有权系统）的内存安全的编程语言，采用 LLVM 作为后端。

目前的 RustIranta 编译器仅支持基本的表达式，且仅支持将源代码编译为 LLVM IR 代码。编译为可执行文件需要自行配置 LLVM 后端。

编译 LLVM IR 代码时需要链接到 Iranta 语言的标准库 [RustIrantaSTD](https://github.com/CleanIce-BlueSnowy/RustIrantaSTD)。

要编译此项目，你需要安装 Inkwell 和 LLVM 开发组件。
