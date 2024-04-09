# fs-err

📂 Better `std::fs` errors for Rust

<table align=center><td>

```rs
let file = File::open("file.txt")?;
```

<tr><td>

```diff
-No such file or directory (os error 2)
+Failed to open `file.txt`
+    caused by: No such file or directory (os error 2)
```

</table>

## Installation

```sh
cargo add fs-err
```

## Usage

```rs
use fs_err::fs;
```
