use std::io::{stdout, stderr, Stdout, Stderr};
use std::path::PathBuf;

// pandora's box of self-mutating code
pub struct PyEnv {
    path: PathBuf,
    std_out: Box<dyn Fn() -> Stdout>,
    std_err: Box<dyn Fn() -> Stderr>,
    persistent: bool,
} 

impl Drop for PyEnv {
    fn drop(&mut self) {
        if !self.persistent {
            std::fs::remove_dir_all(&self.path).unwrap();
        }
    }
}

impl PyEnv {
    // Constructor for piping stdout and stderr to a custom stream.
    // Use `at()` if you want to inherit the streams.
    pub fn new(
        path: PathBuf, 
        std_out: Box<dyn Fn() -> Stdout>,
        std_err: Box<dyn Fn() -> Stderr>,
    ) -> Self {
        Self {path, std_out, std_err, persistent: true}
    }

    /// Constructor inheriting default stdout and stderr; use `new()` to customize the streams.
    pub fn at(path: &str) -> Self {
        let path = PathBuf::from(path);
        Self::new(path, Box::new(stdout), Box::new(stderr))
    }

    /// Installs a package in the PyEnv, returning itself to easily chain dependencies.
    pub fn install(&self, package_name: &str) -> &Self {
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
                    .unwrap()])
            .stdout((self.std_out)())
            .stderr((self.std_err)())
            .spawn()
            .expect("Error installing package");
        handle.wait().unwrap();
        &self
    }
    
    /// Executes arbitrary code in the PyEnv, returning itself to easily chain runs.
    pub fn execute(&self, code: &str) -> &Self {
        std::env::set_var("PYTHONPATH", self.path.join("site-packages"));
        let mut handle = std::process::Command::new("python")
            .args([
                "-c",
                code
            ])
            .stdout((self.std_out)())
            .stderr((self.std_err)())
            .spawn()
            .expect("Error running code");
        handle.wait().unwrap();
        &self
    }

    /// Makes the environment impersistent beyond the PyEnv, deleting it upon dropping
    pub fn make_impersistent(&mut self) -> &Self {
        self.persistent = false;
        self
    }

    /// Makes the environment persistent beyond the PyEnv, keeping it upon dropping (default)
    pub fn make_persistent(&mut self) -> &Self {
        self.persistent = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const DIR: &'static str = "./py_test";

    #[test]
    fn test_install() {
        PyEnv::at(DIR)
            .install("faker");
    }

    #[test]
    fn test_run() {
        PyEnv::at(DIR)
            .execute("print('hello world')");
    }

    #[test]
    fn test_install_run() {
        PyEnv::at(DIR)
            .install("faker")
            .execute("import faker; print(faker.Faker().name())");
    }

    #[test]
    fn test_impersistence() {
        PyEnv::at(DIR)
            .make_impersistent()
            .install("faker");
    }
}
