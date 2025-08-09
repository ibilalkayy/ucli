# Younix `cat` Command

The `cat` command in Younix outputs the contents of a file line-by-line, similar to the Unix `cat` utility. It is designed to be fast, safe, and easy to use, built with Rust.

## Usage

```bash
younix cat [OPTIONS]
```

## Options

| Option | Description |
| --- | --- |
| `-p, --path <PATH>` | Path to the file to display (default: `.`) |
| `-n, --number` | Add line numbers to the output |
| `-h, --help` | Print help information |

## Examples

1. Display the contents of `file.txt`:

   ```bash
   younix cat -p file.txt
   ```

2. Display `file.txt` with line numbers:

   ```bash
   younix cat -p file.txt -n
   ```
