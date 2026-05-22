# Pyoway MCP Configuration & Setup

MCP (Model Context Protocol) standardizes how AI applications connect with external tools. This directory contains MCP configuration files for various AI code editors and tools.

## MCP Servers

The project configures the following MCP servers:

### GitHub MCP Server

Provides read/write access to GitHub repos, issues, PRs, and more.

**Config location:** `mcp.json` (root), `.cursor/mcp.json` (Cursor IDE)

**Setup:**
1. Ensure `GITHUB_TOKEN` environment variable is set
2. The token needs `repo` scope for private repo access
3. The server starts automatically when the AI tool initializes

### Filesystem MCP Server

Grants AI agents controlled access to the project directory tree. Useful for reading and writing files.

**Note:** Not configured by default via npm — most AI tools (Codebuff, Cursor) have built-in filesystem access. Manual setup via `npx @modelcontextprotocol/server-filesystem .` is optional.

## Supported Tools

| Tool | Config Location | Notes |
|---|---|---|
| **Cursor IDE** | `.cursor/mcp.json` | Reads config on startup |
| **Claude Desktop** | `claude_desktop_config.json` | Manual import of `mcp.json` content |
| **Codebuff** | Built-in | Filesystem access built-in; GitHub access via CLI |

## Testing MCP Servers

To verify a server is working, check the AI tool's MCP status panel:
- **Cursor:** Settings > MCP > Server status
- **Claude Desktop:** MCP indicator in the bottom bar

## Adding New MCP Servers

1. Add the server entry to both `mcp.json` and `.cursor/mcp.json`
2. Document it here
3. Test with the AI tool before merging

## Security Note

- MCP servers run with the same permissions as the AI tool
- The GitHub token should have minimal necessary scopes
- Filesystem access is scoped to the project directory
- Review any new MCP server packages before adding them
