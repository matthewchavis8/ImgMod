# 🖼️ PNG Encoder & Decoder with Hidden Messages

A **Rust** implementation of a PNG encoder and decoder with support for **hidden messages** inside PNG chunks. Originally written in **C++**, now ported to **Rust** for improved performance and safety.

---

## 🚀 Features

- 📂 **Read & write PNG files** with chunk-level manipulation.
- 🧩 **Extract and modify PNG chunks dynamically.**
- 🔒 **Embed and retrieve secret messages hidden in PNG chunks.**
- ✅ **Validate chunk types** to ensure they conform to PNG specifications.
- 🛠️ **Built with Rust** for performance and safety.

---

## 🏗️ PNG File Structure

A PNG file consists of multiple **chunks**, each containing:

- 📏 **Length (4 bytes)** - The number of bytes in the chunk's data.
- 🔤 **Chunk Type (4 bytes)** - A 4-character string identifying the chunk.
- 📄 **Chunk Data (variable)** - The actual content of the chunk.
- ✅ **CRC (4 bytes)** - A checksum for error detection.

---

## 🛠️ How to Build & Run

Get started by cloning and running the project.

```sh
# 1️⃣ Clone the repository
git clone https://github.com/yourusername/png-encoder-decoder.git
cd png-encoder-decoder

# 2️⃣ Build the project
cargo build --release

# 3️⃣ Run the encoder/decoder
cargo run -- [options] <input_file>
```

