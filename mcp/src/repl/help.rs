use colored::Colorize;

pub fn print_banner() {
    println!("{}", "mcpcs-client REPL".cyan().bold());
    println!("{}", "Commands:".yellow());
    println!("  {}              - Reload configuration from ~/.mcpcsrs/mcps/*.json", "/reload".green());
    println!("  {}            - List connected MCP servers", "/list mcp".green());
    println!("  {}           - List available tools from all servers", "/list tool".green());
    println!("  {}       - List available resources from all servers", "/list resource".green());
    println!("  {}        - List available prompts from all servers", "/list prompt".green());
    println!("  {} {} - Call a tool with JSON arguments (use server/tool for conflicts)", "/call".green(), "<tool> <json>".dimmed());
    println!("  {} {} - Read and display resource content", "/read resource".green(), "<uri>|<server>/<uri>".dimmed());
    println!("  {} {} - Download resource to local file", "/down resource".green(), "<uri> <path>".dimmed());
    println!("  {} {}    - Show detailed info about a tool", "/info tool".green(), "<name>".dimmed());
    println!("  {} {} - Show detailed info about a resource", "/info resource".green(), "<uri>|<server>/<uri>".dimmed());
    println!("  {} {} - Show detailed info about a prompt", "/info prompt".green(), "<name>|<server>/<name>".dimmed());
    println!("  {} {} - Generate and display prompt", "/use prompt".green(), "<name> [key=value...]".dimmed());
    println!("  {} {}    - Create a new empty MCP configuration file", "/newconfig".green(), "<name>".dimmed());
    println!("  {}                - Exit the REPL", "/exit".green());
    println!();
}
