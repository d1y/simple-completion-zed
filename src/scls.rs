use zed_extension_api::{self as zed, Result,LanguageServerId};

struct SclsExtension {

}

impl zed::Extension for SclsExtension {
    fn new() -> Self {
        Self {
        }
    }

    fn language_server_command(
        &mut self,
        _: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("simple-completion-language-server")
            .ok_or_else(|| "Must install https://github.com/d1y/scls".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: vec![
                // ("RUST_LOG".to_string(), "debug,run=debug".to_string()),
                // ("LOG_FILE".to_string(), "~/Downloads/run.log".to_string())
            ],
        })
    }
}

zed::register_extension!(SclsExtension);
