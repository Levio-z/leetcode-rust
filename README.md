# My Leetcode Solution in Rust

A command-line tool to generate and manage LeetCode problem templates in Rust.

## Installation

### Install to Local System

Install the `lc` command to `/usr/local/bin`:

```bash
make install
```

### Uninstall

Remove the installed binary:

```bash
make uninstall
```

## Usage

After installation, use the `lc` command:

### Fetch a Specific Problem

```bash
lc 01      # Fetch problem 01
lc 42      # Fetch problem 42
lc 100     # Fetch problem 100
```

### Generate a Random Problem

```bash
lc --random
lc -r
```

### Move Problem to Solution Directory

```bash
lc 01 --solve
lc 01 -s
```

### Initialize All Problems

```bash
lc --all
lc -a
```

### Show Help

```bash
lc --help
```

## Command Reference

| Command               | Description                        |
| --------------------- | ---------------------------------- |
| `lc <id>`             | Fetch and initialize problem by ID |
| `lc -r, --random`     | Generate a random problem          |
| `lc <id> -s, --solve` | Move problem to solution directory |
| `lc -h, --help`       | Show help message                  |
