#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use zed_extension_api::{self as zed, Result};

struct LotusExtension;

impl zed::Extension for LotusExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<zed_extension_api::Command> {
        let command = find_lopus(worktree)?;
        Ok(zed::Command {
            command,
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(LotusExtension);

pub fn find_lopus(worktree: &zed::Worktree) -> Result<String> {
    if let Some(path) = worktree.which("lopus") {
        return Ok(path);
    }
    Err("Could not find 'lopus' in the worktree.".into())
}
