# cwc

A simple Rust implementation of the Unix `wc` (word count) command-line tool.

## Overview

`cwc` is a command-line utility written in Rust that mimics the basic functionality of the Unix `wc` tool. It can count the number of lines, words, bytes, or characters in one or more files. This project is designed to help you understand how to work with command-line arguments using Rust, and to serve as a starting point for more advanced text processing utilities.

## Features

- **Byte Counting:** Use the `-c` flag to count the number of bytes in the input files.
- **Line Counting:** Use the `-l` flag to count the number of lines.
- **Word Counting:** Use the `-w` flag to count the number of words.
- **Character Counting:** Use the `-m` flag to count the number of characters.
- **Multiple File Support:** Process one or more file paths provided as arguments.

## Installation

To build and run `cwc`, you need to have [Rust](https://www.rust-lang.org/) installed on your system.

1. Clone the repository:

   ```bash
   git clone git@github.com:Paul-Dejean/wc.git
   cd wc
   ```

2. Build the project using Cargo:

   ```bash
   cargo build --release
   ```

3. (Optional) Install the binary globally:

   ```bash
   cargo install --path .
   ```

## Usage

Once built, you can run cwc from the command line. Below are some example usages:

```bash
./target/release/cwc path/to/file.txt
```

Or if installed globally:

```bash
cwc path/to/file.txt
```

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Contact

Created by Paul Dejean (pauldejeandev@gmail.com).
