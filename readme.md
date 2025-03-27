# 🖼️ Image Encoder & Decoder with Hidden Messages

A **Rust** implementation of a Image encoder and decoder with support for **hidden messages** inside PNG chunks so far. Originally written in **C++**, now ported to **Rust**. (Still in progress)

---

## 🚀 Features

- 📂 **Read & write image files** with chunk-level manipulation.
- 🧩 **Extract and modify image chunks dynamically.**
- 🔒 **Embed and retrieve secret messages hidden in image chunks.**
- ✅ **Validate chunk types** to ensure they conform to image specifications.
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
git clone https://github.com/yourusername/ImgMod.git
cd ImgMod

# 2️⃣ Build the project
cargo build --release
```

Make sure the compiled binary is accessible from anywhere:
```sh
cp ./target/release/ImgMod ~/.cargo/bin
```
Or add the path to your `$PATH` environment variable.

---

## 🧪 CLI Examples

### 📅 Encode a Hidden Message
Embed a secret message into a PNG file using a custom chunk type:

```sh
ImgMod encode ./Gengar.png maTt "Hello World"
```

This adds a custom chunk `maTt` with the message `Hello World` into the PNG file.
you can use any chunk name though along if it is valid like ruSt or RuSt or even just doOr along if it is a valid Chunk Type
---

### 🔍 Print All Chunks
List all the chunks currently inside the PNG file:

```sh
ImgMod print ./Gengar.png
```
**Example Output:**
```
File: ./Gengar.png, Size: 33054
  chunk#0{ chunk_type: IHDR, data_length: 13}
  chunk#1{ chunk_type: iCCP, data_length: 2457}
  ...
  chunk#13{ chunk_type: maTt, data_length: 11}
```

---

### 🔓 Decode a Message
Read a message from a custom chunk type:

```sh
ImgMod decode ./Gengar.png maTt
```
**Example Output:**
```
msg: Hello World
```

---

### ❌ Remove a Chunk
Remove a chunk with a specific type from the PNG file:

```sh
ImgMod remove ./Gengar.png maTt
```
This deletes the `maTt` chunk (and the message) from the image.

---

### 🔀 After Removal 
```sh
ImgMod print ./Gengar.png
```
You’ll notice the custom chunk is no longer present in the output.

---
