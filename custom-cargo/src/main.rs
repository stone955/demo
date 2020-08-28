/*
- Customize your build through release profiles
  cargo build // build dev profile
  cargo build --release // build release profile

- Publish libraries on crates.io
  use /// to write markdown doc
  cargo doc --open
  cargo test will run the code examples in your documentation as tests!
  cargo login or pass --token

- Organize large projects with workspaces
  [workspace]
  members = [
    "adder"
  ]
  cargo run -p adder // specify which package in the workspace we want to run by using -p `package name`
  cargo test -p reducer // specify which package in the workspace we want to test by using -p `package name`

- Install binaries from crates.io
  cargo install

- Extend Cargo using custom commands
 */

fn main() {
    println!("Hello, world!");
}
