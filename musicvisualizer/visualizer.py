import numpy as np
import sounddevice as sd
import pygame
import scipy.fftpack
from typing import List, Tuple
import queue
import threading
import sys
import os

class AudioCapture:
    def __init__(self, sample_rate: int = 44100, block_size: int = 1024):
        # Set PipeWire environment variables
        os.environ['PIPEWIRE_LATENCY'] = '60/48000'
        os.environ['PIPEWIRE_RUNTIME_DIR'] = '/run/user/1000'
        
        self.sample_rate = sample_rate
        self.block_size = block_size
        self.audio_queue = queue.Queue()
        self.stream = None
        self.is_running = False
        
        # Get available devices and their supported sample rates
        self.devices = sd.query_devices()
        self.default_input = sd.query_devices(kind='input')
        
        # Try to find a supported sample rate
        if self.default_input['max_input_channels'] > 0:
            self.sample_rate = int(self.default_input['default_samplerate'])
            print(f"Using sample rate: {self.sample_rate}")
            print(f"Using input device: {self.default_input['name']}")

    def audio_callback(self, indata, frames, time, status):
        """Callback function for audio stream"""
        if status:
            print(f"Audio callback status: {status}")
        self.audio_queue.put(indata[:, 0])

    def start(self):
        """Start audio capture"""
        try:
            self.is_running = True
            
            # Configure stream with PipeWire-optimized settings
            self.stream = sd.InputStream(
                device=None,  # Use default device
                channels=1,
                samplerate=self.sample_rate,
                blocksize=self.block_size,
                callback=self.audio_callback,
                dtype=np.float32,
                latency='low'
            )
            self.stream.start()
            print("Audio capture started successfully with PipeWire")
            
        except sd.PortAudioError as e:
            print(f"Error starting audio capture: {e}")
            print("\nTroubleshooting steps for PipeWire:")
            print("1. Check if PipeWire is running:")
            print("   systemctl --user status pipewire.service")
            print("   systemctl --user status pipewire-pulse.service")
            print("\n2. Check available audio devices:")
            for i, dev in enumerate(self.devices):
                print(f"Device {i}: {dev['name']}")
            print("\n3. Try restarting PipeWire services:")
            print("   systemctl --user restart pipewire.service")
            print("   systemctl --user restart pipewire-pulse.service")
            print("\n4. Check PipeWire configuration:")
            print("   cat ~/.config/pipewire/pipewire.conf")
            sys.exit(1)

    def stop(self):
        """Stop audio capture"""
        self.is_running = False
        if self.stream:
            try:
                self.stream.stop()
                self.stream.close()
                print("Audio capture stopped successfully")
            except Exception as e:
                print(f"Error stopping audio capture: {e}")

class AudioProcessor:
    def __init__(self, block_size: int = 1024):
        self.block_size = block_size

    def get_frequency_spectrum(self, audio_data: np.ndarray) -> Tuple[np.ndarray, np.ndarray]:
        """Convert audio data to frequency spectrum"""
        fft_data = scipy.fftpack.fft(audio_data)
        freqs = scipy.fftpack.fftfreq(len(audio_data))
        return freqs[:self.block_size//2], np.abs(fft_data[:self.block_size//2])

class Visualizer:
    def __init__(self, width: int = 800, height: int = 600):
        pygame.init()
        self.width = width
        self.height = height
        self.screen = pygame.display.set_mode((width, height))
        pygame.display.set_caption("Music Visualizer")
        self.clock = pygame.time.Clock()
        self.running = True

    def draw_spectrum(self, frequencies: np.ndarray, magnitudes: np.ndarray):
        """Draw frequency spectrum visualization"""
        self.screen.fill((0, 0, 0))
        
        # Normalize magnitudes
        if len(magnitudes) > 0:
            max_magnitude = np.max(magnitudes)
            if max_magnitude > 0:
                magnitudes = magnitudes / max_magnitude
        
        # Draw frequency bars
        num_bars = len(magnitudes)
        if num_bars > 0:
            bar_width = max(1, self.width / num_bars)
            for i, magnitude in enumerate(magnitudes):
                # Calculate bar dimensions
                x = int(i * bar_width)
                bar_height = int(magnitude * self.height * 0.8)
                y = self.height - bar_height
                
                # Ensure dimensions are valid
                if bar_height > 0 and x < self.width:
                    # Create a gradient color based on frequency
                    color = (
                        int(255 * (i / num_bars)),  # R
                        int(255 * (1 - i / num_bars)),  # G
                        255  # B
                    )
                    
                    # Draw the bar
                    pygame.draw.rect(
                        self.screen,
                        color,
                        (x, y, max(1, bar_width - 1), bar_height)
                    )
        
        pygame.display.flip()

    def run(self, audio_capture: AudioCapture, audio_processor: AudioProcessor):
        """Main visualization loop"""
        while self.running:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    self.running = False
                elif event.type == pygame.KEYDOWN:
                    if event.key == pygame.K_ESCAPE:
                        self.running = False

            try:
                # Get audio data from queue
                audio_data = audio_capture.audio_queue.get_nowait()
                
                # Process audio data
                freqs, magnitudes = audio_processor.get_frequency_spectrum(audio_data)
                
                # Draw visualization
                self.draw_spectrum(freqs, magnitudes)
                
            except queue.Empty:
                pass

            self.clock.tick(60)

        pygame.quit()

def main():
    # Initialize components
    audio_capture = AudioCapture()
    audio_processor = AudioProcessor()
    visualizer = Visualizer()

    try:
        # Start audio capture
        audio_capture.start()
        
        # Run visualization
        visualizer.run(audio_capture, audio_processor)
    finally:
        # Clean up
        audio_capture.stop()

if __name__ == "__main__":
    main() 