## Filecoin Smart Contract Template - Hello World Actor

### Prerequesites 
1. [Lotus](https://lotus.filecoin.io/developers/local-network/)`fvm-m2` branch
2. Rust 👇
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default nightly
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
rustup target add wasm32-unknown-unknown --toolchain nightly
```

### Further Reading
- [FVM 🤝 EVM Mapping](https://github.com/filecoin-project/fvm-specs/blob/main/04-evm-mapping.md)

- [The Rust Programming Language] https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html
