use zed_extension_api as zed;
use zed_extension_api::{CodeLabel, Command, ContextServerId, KeyValueStore, LanguageServerId, Project, SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput, Worktree};
use zed_extension_api::serde_json::Value;

struct ZedExtensionTest {
    idk: u32,
}

impl zed::Extension for ZedExtensionTest {
    fn new() -> Self
    where
        Self: Sized
    {
        Self {
            idk: 0xdeadbeef,
        }
    }

    fn language_server_command(&mut self, _language_server_id: &LanguageServerId, _worktree: &Worktree) -> zed_extension_api::Result<Command> {
        todo!()
    }

    fn language_server_initialization_options(&mut self, _language_server_id: &LanguageServerId, _worktree: &Worktree) -> zed_extension_api::Result<Option<Value>> {
        todo!()
    }

    fn language_server_workspace_configuration(&mut self, _language_server_id: &LanguageServerId, _worktree: &Worktree) -> zed_extension_api::Result<Option<Value>> {
        todo!()
    }

    fn label_for_completion(&self, _language_server_id: &LanguageServerId, _completion: zed_extension_api::wit::Completion) -> Option<CodeLabel> {
        todo!()
    }

    fn label_for_symbol(&self, _language_server_id: &LanguageServerId, _symbol: zed_extension_api::wit::Symbol) -> Option<CodeLabel> {
        todo!()
    }

    fn complete_slash_command_argument(&self, _command: SlashCommand, _args: Vec<String>) -> zed_extension_api::Result<Vec<SlashCommandArgumentCompletion>, String> {
        todo!()
    }

    fn run_slash_command(&self, _command: SlashCommand, _args: Vec<String>, _worktree: Option<&Worktree>) -> zed_extension_api::Result<SlashCommandOutput, String> {
        todo!()
    }

    fn context_server_command(&mut self, _context_server_id: &ContextServerId, _project: &Project) -> zed_extension_api::Result<Command> {
        todo!()
    }

    fn suggest_docs_packages(&self, _provider: String) -> zed_extension_api::Result<Vec<String>, String> {
        todo!()
    }

    fn index_docs(&self, _provider: String, _package: String, _database: &KeyValueStore) -> zed_extension_api::Result<(), String> {
        todo!()
    }
}

zed::register_extension!(ZedExtensionTest);