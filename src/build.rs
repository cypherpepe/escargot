use std::ffi;
use std::process;

use cargo::Cargo;
use cargo::CURRENT_TARGET;
use error::*;
use msg::*;
use run::CargoRun;

/// The `build` subcommand.
///
/// # Example
///
/// ```rust
/// escargot::CargoBuild::new()
///     .bin("bin_fixture")
///     .current_release()
///     .current_target()
///     .exec()
///     .unwrap();
/// ```
pub struct CargoBuild {
    cmd: process::Command,
    bin: bool,
    example: bool,
}

impl CargoBuild {
    /// Shortcut to create a `build` subcommand.
    ///
    /// See also [`Cargo`].
    ///
    /// # Example
    ///
    /// ```rust
    /// escargot::CargoBuild::new()
    ///     .exec()
    ///     .unwrap();
    /// ```
    ///
    /// [`Cargo`]: struct.Cargo.html
    pub fn new() -> Self {
        Cargo::new().build()
    }

    pub(crate) fn with_command(cmd: process::Command) -> Self {
        Self {
            cmd,
            bin: false,
            example: false,
        }
    }

    /// Build only `name` binary.
    ///
    /// # Example
    ///
    /// ```rust
    /// escargot::CargoBuild::new()
    ///     .bin("bin_fixture")
    ///     .exec()
    ///     .unwrap();
    /// ```
    pub fn bin<S: AsRef<ffi::OsStr>>(mut self, name: S) -> Self {
        self.bin = true;
        self.arg("--bin").arg(name)
    }

    /// Build only `name` example.
    ///
    /// # Example
    ///
    /// ```rust
    /// escargot::CargoBuild::new()
    ///     .example("example_fixture")
    ///     .exec()
    ///     .unwrap();
    /// ```
    pub fn example<S: AsRef<ffi::OsStr>>(mut self, name: S) -> Self {
        self.example = true;
        self.arg("--example").arg(name)
    }

    /// Build artifacts in release mode, with optimizations.
    pub fn release(self) -> Self {
        self.arg("--release")
    }

    /// Build artifacts in release mode if the current process has, with optimizations.
    #[cfg(debug_assertions)]
    pub fn current_release(self) -> Self {
        self
    }

    /// Build artifacts in release mode if the current process has, with optimizations.
    #[cfg(not(debug_assertions))]
    pub fn current_release(self) -> Self {
        self.release()
    }

    /// Build for the target triple.
    pub fn target<S: AsRef<ffi::OsStr>>(self, triplet: S) -> Self {
        self.arg("--target").arg(triplet)
    }

    /// Build for the current process' triple.
    pub fn current_target(self) -> Self {
        self.target(CURRENT_TARGET)
    }

    /// Manually pass an argument that is unsupported.
    ///
    /// Caution: Passing in `--` can throw off the API.
    pub fn arg<S: AsRef<ffi::OsStr>>(mut self, arg: S) -> Self {
        self.cmd.arg(arg);
        self
    }

    /// Build the configured target, returning compiler messages.
    pub fn exec(self) -> CargoResult<MessageItr> {
        MessageItr::from_command(self.cmd)
    }

    /// Provide a proxy for running the built target.
    ///
    /// # Example
    ///
    /// ```rust
    /// let run = escargot::CargoBuild::new()
    ///     .bin("bin_fixture")
    ///     .current_release()
    ///     .current_target()
    ///     .run()
    ///     .unwrap();
    /// println!("artifact={}", run.path().display());
    /// ```
    pub fn run(self) -> CargoResult<CargoRun> {
        let msgs = MessageItr::from_command(self.cmd)?;
        CargoRun::with_messages(msgs, self.bin, self.example)
    }
}

impl Default for CargoBuild {
    fn default() -> Self {
        Self::new()
    }
}