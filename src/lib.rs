#![doc = include_str!("../README.md")]

use std::io::{stdout, stderr, Stdout, Stderr};
use std::path::PathBuf;

/// Error type.
pub type Error = Box<dyn std::error::Error>;

/// Result type with a Boxed error type, for easy chaining of errors in the PyEnv struct
pub type PyResult<T> = Result<T, Box<dyn std::error::Error>>;

/// A Python environment that can install packages and execute code.
pub struct PyEnv {
    path: PathBuf,
    std_out: Box<dyn Fn() -> Stdout>,
    std_err: Box<dyn Fn() -> Stderr>,
    persistent: bool,
} 

impl Drop for PyEnv {
    fn drop(&mut self) {
        if !self.persistent {
            if let Err(e) = std::fs::remove_dir_all(&self.path) {
                eprintln!("Error deleting PyEnv at {}, cause: {}", self.path.display(), e);
            }
        }
    }
}

impl PyEnv {
    /// Constructor for piping stdout and stderr to a custom stream.
    /// Use `at()` if you want to inherit the streams.
    pub fn new(
        path: impl Into<PathBuf>, 
        std_out: Box<dyn Fn() -> Stdout>,
        std_err: Box<dyn Fn() -> Stderr>,
    ) -> Self {
        let path = path.into();
        let persistent = true;
        Self { path, std_out, std_err, persistent }
    }

    /// Constructor inheriting default stdout and stderr; use `new()` to customize the streams.
    pub fn at(path: impl Into<PathBuf>) -> Self {
        Self::new(path, Box::new(stdout), Box::new(stderr))
    }

    /// Installs a package in the PyEnv, returning itself to easily chain dependencies.
    pub fn install(&self, package_name: &str) -> PyResult<&Self> {
        let mut handle = std::process::Command::new("python")
            .args([
                "-m", 
                "pip", 
                "install",
                package_name,
                "--target",
                self.path
                    .join("site-packages")
                    .as_os_str()
                    .to_str()
                    .ok_or("Invalid path")?])
            .stdout((self.std_out)())
            .stderr((self.std_err)())
            .spawn()?;
        handle.wait()?;
        Ok(&self)
    }
    
    /// Executes arbitrary code in the PyEnv, returning itself to easily chain runs.
    pub fn execute(&self, code: &str) -> PyResult<&Self> {
        std::env::set_var("PYTHONPATH", self.path.join("site-packages"));
        let mut handle = std::process::Command::new("python")
            .args(["-c", code])
            .stdout((self.std_out)())
            .stderr((self.std_err)())
            .spawn()?;
        handle.wait()?;
        Ok(&self)
    }

    /// Makes the environment impersistent beyond the PyEnv, deleting it upon dropping
    pub fn persistent(&mut self, persistent: bool) -> &Self {
        self.persistent = persistent;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install() -> PyResult<()> {
        PyEnv::at("./py_test/install")
            .install("faker")?;
        Ok(())
    }

    #[test]
    fn test_run() -> PyResult<()> {
        PyEnv::at("./py_test/run")
            .execute("print('hello world')")?;
        Ok(())
    }

    #[test]
    fn test_install_run() -> PyResult<()> {
        PyEnv::at("./py_test/install_run")
            .install("faker")?
            .execute("import faker; print(faker.Faker().name())")?;
        Ok(())
    }

    #[test]
    fn test_impersistence() -> PyResult<()> {
        PyEnv::at("./py_test/impersistence")
            .persistent(false)
            .install("faker")?;
        Ok(())
    }
}
