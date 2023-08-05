[![AGPL License](https://img.shields.io/badge/license-Apache%20v2-blue.svg)](https://github.com/jkl-org/Tron/blob/main/LICENSE)

# Tron Programming Language

An open-source programming language written in Rust.

## Installation

Before starting the installation, please make sure that you have already installed [Rust](https://www.rust-lang.org/tools/install).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After that, you can create a new directory and install the Tron:

```bash
mkdir helloTron
cd helloTron
npm install tron-lang
```

add the following code in the `package.json`:

```json
  "scripts": {
    "install": "cd node_modules/tron-lang \n cargo build",
    "start": "cd node_modules/tron-lang \n cargo run main"
  },
```

create the `main.tron` file

```rs
print "Hello, Tron!";
```

and run the `npm run start` command in the terminal

## Usage/Examples

```rs
let x = 3;
if x > 1 {
  print "passed";
} else {
  error "failed";
}
```

## Acknowledgements

- [Home Page](https://tronlang.org/)
- [Documentation](https://docs.tronlang.org/)
- [News Page](https://news.tronlang.org/)

## Feedback

If you have any feedback, please reach out to us at tronlang@proton.me

## Support

For support, email tronlang@proton.me or join our [Discrod server](https://discord.com/invite/UgUaUPhzug).
