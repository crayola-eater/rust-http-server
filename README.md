# Purpose

- A basic HTTP server written for learning purposes, using the Rust programming language and [this course on Rust](https://www.udemy.com/course/rust-fundamentals/) by Lyubomir Gavadinov.

# How to run

- Install Rust, Cargo, LLVM
- Clone this repo
- `cd` into cloned directory and then `cargo run`

# Tools used

- Rust standard library only
- No external crates

# What I learned

The Rust programming language and the concepts/features it offers:

- **Immutability** - In Rust, everything is immutable by default. If you want to re-assign to a variable, it needs to be declared as mutable. Similarly, if you want to update some struct's state/data, the struct must be declared mutable.

- **Ownership** - Rust doesn't have a garbage collector. Instead, each bit of data/resource has just one "owner". When the owner goes out of scope, any resources relating to that data will be dropped/freed automatically.

  - If our application requires a change in the data's owner, we can "move" the ownership from the current owner to the next owner. However, there's still only one owner ever at any moment.

- **Borrowing** - If another part of our code needs access to a resource/variable that it does not "own", it may "borrow" it according to these rules:

  - Immutable/shared - this is appropriate if only read-only access is required. (In other words, the thing that's borrowing does _not_ need to update the data being borrowed.) We can give out as many of such read-only references at a time as we like (the only limitation being that we cannot give out a mutable reference)
  - Mutable - this is appropriate if the thing that's borrowing needs to update the data. We can only give one of such mutable references out at a time.
  - Through these rules, Rust ensures that if we immutably borrow a value, we can rely on that value not changing. Conversely, if we mutably borrow a value, we have exclusive access to it. (Otherwise,other parts of our code could behave unpredictably or non-deterministically if values could be mutably and immutably borrowed at the same time.)

- **Memory safety** - Through static analysis and enforcing a novel ownership/borrowing rule set, Rust allows us to write code that is performant and free from memory-related issues (e.g. use after free, dangling pointers, race conditions, double free, memory leaks).

- **Enums** - Rust's enums are very powerful and flexible, as each variant can optionally have data associated with it (and the data's type can vary independently across variants).

- **Pattern matching** - Rust offers us features like `if let` and `match` for our code to take different paths based on some expression's value.

  - `match` is similiar to `switch` statements in other languages, but more flexible and powerful. For example, each match arm can contain variable bindings plus further `if` conditions that the expression needs to match in order for the code in that match arm to execute.
  - Side note: the compiler forces us to handle every case (i.e. exhaustively pattern match). For example, if an operation can fail or if a value may not be present at run time, we are forced to write code to handle such potential failures (meaning we don't end up coding only the "happy path").

- **Expressions** - A lot of things are expressions in Rust (e.g. blocks, `if` expressions, implicit `return` expressions, `match` expressions, `loop` expressions).

- **Memory management** - The difference between resources whose size is known at compile time and can therefore be stack allocated; in contrast to values that need to be dynamically sized at runtime and are therefore heap allocated. These are details that I've not had to concern myself (until now) as languages like TypeScript, Python, etc. take care of such particulars for you.

- **Computer networking** - HTTP is an application level protocol that builds on top of TCP.
  - So to establish a HTTP connection, we need to first establish a TCP connection (between the sender and recipient).
  - Establishing a TCP connection requires opening a socket (on both the sender and recipient). The Rust standard library's `net` module provides a `TcpListener` struct that can be used to achieve this socket binding and abstracts away the underlying OS calls and interactions (that are required to create and bind to a socket).
  - This `TcpListener` then lets us listen for incoming TCP requests/streams. The incoming bytes can be read into a buffer/array and then decoded into a string.
  - We can then parse the string into a "HTTP request" (based on the structure defined in the RFCs), and then route the request (based on the parsed "path") to eventually serve a response to the sender.
  - During this process, there are many operations and subsequently many _potential_ points of failure. Rust's `Option` and `Result` enums ensure that our code is prepared to handle scenarios where a value is absent or an operation failed.

# Things to explore next

- The current implementation is very basic (since the aim was to learn Rust and not have a production-ready HTTP server). It would be good to implement support for parsing request headers and body, as well as adding response headers.
- Have a look at Tokio, which is a performant asynchronous runtime for Rust. It would allow us to become familiar with async/await and cooperative multitasking in Rust.

# Created

- Jun 2021
