# substrate-erc20

> ERC20 on substrate and a official token example 

## Install

> First install substrate v1.0.0rc1 environment , the operation takes a long time！

```shell
$ curl https://getsubstrate.io -sSf | bash 
$ source .cargo/env
```

> **Don't worry!If it has a bug!** just follow the operations.

```shell
$ git clone https://github.com/paritytech/substrate.git
$ cd substrate
$ ./scripts/build.sh
$ cargo build --release
$ cp target/release/substrate /usr/bin/
```

> Test the environment

```shell
$ rustc --version
$ substrate --version
```

> Run the code.Maybe you should install git first!

```shell
$ git clone https://github.com/SherLzp/substrate-erc20.git
$ cd substrate-erc20
$ ./build.sh
$ cargo build --release
$ ./target/release/substratekitties --dev --ws-external --rpc-external
```

> Now you can use the official website to access it！Good luck！

