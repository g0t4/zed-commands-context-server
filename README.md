FYI I think I can use mcp servers w/o an extension too... just configure in zed's config file under context_servers. Pass command info. 

Extension seems to be the way to go if you have to install the server (i.e. npm)


FYI zed only supports prompts from MCP servers, so I need to wait on tools use. 
- See https://modelcontextprotocol.io/clients#feature-support-matrix
- https://zed.dev/blog/mcp (hints tool use is a future feature)


So, basically prompts are for injecting reusable prompts (prompt templates), i.e. the /pg-schema injects table(s) schemas from a configured postgres db. 

I could make a propmt template for commands... where it takes a command and generates a propmt with the stderr/out like tool use and have that inline, would be an interesting way to quickly include command output instead of tool use where the model is deciding to use the commands tool to run smth, this prompt approach would be the end user deciding to run/include the command output in the prompt...
