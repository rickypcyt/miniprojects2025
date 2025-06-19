# PDF.js Viewer for Obsidian

This plugin allows you to view PDF files directly in Obsidian using PDF.js from Firefox.

## Features

- View PDF files directly in Obsidian
- Uses PDF.js from Firefox for high-quality rendering
- Customizable PDF.js viewer path
- Simple and intuitive interface

## Installation

1. Download the latest release from the releases page
2. Extract the zip file into your Obsidian vault's plugins folder
3. Enable the plugin in Obsidian settings

## Usage

1. Open the PDF viewer using the command palette (Ctrl/Cmd + P) and search for "Open PDF Viewer"
2. The PDF viewer will open in a new pane
3. You can load PDFs by using the `loadPDF` method or by clicking on PDF files in your vault

## Development

1. Clone this repository
2. Run `bun install` to install dependencies
3. Run `bun run dev` to start development mode
4. Make your changes
5. Run `bun run build` to build the plugin

## Settings

You can customize the following settings:

- PDF.js Path: The path to the PDF.js viewer (default: https://mozilla.github.io/pdf.js/web/viewer.html)

## License

This project is licensed under the MIT License - see the LICENSE file for details. 