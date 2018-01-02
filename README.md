# Journal

> Dump micro knowledge fluently - Lazy load it later

---

```Bash
jrnl --tag finance --tag python --project trading <some-title>
# other properties and tag are automatically added (based on project,
time, ...)
```

---

### Installation

```Bash
# install Rust if necessary
curl https://sh.rustup.rs -sSf | sh
# reload shell and check installation:
rustc --version

# build project
cargo build
./target/debug/journal --help

# prepare file structure
mkdir -p $HOME/.jrnl/notes
```


### Resources

- [Vim setup](http://seenaburns.com/vim-setup-for-rust/)
- [Rust book - 2nd edition](https://doc.rust-lang.org/book/second-edition/ch01-00-introduction.html)
