# Installation

For now, Tron can only be installed on UNIX like operating systems, such as: Linux and MacOS.

```bash
curl -o tron https://tronlang.org/tron-lang
sudo mv tron /usr/local/bin/
sudo chmod +x /usr/local/bin/tron
```

Execute code above in the terminal and it will install interpreter and cli to your computer.

## Initalization

To start with Tron create any `.tron` file, for example `main.tron` and lets write classical "Hello World" code inside:

```rs
print "Hello World";
```

Save your file and run:

```bash
tron main.tron
```

Hello world will be printed on the terminal.

## CLI Commands

`tron <filename>` - runs Tron file
`tron version` - outputs current verison


read next: [Data types and Variables](./data-types.md)