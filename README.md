# py_shell

A Rust library to run Python scripts and install dependencies within a given environment path.

## Table of Contents

* [Installation](#installation)
* [Usage](#usage)
* [Contributing](#contributing)
* [License](#license)

## Installation

This library requires no dependencies and is not on crates.io, and as such the easiest method is to just copy the `lib.rs` file into your project, rename it to `py_shell.rs` or some other name, and import it from your code.

## Usage

### Creating a Python Environment

This library uses a very simple syntax to run Python scripts. To create a Python environment, simply run `PyEnv::at(PathBuf)`.

```rust
let path = "./py_test";
let env = PyEnv::at(path);
```

### Running Arbitrary Code

```rust
let path = "./py_test";
PyEnv::at(path)
    .execute("print('hello world')");
```

### Installing Python Dependencies

The following code installs numpy into the `./py_test` directory's site-packages and uses it in executed code.

```rust
let path = "./py_test";
PyEnv::at(path)
    .install("numpy");
    .execute("a = np.arange(15).reshape(3, 5); print(a.shape)");
```

## Contributing

This was made as a code bounty, and as such is not a maintained project, but PRs are always welcome and will be reviewed when I see them.

## License

This code is licensed under the [MIT License](https://github.com/uptudev/py_shell/blob/main/LICENSE).
