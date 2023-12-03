//! Subcommand modules for the `mdbook` binary.

pub mod build;
pub mod clean;
#[cfg(feature = "gen-syntax-cache")]
pub mod gen_syntax_cache;
pub mod command_prelude;
pub mod init;
#[cfg(feature = "serve")]
pub mod serve;
pub mod test;
#[cfg(feature = "watch")]
pub mod watch;
