# Jrip

A simple, elegant GUI application for extracting audio from video files built with Rust and Iced.

## Overview

Jrip is a cross-platform desktop application that provides a file browser interface for navigating directories and extracting audio from video files using FFmpeg. It features a clean, intuitive UI with directory navigation and one-click audio extraction.

## Features

- **File Browser**: Navigate through your file system with an intuitive GUI
- **Multi-Format Support**: Extract audio from 16 different video formats
- **One-Click Extraction**: Simple "Jrip" button for instant audio extraction
- **Visual Feedback**: Popup notifications for success and error states
- **Clean UI**: Built with the Iced GUI framework using the KanagawaDragon theme

## Supported Video Formats

- MP4, AVI, MKV, MOV
- WMV, FLV, WebM, M4V
- MPG, MPEG, 3GP, OGV
- TS, MTS, M2TS, VOB

## Prerequisites

- **Rust**: 1.70 or later (edition 2024)
- **FFmpeg**: Must be installed and available in your system PATH

### Installing FFmpeg

1. **macOS** (using Homebrew):

    ```bash
    brew install ffmpeg
    ```

2. **Ubuntu/Debian**:

    ```bash
    sudo apt update
    sudo apt install ffmpeg
    ```

3. **Windows** (using Chocolatey):

    ```bash
    choco install ffmpeg
    ```

## Installation

1. **Clone the repository**:

    ```bash
    git clone https://github.com/Abdulkader-Safi/jrip.git
    cd jrip
    ```

2. **Build the project**:

    ```bash
    cargo build --release
    ```

3. **Run the application**:

    ```bash
    cargo run --release
    ```

## Usage

1. **Launch** the application
2. **Navigate** through directories by clicking on folder names
3. **Go Up** a directory using the "Up" button
4. **Extract Audio** by clicking the "Jrip" button next to any video file
5. **Output**: Audio files are saved as `output.mp3` in the same directory as the source video

### Keyboard Shortcuts

- Click "Exit" button to close the application

## Project Structure

```directories
src/
├── main.rs                  # Application entry point
├── lib.rs                   # Library root
├── models.rs                # Data structures (AppState, Message)
├── app.rs                   # Application logic and update function
├── ui/
│   ├── mod.rs              # UI module root
│   ├── view.rs             # View rendering logic
│   └── styles.rs           # UI styling functions
└── utils/
    ├── mod.rs              # Utilities module root
    ├── file_operations.rs  # File system operations
    └── ffmpeg.rs           # FFmpeg audio extraction
```

## Architecture

The application follows Rust best practices with a modular architecture:

- **Models**: Define the application state and message types
- **App**: Contains the update logic that responds to user actions
- **UI**: Handles all rendering and styling
- **Utils**: Provides file operations and FFmpeg integration

### Message Flow

1. User interacts with UI (clicks button)
2. Message is dispatched to `update()` function
3. State is updated based on message type
4. `view()` function renders new UI based on updated state

## Dependencies

- [iced](https://github.com/iced-rs/iced) v0.13.1 - Cross-platform GUI library

## Development

### Building

```bash
cargo build
```

### Running

```bash
cargo run
```

### Testing

```bash
cargo test
```

### Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Configuration

The application uses the KanagawaDragon theme by default. To change the theme, modify the theme function in `src/main.rs`:

```rust
.theme(|_s| iced::Theme::YourThemeHere)
```

## Limitations

- Audio output is always named `output.mp3` (will overwrite existing files)
- Only extracts audio to MP3 format
- No progress bar for long conversions
- No batch processing support

## Future Enhancements

- [ ] Custom output filename/path selection
- [ ] Multiple output format support (WAV, FLAC, AAC, etc.)
- [ ] Progress bar for conversion
- [ ] Batch processing multiple files
- [ ] Drag-and-drop support
- [ ] Audio quality settings
- [ ] Dark/light theme toggle

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is open source and available under the [MIT License](LICENSE).

## Acknowledgments

- Built with [Iced](https://github.com/iced-rs/iced) GUI framework
- Audio extraction powered by [FFmpeg](https://ffmpeg.org/)
- Inspired by the need for a simple, no-frills audio extraction tool

## Troubleshooting

### "Error Ripping" message appears

- Ensure FFmpeg is installed and accessible in your PATH
- Verify the video file is not corrupted
- Check that you have write permissions in the directory

### Application won't start

- Verify Rust toolchain is properly installed
- Check that all dependencies are correctly resolved
- Try rebuilding with `cargo clean && cargo build`

### Video files not showing

- Ensure the file extension is one of the supported formats
- Check file permissions
- Verify the file is not hidden by the operating system

## Contact

For questions, issues, or suggestions, please open an issue on GitHub.
