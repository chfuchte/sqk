# sqk

A lightweight, vim-inspired terminal database explorer.

## Table of Contents

-   [Installation](#installation)
-   [Contributing](#contributing)
-   [License](#license)

## Installation

To install `sqk`, you can use one of the following installation scripts. These scripts will download the lastest release
and add it to your `$PATH`. The installation script will also create a `~/.sqk` directory where the configuration file be stored.

| Platform             | Method | Command                                                                                                                         |
| -------------------- | ------ | ------------------------------------------------------------------------------------------------------------------------------- |
| Linux/ MacOS         | curl   | `sh -c "$(curl -fsSL https://raw.githubusercontent.com/chfuchte/sqk/master/tools/install.sh)"`                                  |
| Linux/ MacOS         | wget   | `sh -c "$(wget -O- https://raw.githubusercontent.com/chfuchte/sqk/master/tools/install.sh)"`                                    |
| Linux/ MacOS         | fetch  | `sh -c "$(fetch -o - https://raw.githubusercontent.com/chfuchte/sqk/master/tools/install.sh)"`                                  |
| Windows (Powershell) | curl   | `Invoke-WebRequest https://raw.githubusercontent.com/chfuchte/sqk/master/tools/install.ps1 -OutFile install.ps1; ./install.ps1` |

Alternatively, you can download the latest release from the [releases page](https://github.com/chfuchte/sqk/releases)
or [build it from source](#building-from-source) and add the binary to your `$PATH` yourself.

### Building from source

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone the repository

```bash
git clone https://github.com/chfuchte/sqk.git && cd sqk
```

3. Build the binary

```bash
cargo build --release
```

## Contributing

If you'd love to help improving this tool checkout the [contributing guide](CONTRIBUTING.md).

## License

This project is licensed under the [MIT License](LICENSE.txt).
