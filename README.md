# rust
learn rust.

[TOC]

## code

### [01 hello word](./src/01/01.rs)

### [02 hello word](./src/01/01.rs)

## Cargo

Cargo 是 Rust 的构建系统和包管理工具，同时 Rustacean 们使用 Cargo 来管理它们的 Rust 项目。Cargo 负责三个工作：构建你的代码，下载你代码依赖的库并编译这些库。我们把你代码需要的库叫做“依赖（dependencies）”因为你的代码依赖他们。

最简单的 Rust 程序并没有任何依赖，所以目前我们只使用它的第一部分功能。随着你编写更加复杂的 Rust 程序，你会想要添加依赖，那么如果你使用 Cargo 开始的话，这将会变得简单许多。

因为绝大部分 Rust 项目使用 Cargo，本书接下来的部分将假设你使用它。如果你使用官方安装包的话，Rust 自带 Cargo。如果你使用其他方式安装 Rust 的话，你可以在终端输入如下命令检查你是否安装了 Cargo：

```
$ cargo --version
```

如果你看到了版本号，一切 OK！如果你一个类似“command not found”的错误，那么你应该去查看你安装 Rust 的系统的相关文档，来确定 Cargo 是否需要单独安装。

### 转换到 Cargo

让我们将 Hello World 程序迁移至 Cargo。为了 Cargo 化一个项目，需要做三件事：
1. 将源文件放到正确的目录
2. 删除旧的可执行文件（Windows下是main.exe，其他平台是main）。
3. 创建一个 Cargo 配置文件
让我们开始吧！

### 创建源文件目录并移除旧的可执行文件
首先，回到你的终端，移动到你的hello_world目录，并输入如下命令：

```
$ mkdir src
$ mv main.rs src/main.rs # or 'move main.rs src/main.rs' on Windows
$ rm main  # or 'del main.exe' on Windows
```

Cargo 期望源文件位于 src 目录，所以先做这个。这样将项目顶级目录（在这里，是 hello_world）留给 README，license 信息和其他跟代码无关的文件。这样，Cargo 帮助你保持项目干净整洁。一切井井有条。

现在，移动main.rs到src目录，并删除你用rustc创建的编译过的文件。像往常一样，如果你使用 Windows，用main.exe代替main。

例子中我们继续使用main.rs作为源文件名是因为它创建了一个可执行文件。如果
你想要创建一个库文件，使用lib.rs作为文件名。Cargo 使用这个约定来正确编译你的项目，不过如果你愿意的话也可以覆盖它。

###创建配置文件

下一步，在hello_world目录创建一个文件，叫做Cargo.toml。

确保Cargo.toml的C是大写的，否则 Cargo 不知道如何处理配置文件。

这个文件使用TOML（Tom's Obvious, Minimal Language）格式。 TOML 类似于 INI，不过有一些额外的改进之处，并且被用作 Cargo 的配置文件。
在这个文件中，输入如下信息：
```
[package]

name = "hello_world"
version = "0.0.1"
authors = [ "Your name <you@example.com>" ]
```

第一行的[package]表明下面的语句用来配置一个包。随着我们在这个文件增加更多的信息，我们会增加其他部分，不过现在，我们只有包配置。
另外三行设置了 Cargo 编译你的程序所需要知道的三个配置：包的名字，版本，和作者。

当你在Cargo.toml中添加完这些信息后，保存它以完成配置文件的创建。

### 构建并运行 Cargo 项目

当Cargo.toml文件位于项目的根目录时，我们就准备好可以构建并运行 Hello World 程序了！为此，我们输入如下命令：

```
$ cargo build
   Compiling hello_world v0.0.1 (file:///home/yourname/projects/hello_world)
$ ./target/debug/hello_world
Hello, world!
```

如果一切顺利，你应该再次看到Hello, world!出现在终端里。

你刚刚用cargo build构建了一个程序并用./target/debug/hello_world运行了它，不过你也可以用如下的一步操作cargo run来完成这两步操作：

```
$ cargo run
     Running `target/debug/hello_world`
Hello, world!
```

`run`命令在你需要快速迭代项目时显得很有用。

注意这个例子并没有重新构建项目。Cargo 发现文件并没有被修改，所以它只是运行了二进制文件。如果你修改了源文件，Cargo 会在运行前重新构建项目，这样你将看到像这样的输出：
```
$ cargo run
   Compiling hello_world v0.0.1 (file:///home/yourname/projects/hello_world)
     Running `target/debug/hello_world`
Hello, world!
```
Cargo 检查任何项目文件是否被修改，并且只会在你上次构建后修改了他们才重新构建。

对于简单的项目，Cargo 并不比使用rustc要好多少，不过将来它会变得有用。这在你开始使用 crate 时显得尤为正确；（crate）在其他语言中有“库（library）”或“包（package）”这样的同义词。对于包含多个 crate 的项目，让 Cargo 来协调构建将会轻松很多。有了 Cargo，你可以运行cargo build，而一切将有条不紊的运行。

### 发布构建

当你的项目准备好发布了，可以使用cargo build --release来优化编译项目。这些优化可以让 Rust 代码运行的更快，不过启用他们会让程序花更长的时间编译。这也是为何这是两种不同的配置，一个为了开发，另一个构建提供给用户的最终程序。

### 那个Cargo.lock是什么？

运行这个命令同时也会让 Cargo 创建一个叫做Cargo.lock的文件，它看起来像这样：

```
[root]
name = "hello_world"
version = "0.0.1"
```

Cargo 用Cargo.lock文件跟踪你程序的依赖。这里是 Hello World 项目的Cargo.lock文件。这个项目并没有依赖，所以内容有一点稀少。事实上，你自己甚至都不需要碰这个文件；仅仅让 Cargo 处理它就行了。

就是这样！如果你一路跟过来了，你应该已经成功使用 Cargo 构建了hello_world。

虽然这个项目很简单，现在它使用了很多在你余下的 Rust 生涯中将会用到的实际的工具。事实上，你可以期望使用如下命令的变体开始所有的 Rust 项目：

```
$ git clone someurl.com/foo
$ cd foo
$ cargo build
```

### 创建新 Cargo 项目的简单方法

你并不需要每次都过一遍上面的操作来开始一个新的项目！Cargo 可以快速创建一个骨架项目目录这样你就可以立即开始开发了。

用 Cargo 来开始一个新项目，在命令行输入cargo new：

```
$ cargo new hello_world --bin
```

这个命令传递了--bin参数因为我们的目标是直接创建一个可执行程序，而不是一个库。可执行文件通常叫做二进制文件（因为它们位于/usr/bin，如果你使用 Unix 系统的话）。

Cargo 为我们创建了两个文件和一个目录：一个Cargo.toml和一个包含了main.rs文件的src目录。这应该看起来很眼熟，他们正好是我们在之前手动创建的那样。

这些输出是你开始所需要的一切。首先，打开Cargo.toml。它应该看起来像这样：

```
[package]

name = "hello_world"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
```

不要担心[dependencies]那一行，之后我们会讲到。

Cargo 已经根据你给出的参数和git全局配置给出了合理的默认配置。你可能会注意到 Cargo 也把hello_world目录初始化为了一个git仓库。

这是应该写入src/main.rs的代码：

```
fn main() {
    println!("Hello, world!");
}
```

Cargo 已经为你生成了一个“Hello World!”，现在你已经准备好开始撸代码了！

> 注意：如果你想要查看 Cargo 的详细信息，请查看官方的Cargo 指导，它包含了所有这些功能。
