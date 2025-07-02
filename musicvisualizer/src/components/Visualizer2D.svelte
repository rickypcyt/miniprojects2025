<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import p5 from 'p5';
  import type { AudioInput } from './AudioInput';
  export let audio: AudioInput;
  let container: HTMLDivElement;
  let sketchInstance: p5;

  onMount(() => {
    sketchInstance = new p5((p: p5) => {
      p.setup = () => {
        p.createCanvas(container.offsetWidth, container.offsetHeight);
        p.noStroke();
      };
      p.draw = () => {
        p.background(10, 0.5); // Fondo oscuro, efecto de arrastre
        const freq = audio.getFrequencyData();
        const wave = audio.getWaveformData();
        if (freq && wave) {
          // Ejemplo: barras trippy
          for (let i = 0; i < freq.length; i++) {
            const x = (i / freq.length) * p.width;
            const h = p.map(freq[i], 0, 255, 0, p.height);
            p.fill(p.random(180,255), p.random(0,255), p.random(200,255), 180);
            p.rect(x, p.height - h, p.width / freq.length, h);
          }
          // Ejemplo: onda psicodélica
          p.beginShape();
          for (let i = 0; i < wave.length; i++) {
            const x = (i / wave.length) * p.width;
            const y = p.height/2 + p.map(wave[i], 0, 255, -p.height/2, p.height/2);
            p.stroke(p.random(100,255), p.random(100,255), 255, 120);
            p.vertex(x, y);
          }
          p.endShape();
        }
      };
      p.windowResized = () => {
        p.resizeCanvas(container.offsetWidth, container.offsetHeight);
      };
    }, container);
  });

  onDestroy(() => {
    sketchInstance?.remove();
  });
</script>

<div bind:this={container} class="w-full h-full absolute top-0 left-0"></div> 