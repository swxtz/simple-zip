Simple Zip - Rust-based Library for File Compression and Decompression
======


[![Crates.io version](https://img.shields.io/crates/v/simple-zip.svg)](https://crates.io/crates/simple-zip)
[![Discord](https://img.shields.io/crates/dv/simple-zip/1.0.0)](https://crates.io/crates/simple-zip)
![Crates.io Downloads (latest version)](https://img.shields.io/crates/dv/simple-zip)

<img alt="GitHub repo size" src="https://img.shields.io/github/languages/code-size/swxtz/simple-zip?style=flat-square">
<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/swxtz/simple-zip?style=flat-square">
<img alt="Lincese" src="https://img.shields.io/github/license/swxtz/simple-zip?style=flat-square">



[Documentation](https://docs.rs/simple-zip/)

Info
-----
# Simple Zip - Rust-based Library for File Compression and Decompression

**Simple Zip** is a Rust library designed to simplify and enhance the process of compressing and decompressing files in development environments. Created with a DX First approach (Developer Experience comes first), Simple Zip prioritizes the developer experience to ensure that working with compressed files is an intuitive and efficient task.

## Key Features

- **DX First:** The library is conceived from the ground up with the goal of providing a hassle-free development experience. All functionalities are designed with ease of use and developer productivity in mind.

- **Optimized Performance:** Despite the focus on developer experience, Simple Zip, written in Rust, also excels in terms of performance. Compression and decompression operations are carried out efficiently, ensuring fast execution times.

## Features

- **Simple Compression (Coming Soon):** While compression is not yet implemented, the library anticipates the addition of a straightforward compression method, making it easy to compress files with just a few lines of code.

- **Intuitive Decompression:** File decompression is simplified using methods like `Unzip::local_str(&path)` or `Unzip::local_buffer(&pathbuf)`, providing seamless integration into development routines.

- **Format Support:** Currently, Simple Zip supports decompression for .zip files. However, there are plans to expand support to include all major popular formats in future updates.

## How to Use

1. **Add to Your Cargo.toml:**
   ```toml
   [dependencies]
   simple-zip = "0.1.0"
   ```

2. **Usage in Your Rust Code:**
   ```rust
   use simple_zip::Unzip;

   // Decompress a file using a path string
   Unzip::local_str(&path);

   // Or decompress using a buffer
   Unzip::local_buffer(&pathbuf);
   ```

   Note: Compression functionality is coming soon.

## Contributions

Contributions are welcome! Feel free to report issues, suggest improvements, or submit pull requests. Together, we can make Simple Zip, the Rust-based library, an even more powerful and developer-friendly tool.

## License

This project is licensed under the [MIT License](LICENSE) - see the LICENSE file for more details.

---

**Simple Zip** - Developed in Rust with a passion to provide a straightforward development experience. Support for additional popular formats is planned for future updates. 