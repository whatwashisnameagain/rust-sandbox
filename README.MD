# Rust Sandbox

[Official Docs](https://www.rust-lang.org/)

## Installation
https://www.rust-lang.org/tools/install
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## Uninstall
`rustup self uninstall`

## Getting Started
https://www.rust-lang.org/learn/get-started
[ Playground ](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021)

## Creating a New Project
`cargo new [project-name]` 
or
`cargo init` if the repos is already created

##  Offical Docs / Rust-lang Book
https://doc.rust-lang.org/book/

## Issues encountered
### 020624 | 11A | Reumy
- On installation I found that rustup wasn't added to path so I had to manually add it. Luckily since I set up an alias to edit my zshrc with vi I was able to quickly create a manual export for it and getting running by testing "rustup update". The exported PATH I created was the following since rust binaries are located it $HOME/.cargo/bin: 

`export PATH="$PATH:$HOME/.cargo.bin"`

- Everything afterwards worked beautifully