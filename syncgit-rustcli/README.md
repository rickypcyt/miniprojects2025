# SyncGit CLI

A command-line tool to easily sync Git repositories.

## Features

- 🔄 Automatically syncs the current repository with its remote
- 📂 Explore and sync Git sub-repositories
- 🌐 Checks internet connection before performing operations
- 💻 Intuitive and user-friendly interface

## Global Installation

To make the tool globally available:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install syncgit
```

## Usage

Simply run the program in any folder containing a Git repository:

```
syncgit
```

Follow the interactive menu options to:
- Sync the current repository
- View and sync sub-repositories
- Exit the program

## Requirements

- Rust 2021 Edition or higher
- Internet connection to sync with remotes
- Git installed on the system

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.