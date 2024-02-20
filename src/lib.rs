#![doc = include_str!("../README.md")]

use std::path::PathBuf;

/// Error type.
pub type Error = Box<dyn std::error::Error>;

/// Result type with a Boxed error type, for easy chaining of errors in the PyEnv struct
pub type PyResult<T> = Result<T, Box<dyn std::error::Error>>;

/// A Python environment that can install packages and execute code.
pub struct PyEnv {
    path: PathBuf,
    std_out: Box<dyn Fn(&str)>,
    std_err: Box<dyn Fn(&str)>,
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
        std_out: impl Fn(&str) + 'static,
        std_err: impl Fn(&str) + 'static,
    ) -> Self {
        let path = path.into();
        let persistent = true;
        let std_out = Box::new(std_out) as Box<dyn Fn(&str)>;
        let std_err = Box::new(std_err) as Box<dyn Fn(&str)>;
        Self { path, std_out, std_err, persistent }
    }

    /// Constructor inheriting default stdout and stderr; use `new()` to customize the streams.
    pub fn at(path: impl Into<PathBuf>) -> Self {
        let std_out = |line: &str| println!("{}", line);
        let std_err = |line: &str| eprintln!("{}", line);
        Self::new(path, std_out, std_err)
    }

    fn stream_command(&self, command: &mut std::process::Command) -> Result<bool, Box<dyn std::error::Error>> {
        use std::io::{BufReader, BufRead};

        let mut command = command
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        command.stdout.as_mut().map(|stdout| {
            let reader = BufReader::new(stdout);
            reader.lines().for_each(|line| {
                if let Ok(line) = line {
                    (self.std_out)(&line);
                }
            });
        });
        command.stderr.as_mut().map(|stderr| {
            let reader = BufReader::new(stderr);
            reader.lines().for_each(|line| {
                if let Ok(line) = line {
                    (self.std_err)(&line);
                }
            });
        });

        let status = command.wait()?;
        Ok(status.success())
    }

    /// Installs a package in the PyEnv, returning itself to easily chain dependencies.
    pub fn install(&self, package_name: &str) -> PyResult<&Self> {
        self.stream_command(std::process::Command::new("python")
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
        )?;
        Ok(&self)
    }
    
    /// Executes arbitrary code in the PyEnv, returning itself to easily chain runs.
    pub fn execute(&self, code: &str) -> PyResult<&Self> {
        std::env::set_var("PYTHONPATH", self.path.join("site-packages"));
        self.stream_command(
            std::process::Command::new("python")
            .args(["-c", code])
        )?;
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
