# Younix `grep` Command

The `grep` command in Younix searches for lines in a file that match a specified pattern, similar to the Unix `grep` utility. It supports case-insensitive searches and inverted matching.

## Usage

```bash
younix grep [OPTIONS] --pattern <PATTERN> <FILE>
```

## Options

| Option                  | Description                              |
|-------------------------|------------------------------------------|
| `-p, --pattern <PATTERN>` | The pattern to search for               |
| `-c, --case-insensitive` | Match the pattern case-insensitively     |
| `-i, --invert`          | Show lines that don't match the pattern  |
| `-n, --number`          | Show line numbers in the output          |
| `-h, --help`            | Print help information                   |

## Arguments

| Argument | Description           |
|----------|-----------------------|
| `<FILE>` | File to search        |

## Examples

- Search for "error" in `logs.txt`:
  ```bash
  younix grep --pattern error logs.txt
  ```

- Case-insensitive search with line numbers:
  ```bash
  younix grep --pattern error logs.txt -c -n
  ```

- Show lines that don't match "error":
  ```bash
  younix grep --pattern error logs.txt -i
  ```
