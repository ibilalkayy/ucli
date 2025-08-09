# Younix `ls` Command

The `ls` command in Younix lists directory contents, similar to the Unix `ls` utility. It supports various options to customize the output format and content.

## Usage

```bash
younix ls [OPTIONS]
```

## Options

| Option            | Description                                  |
|-------------------|----------------------------------------------|
| `-p, --path <PATH>` | Directory path to list files (default: `.`) |
| `-a, --all`        | Show all files, including hidden ones        |
| `-l, --long`       | Use long listing format                      |
| `-h, --help`       | Print help information                       |

## Examples

- List files in the current directory:
  ```bash
  younix ls
  ```

- List all files, including hidden ones:
  ```bash
  younix ls -a
  ```

- List files in long format from a specific directory:
  ```bash
  younix ls -p /path/to/dir -l
  ```
