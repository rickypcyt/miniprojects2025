# Music Visualizer

A real-time audio visualizer that captures and displays internal audio from your Linux system using PipeWire.

## Features

- Real-time audio capture from system audio using PipeWire
- Frequency spectrum visualization
- Smooth, colorful display
- Low latency performance

## Requirements

- Python 3.8+
- Linux system with PipeWire
- Required Python packages (listed in requirements.txt)

## Installation

1. Clone this repository
2. Install the required packages:
```bash
pip install -r requirements.txt
```

## Usage

1. Make sure PipeWire is running:
```bash
systemctl --user status pipewire.service
systemctl --user status pipewire-pulse.service
```

2. If PipeWire is not running, start it:
```bash
systemctl --user start pipewire.service
systemctl --user start pipewire-pulse.service
```

3. Set PipeWire environment variables:
```bash
export PIPEWIRE_LATENCY=60/48000
export PIPEWIRE_RUNTIME_DIR=/run/user/1000
```

4. Run the visualizer:
```bash
python visualizer.py
```

5. Controls:
- Press ESC or close the window to exit
- The visualization will automatically start capturing system audio

## Troubleshooting

If you encounter any issues with audio capture:

1. Check PipeWire services:
```bash
systemctl --user status pipewire.service
systemctl --user status pipewire-pulse.service
```

2. If services are not running, start them:
```bash
systemctl --user start pipewire.service
systemctl --user start pipewire-pulse.service
```

3. Check available audio devices:
```bash
python -c "import sounddevice as sd; print(sd.query_devices())"
```

4. If you're still having issues, try restarting PipeWire:
```bash
systemctl --user restart pipewire.service
systemctl --user restart pipewire-pulse.service
```

5. Check PipeWire configuration:
```bash
cat ~/.config/pipewire/pipewire.conf
```

6. If needed, install or reinstall PipeWire:
```bash
sudo pacman -S pipewire pipewire-pulse pipewire-alsa pipewire-jack
```

7. Check system audio settings:
- Make sure your system's audio input is properly configured
- Verify that the correct input device is selected
- Check if the input volume is not muted
- Use `pavucontrol` to check audio routing and settings:
  ```bash
  pavucontrol
  ```
  - In pavucontrol, go to the "Input Devices" tab
  - Make sure "Monitor of Built-in Audio" or similar is not muted
  - Check that the input volume is at an appropriate level

## License

MIT License 