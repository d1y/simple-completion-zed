use zed_extension_api::{self as zed, Result};

struct SclsExtension {

}

impl zed::Extension for SclsExtension {
    fn new() -> Self {
        Self {
        }
    }

    fn language_server_command(
        &mut self,
        _: zed::LanguageServerConfig,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("simple-completion-language-server")
            .ok_or_else(|| "Must install https://github.com/estin/simple-completion-language-server".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(SclsExtension);
