# AdiIRC/mIRC DLL Template

![Build Status](../../actions/workflows/build.yml/badge.svg)
![Sponsored by Byrnes Tech Pty Ltd](https://img.shields.io/badge/Sponsor-Byrnes%20Tech%20Pty%20Ltd-blue)

This repository is a template for building DLLs for AdiIRC and mIRC, two extensible IRC clients for Windows. It includes a GitHub workflow to build the DLLs for different targets.

## Prerequisites

- Rust installed on your system. You can download it from [rust-lang.org](https://www.rust-lang.org/).

## Targets

This project builds DLLs for the following targets:
- `x86_64-pc-windows-msvc` for AdiIRC 64 Bit (and the upcoming 64 Bit mIRC)
- `i686-pc-windows-msvc` for AdiIRC 32 Bit and mIRC
- `aarch64-pc-windows-msvc` for AdiIRC ARM

## Using This Template

1. Click the "Use this template" button on the GitHub repository page to create a new repository based on this template.

2. Clone your new repository to your local machine:
    ```sh
    git clone https://github.com/your-username/your-repo-name.git
    cd your-repo-name
    ```

3. If you have Rust installed, you can build the project locally:
    ```sh
    cargo build --release --target x86_64-pc-windows-msvc
    cargo build --release --target i686-pc-windows-msvc
    cargo build --release --target aarch64-pc-windows-msvc
    ```

4. Alternatively, you can rely on the built-in GitHub workflow to build the DLLs for you. Simply push your changes to the repository, and the workflow will automatically build the DLLs and upload them as artifacts.

## GitHub Workflow

The GitHub workflow is defined in [build.yml](.github/workflows/build.yml). It builds the DLLs for the specified targets and uploads them as artifacts.

## Sponsors

Special thanks to our sponsor, Byrnes Tech Pty Ltd, for their support in creating this project.

## Contributing

Feel free to open issues or submit pull requests if you have any improvements or suggestions.

## License

This project is licensed under the MIT License. See the LICENSE file for details.