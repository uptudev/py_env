#![doc = include_str!("../README.md")]
mod pipette;
use pyo3::prelude::*;
use std::path::PathBuf;

/// Error type.
pub type Error = Box<dyn std::error::Error>;

/// Result type with a Boxed error type, for easy chaining of errors in the PyEnv struct
pub type PyResult<T> = Result<T, Box<dyn std::error::Error>>;

/// A Python environment that can install packages and execute code.
pub struct PyEnv {
    path: PathBuf,
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
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let persistent = true;
        Self { path, persistent }
    }

    /// Executes arbitrary code in the PyEnv, returning itself to easily chain runs.
    pub fn execute(&self, code: &str) -> PyResult<&Self> {
        pipette::get_dependencies(code, &self.path)?;
        Python::with_gil(|py| -> pyo3::PyResult<()> {
            let syspath: &pyo3::types::PyList = py.import("sys")?.getattr("path")?.downcast()?;
            syspath.insert(0, &self.path.join("site-packages"))?;
            py.run(code, None, None)
        })?;
        Ok(&self)
    }

    /// An unwrapped `execute()` run, which panics upon failure. See `execute()` for the version
    /// which returns a PyResult.
    pub fn try_execute(&self, code: &str) -> &Self {
        self.execute(code)
            .expect("Error executing code");
        &self
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
    fn test_run() -> PyResult<()> {
        PyEnv::new("./py_test/run")
            .execute("print('hello world')")?;
        Ok(())
    }

    #[test]
    fn test_install_run() -> PyResult<()> {
        PyEnv::new("./py_test/install_run")
            .execute("import faker; print(faker.Faker().name())")?;
        Ok(())
    }

    #[test]
    fn test_impersistence() -> PyResult<()> {
        PyEnv::new("./py_test/impersistence")
            .persistent(false)
            .execute("import faker; print(faker.Faker().name())")?;
        Ok(())
    }

    #[test]
    fn test_unwrapped_funcs() {
        PyEnv::new("./py_test/unwrapped_funcs")
            .try_execute("import faker; print(faker.Faker().name())");
    }
}
