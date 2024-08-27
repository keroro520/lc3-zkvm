# LC3 ZKVM, LC3 Zero-Knowledge Virtual Machine

## Introduction

LC3 ZKVM is a virtual machine implementation based on the Little Computer 3 (LC3) architecture, enhanced with zero-knowledge proof capabilities. This project aims to provide a secure and efficient environment for executing LC3 programs with privacy-preserving features.

## Build

```sh
cargo build --release
```

## Usage

```sh
cargo run --release --bin lc3-zkvm -- <path_to_obj_file>
```

Example:

```sh
cargo run --release --bin lc3-zkvm -- ./assets/hello.obj
```

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Reference

- 📚 [Little Computer 3 - Wikipedia](https://en.wikipedia.org/wiki/Little_Computer_3)
- 📚 [Write your Own Virtual Machine](https://www.jmeiners.com/lc3-vm/) By Justin Meiners and Ryan Pendleton
