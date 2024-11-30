//use std::env;
use zed_extension_api::{self as zed, Command, ContextServerId, Project, Result};

//const PACKAGE_NAME: &str = "@zeddotdev/postgres-context-server";
//const PACKAGE_VERSION: &str = "0.1.2";
//const SERVER_PATH: &str = "node_modules/@zeddotdev/postgres-context-server/index.mjs";
const SERVER_DEV_PATH: &str = "/Users/wes/repos/github/g0t4/mcp-server-commands/build/index.js";

struct CommandsModelContextExtension;

// TODO consider using shell type setting (or other settings), see postgres-context-server for using settings

impl zed::Extension for CommandsModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        //let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        //if version.as_deref() != Some(PACKAGE_VERSION) {
        //    zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        //}
        println!("commands model context extension starting");


        Ok(Command {
            command: "node".to_string(),
            args: vec![
                SERVER_DEV_PATH.to_string(),
                //env::current_dir()
                //    .unwrap()
                //    .join(SERVER_PATH)
                //    .to_string_lossy()
                //    .to_string(),
            ],
            env: vec![
            //    ("DATABASE_URL".into(), settings.database_url)
            ],
        })
    }
}

zed::register_extension!(CommandsModelContextExtension);

