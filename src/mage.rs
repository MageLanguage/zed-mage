use serde::{Deserialize, Serialize};
use serde_json::Value;
use zed_extension_api::{
    self as zed, register_extension, settings::LspSettings, Command, ContextServerId,
    DebugAdapterBinary, DebugConfig, DebugRequest, DebugScenario, DebugTaskDefinition, Extension,
    LanguageServerId, Result, StartDebuggingRequestArguments,
    StartDebuggingRequestArgumentsRequest, Worktree,
};

const DEBUG_ADAPTER_NAME: &str = "mage";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MageDebugConfig {
    request: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    program: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    args: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cwd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    stop_on_entry: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    process_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    port: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<String>,
}

struct MageExtension;

impl Extension for MageExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let path = worktree
            .which("mage-ls")
            .ok_or_else(|| "Can't find mage executable".to_string())?;

        Ok(Command {
            command: path,
            args: vec![],
            env: vec![],
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        Ok(LspSettings::for_worktree(language_server_id.as_ref(), worktree)?.settings)
    }

    fn get_dap_binary(
        &mut self,
        adapter_name: String,
        config: DebugTaskDefinition,
        _user_provided_debug_adapter_path: Option<String>,
        worktree: &Worktree,
    ) -> zed::Result<DebugAdapterBinary, String> {
        if adapter_name != DEBUG_ADAPTER_NAME {
            return Err(format!(
                "Cannot create binary for adapter: {}",
                adapter_name
            ));
        }

        let path = worktree
            .which("mage-da")
            .ok_or_else(|| "Can't find mage executable".to_string())?;

        let configuration = config.config.to_string();
        let parsed_config: MageDebugConfig = serde_json::from_str(&configuration).map_err(|e| {
            format!(
                "Failed to parse debug configuration: {}. Expected Mage configuration format.",
                e
            )
        })?;

        let request = match parsed_config.request.as_str() {
            "launch" => StartDebuggingRequestArgumentsRequest::Launch,
            "attach" => StartDebuggingRequestArgumentsRequest::Attach,
            other => {
                return Err(format!(
                    "Invalid 'request' value: '{}'. Expected 'launch' or 'attach'",
                    other
                ))
            }
        };

        Ok(DebugAdapterBinary {
            command: Some(path),
            arguments: vec![],
            envs: vec![],
            cwd: Some(parsed_config.cwd.unwrap_or_else(|| worktree.root_path())),
            connection: None,
            request_args: StartDebuggingRequestArguments {
                configuration,
                request,
            },
        })
    }

    fn dap_request_kind(
        &mut self,
        adapter_name: String,
        config: Value,
    ) -> zed::Result<StartDebuggingRequestArgumentsRequest, String> {
        if adapter_name != DEBUG_ADAPTER_NAME {
            return Err(format!("Unknown adapter: {}", adapter_name));
        }

        match config.get("request").and_then(|v| v.as_str()) {
            Some("launch") => Ok(StartDebuggingRequestArgumentsRequest::Launch),
            Some("attach") => Ok(StartDebuggingRequestArgumentsRequest::Attach),
            Some(other) => Err(format!(
                "Invalid 'request' value: '{}'. Expected 'launch' or 'attach'",
                other
            )),
            None => Err(
                "Debug configuration missing required 'request' field. Must be 'launch' or 'attach'"
                    .to_string(),
            ),
        }
    }

    fn dap_config_to_scenario(
        &mut self,
        config: DebugConfig,
    ) -> zed::Result<DebugScenario, String> {
        match config.request {
            DebugRequest::Launch(launch) => {
                let adapter_config = MageDebugConfig {
                    request: "launch".to_string(),
                    program: Some(launch.program),
                    args: if launch.args.is_empty() {
                        None
                    } else {
                        Some(launch.args)
                    },
                    cwd: launch.cwd,
                    stop_on_entry: config.stop_on_entry,
                    process_id: None,
                    port: None,
                    host: None,
                };

                let config_json = serde_json::to_string(&adapter_config)
                    .map_err(|e| format!("Failed to serialize launch config: {}", e))?;

                Ok(DebugScenario {
                    label: config.label,
                    adapter: config.adapter,
                    build: None,
                    config: config_json,
                    tcp_connection: None,
                })
            }
            DebugRequest::Attach(attach) => {
                let adapter_config = MageDebugConfig {
                    request: "attach".to_string(),
                    program: None,
                    args: None,
                    cwd: None,
                    stop_on_entry: config.stop_on_entry,
                    process_id: attach.process_id.map(|id| id as i32),
                    port: None,
                    host: None,
                };

                let config_json = serde_json::to_string(&adapter_config)
                    .map_err(|e| format!("Failed to serialize attach config: {}", e))?;

                Ok(DebugScenario {
                    label: config.label,
                    adapter: config.adapter,
                    build: None,
                    config: config_json,
                    tcp_connection: None,
                })
            }
        }
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &zed::Project,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: "".to_string(),
            args: vec![],
            env: vec![],
        })
    }
}

register_extension!(MageExtension);
