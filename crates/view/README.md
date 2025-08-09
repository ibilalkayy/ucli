# Younix `view` Command

The `view` command in Younix allows interactive viewing of file content, similar to the Unix `less` utility. It supports pagination for easier navigation.

## Usage

```bash
younix view [OPTIONS]
```

## Options

| Option              | Description                              |
|---------------------|------------------------------------------|
| `-f, --file <FILE>` | File to view                             |
| `-l, --lines <LINES>` | Number of lines per page (default: `20`) |
| `-h, --help`        | Print help information                   |

## Examples

- View `file.txt` interactively:
  ```bash
  younix view -f file.txt
  ```

- View `file.txt` with 10 lines per page:
  ```bash
  younix view -f file.txt -l 10
  ```
