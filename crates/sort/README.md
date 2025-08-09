# Younix `sort` Command

The `sort` command in Younix sorts lines in a file, similar to the Unix `sort` utility. It supports reverse and numeric sorting options.

## Usage

```bash
younix sort [OPTIONS]
```

## Options

| Option            | Description                              |
|-------------------|------------------------------------------|
| `-f, --file <FILE>` | File to sort (default: `.`)             |
| `-r, --reverse`    | Sort in reverse order                   |
| `-n, --number`     | Sort numerically                        |
| `-h, --help`       | Print help information                   |

## Examples

- Sort `data.txt` alphabetically:
  ```bash
  younix sort -f data.txt
  ```

- Sort `data.txt` in reverse order:
  ```bash
  younix sort -f data.txt -r
  ```

- Sort `numbers.txt` numerically:
  ```bash
  younix sort -f numbers.txt -n
  ```
