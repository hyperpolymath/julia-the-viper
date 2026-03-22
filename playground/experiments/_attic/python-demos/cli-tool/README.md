# DevTools CLI

A comprehensive command-line tool providing developer utilities for everyday tasks.

## Features

- **JSON Operations**: Format, validate, and minify JSON files
- **File Operations**: Count files, calculate sizes, count lines
- **Text Processing**: Base64 encoding/decoding, case conversion
- **Hash Calculations**: MD5, SHA1, SHA256, SHA512
- **Search**: Regex search across files

## Installation

```bash
chmod +x devtools.py
# Optionally, create a symlink or alias
ln -s $(pwd)/devtools.py /usr/local/bin/devtools
```

## Usage

### JSON Operations

```bash
# Format JSON with custom indentation
./devtools.py json format data.json -i 4

# Format and save to new file
./devtools.py json format data.json -o formatted.json

# Validate JSON
./devtools.py json validate data.json

# Minify JSON
./devtools.py json minify data.json -o min.json
```

### File Operations

```bash
# Count files matching pattern
./devtools.py file count "*.py"

# Count files recursively
./devtools.py file count "*.js" -r

# Calculate file size
./devtools.py file size myfile.txt

# Calculate directory size (human readable)
./devtools.py file size ./src -h

# Count lines in files
./devtools.py file lines "*.py"
./devtools.py file lines "*.js" -r
```

### Text Operations

```bash
# Base64 encode
./devtools.py text encode "Hello World"

# Base64 decode
./devtools.py text decode "SGVsbG8gV29ybGQ="

# Convert to uppercase
./devtools.py text upper "hello world"

# Convert to lowercase
./devtools.py text lower "HELLO WORLD"
```

### Hash Operations

```bash
# Hash a string
./devtools.py hash md5 "Hello World"
./devtools.py hash sha256 "Hello World"

# Hash a file
./devtools.py hash md5 myfile.txt -f
./devtools.py hash sha256 document.pdf -f
```

### Search Operations

```bash
# Search for pattern in file
./devtools.py search "TODO" script.py

# Case-insensitive search
./devtools.py search "error" log.txt -i

# Recursive search in directory
./devtools.py search "import.*pandas" ./src -r
```

## Command Reference

### JSON Commands

| Command | Description | Options |
|---------|-------------|---------|
| `json format <file>` | Format JSON file | `-i, --indent` (indent level)<br>`-o, --output` (output file) |
| `json validate <file>` | Validate JSON syntax | None |
| `json minify <file>` | Remove whitespace | `-o, --output` (output file) |

### File Commands

| Command | Description | Options |
|---------|-------------|---------|
| `file count <pattern>` | Count matching files | `-r, --recursive` |
| `file size <path>` | Calculate size | `-h, --human` (human readable) |
| `file lines <pattern>` | Count lines | `-r, --recursive` |

### Text Commands

| Command | Description |
|---------|-------------|
| `text encode <text>` | Base64 encode |
| `text decode <text>` | Base64 decode |
| `text upper <text>` | Convert to uppercase |
| `text lower <text>` | Convert to lowercase |

### Hash Commands

| Command | Description | Options |
|---------|-------------|---------|
| `hash md5 <input>` | MD5 hash | `-f, --file` (input is file) |
| `hash sha1 <input>` | SHA1 hash | `-f, --file` |
| `hash sha256 <input>` | SHA256 hash | `-f, --file` |
| `hash sha512 <input>` | SHA512 hash | `-f, --file` |

### Search Commands

| Command | Description | Options |
|---------|-------------|---------|
| `search <pattern> <path>` | Regex search | `-i, --ignore-case`<br>`-r, --recursive` |

## Examples

### Workflow Examples

#### 1. Code Quality Check
```bash
# Count Python files
./devtools.py file count "*.py" -r

# Count total lines of code
./devtools.py file lines "*.py" -r

# Search for TODO comments
./devtools.py search "TODO|FIXME" ./src -r -i
```

#### 2. JSON Processing
```bash
# Validate API response
./devtools.py json validate response.json

# Format for readability
./devtools.py json format response.json -o formatted_response.json

# Minify for production
./devtools.py json minify config.json -o config.min.json
```

#### 3. File Integrity
```bash
# Generate checksums
./devtools.py hash sha256 file.zip -f

# Verify download
./devtools.py hash md5 downloaded.tar.gz -f
```

#### 4. Text Encoding
```bash
# Encode API key
./devtools.py text encode "my-secret-key-123"

# Decode configuration
./devtools.py text decode "bXktc2VjcmV0LWtleS0xMjM="
```

## Exit Codes

- `0`: Success
- `1`: Error (invalid input, file not found, etc.)

## Error Handling

The tool provides clear error messages:
- File not found errors
- Invalid JSON syntax
- Invalid base64 encoding
- Invalid regex patterns

## Extending the Tool

Add new commands by:
1. Adding subparser in `_create_parser()`
2. Creating handler method (e.g., `_handle_mycommand()`)
3. Adding routing logic in `run()`

## Future Enhancements

- YAML support
- XML operations
- CSV processing
- Network utilities (ping, port check)
- Git helpers
- Docker utilities
- String manipulation (slugify, pluralize)
- Date/time utilities

## License

MIT License
