#!/usr/bin/env python3
"""
DevTools CLI - A comprehensive command-line tool for developer utilities.

This CLI tool provides various utilities for developers including:
- File operations
- Text processing
- JSON/YAML manipulation
- Code analysis
- Git helpers
"""

import argparse
import sys
import json
import os
from pathlib import Path
from typing import List, Dict, Optional
import re
import hashlib
import base64


class DevTools:
    """Main class for developer utilities."""

    def __init__(self):
        self.parser = self._create_parser()

    def _create_parser(self) -> argparse.ArgumentParser:
        """Create the argument parser with all subcommands."""
        parser = argparse.ArgumentParser(
            description='DevTools - Developer Utilities CLI',
            formatter_class=argparse.RawDescriptionHelpFormatter,
            epilog='''
Examples:
  devtools.py json format input.json
  devtools.py file count *.py
  devtools.py text encode "Hello World"
  devtools.py hash md5 myfile.txt
            '''
        )

        parser.add_argument('-v', '--version', action='version', version='DevTools 1.0.0')

        subparsers = parser.add_subparsers(dest='command', help='Available commands')

        # JSON command
        json_parser = subparsers.add_parser('json', help='JSON operations')
        json_subparsers = json_parser.add_subparsers(dest='json_action')

        json_format = json_subparsers.add_parser('format', help='Format JSON file')
        json_format.add_argument('file', help='JSON file to format')
        json_format.add_argument('-i', '--indent', type=int, default=2, help='Indentation level')
        json_format.add_argument('-o', '--output', help='Output file (default: overwrite)')

        json_validate = json_subparsers.add_parser('validate', help='Validate JSON file')
        json_validate.add_argument('file', help='JSON file to validate')

        json_minify = json_subparsers.add_parser('minify', help='Minify JSON file')
        json_minify.add_argument('file', help='JSON file to minify')
        json_minify.add_argument('-o', '--output', help='Output file')

        # File command
        file_parser = subparsers.add_parser('file', help='File operations')
        file_subparsers = file_parser.add_subparsers(dest='file_action')

        file_count = file_subparsers.add_parser('count', help='Count files')
        file_count.add_argument('pattern', help='File pattern (e.g., *.py)')
        file_count.add_argument('-r', '--recursive', action='store_true', help='Recursive search')

        file_size = file_subparsers.add_parser('size', help='Calculate file/directory size')
        file_size.add_argument('path', help='File or directory path')
        file_size.add_argument('-h', '--human', action='store_true', help='Human readable')

        file_lines = file_subparsers.add_parser('lines', help='Count lines in files')
        file_lines.add_argument('pattern', help='File pattern')
        file_lines.add_argument('-r', '--recursive', action='store_true', help='Recursive')

        # Text command
        text_parser = subparsers.add_parser('text', help='Text operations')
        text_subparsers = text_parser.add_subparsers(dest='text_action')

        text_encode = text_subparsers.add_parser('encode', help='Base64 encode')
        text_encode.add_argument('text', help='Text to encode')

        text_decode = text_subparsers.add_parser('decode', help='Base64 decode')
        text_decode.add_argument('text', help='Text to decode')

        text_upper = text_subparsers.add_parser('upper', help='Convert to uppercase')
        text_upper.add_argument('text', help='Text to convert')

        text_lower = text_subparsers.add_parser('lower', help='Convert to lowercase')
        text_lower.add_argument('text', help='Text to convert')

        # Hash command
        hash_parser = subparsers.add_parser('hash', help='Hash operations')
        hash_subparsers = hash_parser.add_subparsers(dest='hash_action')

        for algo in ['md5', 'sha1', 'sha256', 'sha512']:
            hash_sub = hash_subparsers.add_parser(algo, help=f'Calculate {algo.upper()} hash')
            hash_sub.add_argument('input', help='File or text to hash')
            hash_sub.add_argument('-f', '--file', action='store_true', help='Input is a file')

        # Search command
        search_parser = subparsers.add_parser('search', help='Search operations')
        search_parser.add_argument('pattern', help='Regex pattern to search')
        search_parser.add_argument('path', help='File or directory to search')
        search_parser.add_argument('-i', '--ignore-case', action='store_true', help='Case insensitive')
        search_parser.add_argument('-r', '--recursive', action='store_true', help='Recursive search')

        return parser

    def run(self, args: List[str] = None):
        """Run the CLI with given arguments."""
        parsed_args = self.parser.parse_args(args)

        if not parsed_args.command:
            self.parser.print_help()
            return

        # Route to appropriate handler
        if parsed_args.command == 'json':
            self._handle_json(parsed_args)
        elif parsed_args.command == 'file':
            self._handle_file(parsed_args)
        elif parsed_args.command == 'text':
            self._handle_text(parsed_args)
        elif parsed_args.command == 'hash':
            self._handle_hash(parsed_args)
        elif parsed_args.command == 'search':
            self._handle_search(parsed_args)

    # JSON Handlers
    def _handle_json(self, args):
        """Handle JSON operations."""
        if args.json_action == 'format':
            self._json_format(args.file, args.indent, args.output)
        elif args.json_action == 'validate':
            self._json_validate(args.file)
        elif args.json_action == 'minify':
            self._json_minify(args.file, args.output)

    def _json_format(self, file: str, indent: int, output: Optional[str]):
        """Format a JSON file."""
        try:
            with open(file, 'r') as f:
                data = json.load(f)

            formatted = json.dumps(data, indent=indent, sort_keys=True)

            if output:
                with open(output, 'w') as f:
                    f.write(formatted)
                print(f"Formatted JSON written to {output}")
            else:
                print(formatted)
        except json.JSONDecodeError as e:
            print(f"Error: Invalid JSON - {e}", file=sys.stderr)
            sys.exit(1)
        except FileNotFoundError:
            print(f"Error: File not found - {file}", file=sys.stderr)
            sys.exit(1)

    def _json_validate(self, file: str):
        """Validate a JSON file."""
        try:
            with open(file, 'r') as f:
                json.load(f)
            print(f"✓ {file} is valid JSON")
        except json.JSONDecodeError as e:
            print(f"✗ Invalid JSON: {e}", file=sys.stderr)
            sys.exit(1)
        except FileNotFoundError:
            print(f"Error: File not found - {file}", file=sys.stderr)
            sys.exit(1)

    def _json_minify(self, file: str, output: Optional[str]):
        """Minify a JSON file."""
        try:
            with open(file, 'r') as f:
                data = json.load(f)

            minified = json.dumps(data, separators=(',', ':'))

            if output:
                with open(output, 'w') as f:
                    f.write(minified)
                print(f"Minified JSON written to {output}")
            else:
                print(minified)
        except json.JSONDecodeError as e:
            print(f"Error: Invalid JSON - {e}", file=sys.stderr)
            sys.exit(1)

    # File Handlers
    def _handle_file(self, args):
        """Handle file operations."""
        if args.file_action == 'count':
            self._file_count(args.pattern, args.recursive)
        elif args.file_action == 'size':
            self._file_size(args.path, args.human)
        elif args.file_action == 'lines':
            self._file_lines(args.pattern, args.recursive)

    def _file_count(self, pattern: str, recursive: bool):
        """Count files matching pattern."""
        if recursive:
            files = list(Path('.').rglob(pattern))
        else:
            files = list(Path('.').glob(pattern))

        print(f"Found {len(files)} files matching '{pattern}'")
        for f in files:
            print(f"  {f}")

    def _file_size(self, path: str, human: bool):
        """Calculate file or directory size."""
        p = Path(path)

        if not p.exists():
            print(f"Error: Path not found - {path}", file=sys.stderr)
            sys.exit(1)

        if p.is_file():
            size = p.stat().st_size
        else:
            size = sum(f.stat().st_size for f in p.rglob('*') if f.is_file())

        if human:
            size_str = self._human_readable_size(size)
        else:
            size_str = str(size)

        print(f"{path}: {size_str}")

    def _human_readable_size(self, size: int) -> str:
        """Convert bytes to human readable format."""
        for unit in ['B', 'KB', 'MB', 'GB', 'TB']:
            if size < 1024.0:
                return f"{size:.2f} {unit}"
            size /= 1024.0
        return f"{size:.2f} PB"

    def _file_lines(self, pattern: str, recursive: bool):
        """Count lines in files."""
        if recursive:
            files = list(Path('.').rglob(pattern))
        else:
            files = list(Path('.').glob(pattern))

        total_lines = 0
        for f in files:
            try:
                with open(f, 'r') as file:
                    lines = len(file.readlines())
                    print(f"{f}: {lines} lines")
                    total_lines += lines
            except Exception as e:
                print(f"Error reading {f}: {e}", file=sys.stderr)

        print(f"\nTotal: {total_lines} lines across {len(files)} files")

    # Text Handlers
    def _handle_text(self, args):
        """Handle text operations."""
        if args.text_action == 'encode':
            encoded = base64.b64encode(args.text.encode()).decode()
            print(encoded)
        elif args.text_action == 'decode':
            try:
                decoded = base64.b64decode(args.text.encode()).decode()
                print(decoded)
            except Exception as e:
                print(f"Error: Invalid base64 - {e}", file=sys.stderr)
                sys.exit(1)
        elif args.text_action == 'upper':
            print(args.text.upper())
        elif args.text_action == 'lower':
            print(args.text.lower())

    # Hash Handlers
    def _handle_hash(self, args):
        """Handle hash operations."""
        algo = args.hash_action

        if args.file:
            try:
                with open(args.input, 'rb') as f:
                    data = f.read()
            except FileNotFoundError:
                print(f"Error: File not found - {args.input}", file=sys.stderr)
                sys.exit(1)
        else:
            data = args.input.encode()

        if algo == 'md5':
            h = hashlib.md5(data).hexdigest()
        elif algo == 'sha1':
            h = hashlib.sha1(data).hexdigest()
        elif algo == 'sha256':
            h = hashlib.sha256(data).hexdigest()
        elif algo == 'sha512':
            h = hashlib.sha512(data).hexdigest()

        print(f"{algo.upper()}: {h}")

    # Search Handler
    def _handle_search(self, args):
        """Handle search operations."""
        pattern = re.compile(args.pattern, re.IGNORECASE if args.ignore_case else 0)
        path = Path(args.path)

        if path.is_file():
            self._search_file(path, pattern)
        elif path.is_dir() and args.recursive:
            for f in path.rglob('*'):
                if f.is_file():
                    self._search_file(f, pattern)
        else:
            print(f"Error: {args.path} is not a file or use -r for directories", file=sys.stderr)

    def _search_file(self, file: Path, pattern: re.Pattern):
        """Search for pattern in a file."""
        try:
            with open(file, 'r') as f:
                for i, line in enumerate(f, 1):
                    if pattern.search(line):
                        print(f"{file}:{i}: {line.rstrip()}")
        except Exception:
            pass  # Skip files that can't be read


def main():
    """Main entry point."""
    cli = DevTools()
    cli.run()


if __name__ == '__main__':
    main()
