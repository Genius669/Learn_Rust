### cargo build
`cargo build`会创建可执行文件并执行和执行文件。第一次运行cargo build的时候会在顶层目录生成`cargo.lock`文件，该文件负责追踪项目以来的精确版本，不要手动修改此文件！

#### cargo build --release
`cargo build --release`发布应用。

### cargo run
`cargo run`编译+执行，如果没有修改源码，就会一直执行二进制文件。

### cargo check
`cargo check`检查代码，保证代码通过编译，但是不产生可执行文件（比`cargo build`快很多）。