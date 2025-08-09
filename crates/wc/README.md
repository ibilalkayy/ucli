# Younix `wc` Command

The `wc` command in Younix counts lines, words, and bytes in a file, similar to the Unix `wc` utility. It allows selective counting through flags.

## Usage

```bash
younix wc [OPTIONS]
```

## Options

| Option            | Description                              |
|-------------------|------------------------------------------|
| `-f, --file <FILE>` | File path to target for counting        |
| `-l, --lines`      | Count lines                             |
| `-w, --words`      | Count words                             |
| `-b, --bytes`      | Count bytes                             |
| `-h, --help`       | Print help information                   |

## Examples

- Count lines, words, and bytes in `file.txt`:
  ```bash
  younix wc -f file.txt -l -w -b
  ```

- Count only lines in `file.txt`:
  ```bash
  younix wc -f file.txt -l
  ```

- Count words and bytes in `file.txt`:
  ```bash
  younix wc -f file.txt -w -b
  ```
