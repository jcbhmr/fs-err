![🚧 Under construction 👷‍♂️](https://i.imgur.com/LEP2R3N.png)

# fs-err

📂 Better `std::fs` errors for Rust

<table align=center><td>

```rs
let file = File::open("file.txt")?;
```

<td>

```diff
-No such file or directory (os error 2)
+file.txt: No such file or directory (os error 2)
```

<tr><td>

```rs
remove_dir_all("dist")?;
```

<td>

```diff
-Permission denied (os error 13)
+dist/assets/special.txt: Permission denied (os error 13)
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
