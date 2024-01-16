# Overview
This project is an exploration of Foreign Function Interface (FFI) and Leptos in Rust. The project consists of four crates, each serving a specific role within the ecosystem. While more efficient monitoring tools exist, this project was developed for fun and to delve deeper into Rust's capabilities.

## Crates
1. proc-mon-sys

Description: This crate is the foundation of our project. It handles the FFI with libc, allowing us to interface directly with the system's process monitoring capabilities at a low level. It's crucial for understanding how Rust interacts with C libraries and system-level resources.

2. proc-mon

Description: Built on top of proc-mon-sys, this crate provides a safe, Rust-centric abstraction. It encapsulates the complexity of FFI calls and presents a more Rust-friendly interface, ensuring safety and ease of use while interacting with system-level process monitoring features.

3. shared-types

Description: This crate is the central hub for common types used across the project. It ensures consistency and reduces redundancy by providing shared data structures and types that are fundamental to process monitoring and FFI operations in Rust.

4. app

Description: The culmination of our project, app is a desktop application for process monitoring. It leverages the lower-level crates to provide a user-friendly interface for monitoring system processes. This GUI showcases the practical application of our FFI exploration in a real-world context.

# Getting Started

1. **Clone the Repository:**

```
git clone https://github.com/lpturmel/proc-mon.git
```

2. **Build and Run:**

```
cd proc-mon/app
```

## For running locally

```
cargo tauri dev
```

## For building and using it

```
cargo tauri build
```

Note: that this assumes you have a Mac running with `libproc` discoverable
