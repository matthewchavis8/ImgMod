
```markdown
# 🖼️ ImgMod: Image Encoder & Decoder with Hidden Messages

 A **Rust** implementation of an image encoder and decoder with support for embedding and retrieving **hidden messages** inside PNG chunks using steganography. This project is a port from my **C++** version and is currently under active development.

---

## 🚀 Features

-   **Read & Write PNG Files:** Manipulate PNG files at the chunk level.
-   **Dynamic Chunk Handling:** Extract and modify image chunks easily.
-   **Steganography:** Embed and retrieve secret messages hidden within valid PNG chunks.
-   **Chunk Validation:** Ensure chunk types conform to PNG specifications.
-   **Performance & Safety:** Built with Rust for speed and memory safety.
-   **File Management:** Includes commands for file conversion, download, and deletion.

---

## 🏗️ Understanding PNG File Structure

A PNG (Portable Network Graphics) file is composed of a signature followed by a series of **chunks**. Each chunk contains:

-   **Length (4 bytes):** An unsigned integer specifying the number of bytes in the chunk's data field.
-   **Chunk Type (4 bytes):** A 4-character ASCII string identifying the chunk's purpose (e.g., `IHDR`, `PLTE`, `IDAT`, `IEND`). Ancillary chunks (like those used for hidden messages) often use lowercase letters (e.g., `maTt`, `ruSt`).
-   **Chunk Data (variable length):** The actual data associated with the chunk type. Its length must match the value in the Length field.
-   **CRC (4 bytes):** A Cyclic Redundancy Check value calculated over the Chunk Type and Chunk Data fields to detect errors.

---

## 🛠️ Installation & Setup

### Prerequisites

-   **Rust & Cargo:** Ensure you have the Rust programming language and its package manager, Cargo, installed. You can get them from [rustup.rs](https://rustup.rs/).

### Steps

1.  **Clone the Repository:**
    ```sh
    git clone [https://github.com/yourusername/ImgMod.git](https://github.com/yourusername/ImgMod.git) # Replace with your actual repo URL
    cd ImgMod
    ```

2.  **Build the Project:**
    ```sh
    cargo build --release
    ```
    This compiles the project in release mode for optimal performance. The executable will be located in `./target/release/`.

3.  **Install Globally (Optional but Recommended):**
    To make the `ImgMod` command accessible from anywhere in your terminal, install it using Cargo:
    ```sh
    cargo install --path .
    ```
    Alternatively, you can add Cargo's binary directory (usually `~/.cargo/bin` on Linux/macOS or `%USERPROFILE%\.cargo\bin` on Windows) to your system's `PATH` environment variable.

---

## 💻 Usage (CLI Examples)

The `ImgMod` tool provides several commands for interacting with PNG files and managing images.

### Core Steganography Commands

These commands allow you to hide and retrieve messages within PNG chunks.

**1. Encode a Hidden Message**

Embed a secret message into a PNG file using a specified chunk type.

```sh
ImgMod encode ./path/to/your/image.png <chunk_type> "Your secret message here"
```

*Example:*
```sh
ImgMod encode ./images/Gengar.png maTt "Hello World from Rust!"
```
This command adds a custom chunk named `maTt` containing the message "Hello World from Rust!" to the `Gengar.png` file.
*(Note: Valid custom chunk types should follow PNG naming conventions. Ancillary chunks often use lowercase letters, e.g., `ruSt`, `secR`, `maTt`)*

**2. Print All Chunks**

List all chunks present in a PNG file, showing their type and data length.

```sh
ImgMod print ./path/to/your/image.png
```

*Example:*
```sh
ImgMod print ./images/Gengar.png
```

*Example Output:*
```
File: ./images/Gengar.png, Size: 33054
  chunk#0{ chunk_type: IHDR, data_length: 13}
  chunk#1{ chunk_type: iCCP, data_length: 2457}
  chunk#2{ chunk_type: pHYs, data_length: 9}
  ...
  chunk#13{ chunk_type: maTt, data_length: 22}  # The hidden message chunk
  chunk#14{ chunk_type: IEND, data_length: 0}
```

**3. Decode a Hidden Message**

Extract a hidden message associated with a specific chunk type from a PNG file.

```sh
ImgMod decode ./path/to/your/image.png <chunk_type>
```

*Example:*
```sh
ImgMod decode ./images/Gengar.png maTt
```

*Example Output:*
```
msg: Hello World from Rust!
```

**4. Remove a Chunk**

Delete a specific chunk (and any hidden message it contains) from the PNG file.

```sh
ImgMod remove ./path/to/your/image.png <chunk_type>
```

*Example:*
```sh
ImgMod remove ./images/Gengar.png maTt
```
This removes the `maTt` chunk from `Gengar.png`.

### `manage` Subcommand

The `manage` subcommand offers additional file utilities.

**1. Delete a File**

Remove an image file from your disk.

```sh
ImgMod manage delete ./path/to/file/to_delete.png
```

*Example:*
```sh
ImgMod manage delete ./images/temporary_copy.png
```

**2. Download an Image**

Download an image from a URL and save it locally.

```sh
ImgMod manage download <URL> <output_filename>
```

*Example:*
```sh
ImgMod manage download [https://www.rust-lang.org/logos/rust-logo-512x512.png](https://www.rust-lang.org/logos/rust-logo-512x512.png) rust_logo.png
```
This downloads the Rust logo and saves it as `rust_logo.png` in the current directory.

**3. Convert Image Format**

Convert an image file to a different format. The file extension will be updated automatically.

*Flags:*
- `-j`: Convert to JPEG (`.jpg`)
- `-p`: Convert to PNG (`.png`)
- `-t`: Convert to TIFF (`.tiff`)
- `-w`: Convert to WebP (`.webp`)

*Usage:*
```sh
ImgMod manage convert <flag> <input_image_path>
```

*Examples:*
```sh
# Convert to JPEG
ImgMod manage convert -j ./images/my_image.png

# Convert to WebP
ImgMod manage convert -w ./images/photo.tiff
```

---

## 🚧 Development Status

This project is actively being developed as a port from C++ to Rust. Features may change, and contributions are welcome!
