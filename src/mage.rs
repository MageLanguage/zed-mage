use serde_json::Value;
use zed_extension_api::{
    register_extension, settings::LspSettings, /* Command, */ Extension, LanguageServerId,
    Result, Worktree,
};

struct MageExtension;

impl Extension for MageExtension {
    fn new() -> Self {
        Self
    }

    // fn language_server_command(
    //     &mut self,
    //     _: &LanguageServerId,
    //     worktree: &Worktree,
    // ) -> Result<Command> {
    //     let path = match worktree.which("mage-rs") {
    //         Some(path) => path,
    //         None => return Err("Can't find mage executable".to_string()),
    //     };

    //     Ok(Command {
    //         command: path,
    //         args: Vec::from(["language-server".to_string()]),
    //         env: Vec::new(),
    //     })
    // }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

register_extension!(MageExtension);
