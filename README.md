## Filecoin Smart Contract Template - Hello World Actor

### Prerequesites 
1. [Lotus](https://lotus.filecoin.io/developers/local-network/) `fvm-m2` branch
2. Rust 👇
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default nightly
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
rustup target add wasm32-unknown-unknown --toolchain nightly
```
Actor deployment on Filecoin is different from Ethereum, in that, actor deployment is a two-step process. The first step "installs" the actor in the virtual machine, which you can think of as inserting a flash drive with code on a computer so that you can run it. This is done via the install `install-actor` command. Once an actor has been installed to the VM, it needs to be created (e.g. instantiated). When created, the Actor runs an initializer method which can be used to set up things like access control for that specific instance of the Actor.

### Actor Deployment
```sh
cargo build # build the actor

lotus chain install-actor ./target/debug/wbuild/fil_hello_world_actor/fil_hello_world_actor.compact.wasm
# sending message...
# gas limit: 702194416
# waiting for message to execute...
# Actor Code CID: bafk2b[...]
# Installed: false (prior installation flag?)

lotus chain create-actor bafk2b[...]
# sending message...
# waiting for message to execute...
# b0hlbGxvIHdvcmxkICMxIQ==   <-- base64 encoded return data

echo "b0hlbGxvIHdvcmxkICMxIQ==" | base64 -d
```

### Further Reading
- [FVM Architecture](https://github.com/filecoin-project/fvm-specs/blob/main/01-architecture.md)
- [FVM 🤝 EVM Mapping](https://github.com/filecoin-project/fvm-specs/blob/main/04-evm-mapping.md)

- [The Rust Programming Language](https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html)
