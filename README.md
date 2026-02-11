# Build Your Own Shell (Rust)

[![progress-banner](https://backend.codecrafters.io/progress/shell/1b45c1e5-6de1-4249-9689-9f1eb08071a7)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

This is my implementation of the ["Build Your Own Shell" Challenge](https://app.codecrafters.io/courses/shell/overview) provided by CodeCrafters, written in **Rust**. 

It is a POSIX-compliant shell capable of interpreting shell commands, running built-in commands, and executing external programs.

## Features

* **Interactive REPL**: Built with `rustyline` to support command history and auto-completion.
* **Built-in Commands**: Supports standard built-ins including `cd`, `pwd`, `echo`, `exit`, `type`, and `history`.
* **Program Execution**: Resolves and executes external programs found in the system's `$PATH`.
* **Output Redirection**: Supports standard output and standard error redirection (e.g., `>`, `>>`, `1>`, `2>`).
* **Pipelining**: Can chain multiple commands together using pipes (`|`), allowing concurrent execution of external commands.

## Usage

You can build and run the shell locally using the provided script:

```bash
# Clone the repository
git clone <your-repo-url>
cd codecrafters-shell-rust

# Run the shell
./your_program.sh
```

Alternatively, you can use Cargo directly:
```bash
cargo run --release
```
