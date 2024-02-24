# py_env

A Rust library to run Python scripts and install dependencies within a given environment path.

## Table of Contents

* [Installation](#installation)
* [Usage](#usage)
* [Contributing](#contributing)
* [License](#license)

## Installation

Simply add the library as a dependency in your Cargo.toml as follows, and invoke via the [usage instructions](#usage).
```toml
[dependencies]
py_env = "2.0.0"
```

## Usage

### Creating a Python Environment

This library uses a very simple syntax to run Python scripts. To create a Python environment, simply run `PyEnv::at(PathBuf)`.

```rust
use py_env::PyEnv;

let env = PyEnv::at("./py_test");
```

### Running Arbitrary Code

```rust
use py_env::PyEnv;

PyEnv::new("./py_test")
    .execute("print('hello world')")
    .expect("Failed to execute code");
```

### Installing Python Dependencies

The following code installs numpy into the `./py_test` directory's site-packages and uses it in executed code.

```rust
use py_env::PyEnv;

PyEnv::new("./py_test")
    .execute("import numpy; a = numpy.arange(15).reshape(3, 5); print(a.shape)")
    .expect("Failed to execute code");
```

### Making Environments Impersistent

The following code deletes the python environment off of the disk once it's done running.

```rust
use py_env::PyEnv;

PyEnv::new("./py_test")
    .persistent(false);
```

### Using the `try_` Unwrappers

The `try_install()` and `try_execute()` unwrapper functions panic upon errors being returned from their inner `install()` and `execute()` functions, which fail only if there is an error spawning the Python commands needed to run or upon waiting for them to finish running. Given that should only really happen if you don't have Python installed, we have provided these functions for convenience, but warn against their use in production code over handling the errors manually.

```rust
use py_env::PyEnv;

PyEnv::new("./py_test")
    .persistent(false)
    .try_execute("import numpy; a = numpy.arange(15).reshape(3, 5); print(a.shape)");
```

## Contributing

PRs are always welcome and will be reviewed when I see them. Code is released under the MIT License ([see below](#license)), and as such forking this repo to add features you'd like to see implemented would be greatly appreciated.

## License

This code is licensed under the [MIT License](https://github.com/uptudev/py_shell/blob/main/LICENSE).
