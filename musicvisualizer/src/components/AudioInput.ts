// AudioInput.ts
// Módulo para capturar audio del micrófono y exponer datos de frecuencia y waveform

export class AudioInput {
  private audioContext: AudioContext | null = null;
  private analyser: AnalyserNode | null = null;
  private dataArray: Uint8Array | null = null;
  private waveformArray: Uint8Array | null = null;
  private source: MediaStreamAudioSourceNode | null = null;
  private stream: MediaStream | null = null;
  private fftSize: number;

  constructor(fftSize = 2048) {
    this.fftSize = fftSize;
  }

  async init() {
    this.audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
    this.stream = await navigator.mediaDevices.getUserMedia({ audio: true });
    this.source = this.audioContext.createMediaStreamSource(this.stream);
    this.analyser = this.audioContext.createAnalyser();
    this.analyser.fftSize = this.fftSize;
    this.dataArray = new Uint8Array(this.analyser.frequencyBinCount);
    this.waveformArray = new Uint8Array(this.analyser.fftSize);
    this.source.connect(this.analyser);
  }

  getFrequencyData(): Uint8Array | null {
    if (this.analyser && this.dataArray) {
      this.analyser.getByteFrequencyData(this.dataArray);
      return this.dataArray;
    }
    return null;
  }

  getWaveformData(): Uint8Array | null {
    if (this.analyser && this.waveformArray) {
      this.analyser.getByteTimeDomainData(this.waveformArray);
      return this.waveformArray;
    }
    return null;
  }

  close() {
    if (this.audioContext) {
      this.audioContext.close();
      this.audioContext = null;
    }
    if (this.stream) {
      this.stream.getTracks().forEach(track => track.stop());
      this.stream = null;
    }
  }
} 