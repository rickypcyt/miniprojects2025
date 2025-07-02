<script lang="ts">
  import Visualizer2D from './Visualizer2D.svelte';
  import Visualizer3D from './Visualizer3D.svelte';
  import type { AudioInput } from './AudioInput';
  export let audio: AudioInput;
  let mode: '2D' | '3D' = '2D';
</script>

<div class="flex gap-4 p-4 z-10 absolute top-0 left-0">
  <button class="px-4 py-2 bg-black text-pink-400 border border-pink-400 rounded shadow hover:bg-pink-900 transition" on:click={() => mode = '2D'}>2D</button>
  <button class="px-4 py-2 bg-black text-cyan-400 border border-cyan-400 rounded shadow hover:bg-cyan-900 transition" on:click={() => mode = '3D'}>3D</button>
</div>

{#if mode === '2D'}
  <Visualizer2D {audio} />
{:else}
  <Visualizer3D {audio} />
{/if} 