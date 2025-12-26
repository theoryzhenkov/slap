# slap

Create files and directories with ease — `touch`, but slappier.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
slap [OPTIONS] [PATHS]...
```

### Options

| Flag | Description |
|------|-------------|
| `-p` | Print created paths to stdout |
| `-t` | Create in a temporary directory |
| `-d` | Create directories instead of files |
| `-o [APP]` | Open created paths (with `$EDITOR` or specify app) |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

### Examples

```bash
# Create a file (parent directories are created automatically)
slap src/utils/helpers.rs

# Create multiple files
slap foo.txt bar.txt baz.txt

# Create a directory (use -d or trailing slash)
slap -d my_project
slap my_project/

# Create files in a temp directory
slap -t scratch.txt notes.md
# Output: /tmp/.tmpXXXXXX/scratch.txt
#         /tmp/.tmpXXXXXX/notes.md

# Create and immediately open in editor
slap -o draft.md

# Create and open with specific app
slap -o=code src/main.rs

# Create temp file and print its path
slap -t -p
# Output: /tmp/.tmpXXXXXX

# Combine flags: temp dir, print path, open in editor
slap -tpo notes.md
```

## Configuration

slap can be configured via a config file or environment variables.

### Config File

Create `~/.config/slap/config.toml` (or `$XDG_CONFIG_HOME/slap/config.toml`):

```toml
# Custom subdirectory for temp files (under system temp dir)
tmpdir = "slap"
```

With this config, temp files will be created in `/tmp/slap/.tmpXXXXXX` instead of `/tmp/.tmpXXXXXX`.

### Environment Variables

| Variable | Description |
|----------|-------------|
| `SLAP_TMPDIR` | Override the temp directory subdirectory (takes precedence over config file) |
| `EDITOR` | Editor to use with `-o` flag (defaults to `vi`) |

### Config Precedence

1. Environment variable `SLAP_TMPDIR` (highest priority)
2. Config file `tmpdir` setting
3. System default temp directory (lowest priority)

## Tips

- **Trailing slash = directory**: `slap foo/` creates a directory, even without `-d`
- **Auto-create parents**: `slap deep/nested/path/file.txt` creates all parent directories
- **Temp mode prints by default**: `-t` implies `-p`, so paths are always printed
- **Combine with other tools**: `cd $(slap -t -d)` to cd into a fresh temp directory

## License

MIT

