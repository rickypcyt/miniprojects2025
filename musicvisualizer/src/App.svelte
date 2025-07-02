<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import VisualizerSwitcher from './components/VisualizerSwitcher.svelte';
  import { AudioInput } from './components/AudioInput';
  let audio: AudioInput;
  let ready = false;
  let error = '';
  let recording = false;
  let visualizerMounted = false;

  async function startMic() {
    console.log("startMic called");
    recording = true;
    audio = new AudioInput(2048);
    try {
      await audio.init();
      ready = true;
      error = '';
      console.log("audio.init() success");
    } catch (e) {
      error = 'No se pudo acceder al micrófono. Permite el acceso para ver los visuales.';
      recording = false;
      console.error("audio.init() failed", e);
    }
  }

  onMount(() => {
    startMic(); // Activar el micrófono automáticamente al montar
  });

  onDestroy(() => {
    audio?.close();
  });

  function handleMount() {
    visualizerMounted = true;
  }
  function handleDestroy() {
    visualizerMounted = false;
  }
</script>

<main class="w-screen h-screen bg-black overflow-hidden relative">
  <h1 class="text-3xl text-center text-pink-400 font-bold pt-4 z-20 relative drop-shadow-lg">MUSIC VISUALIZER TECHNO TRIPPY</h1>
  <!-- DEBUG INFO -->
  <div class="fixed bottom-2 left-2 bg-black/80 text-xs text-green-400 p-2 rounded z-50">
    <div>ready: {ready ? 'true' : 'false'}</div>
    <div>recording: {recording ? 'true' : 'false'}</div>
    <div>error: {error}</div>
    <div>VisualizerSwitcher montado: {visualizerMounted ? 'true' : 'false'}</div>
  </div>
  {#if error}
    <div class="text-red-400 text-center mt-8">{error}</div>
  {:else if !ready}
    <div class="flex flex-col items-center justify-center h-full text-cyan-400 gap-6 px-4">
      <div class="text-2xl font-bold">Solicitando acceso al micrófono...</div>
      <div class="mt-8 text-green-400 text-lg font-bold animate-pulse">Grabando... Revisa pavucontrol</div>
    </div>
  {:else}
    <VisualizerSwitcher {audio} on:mount={handleMount} on:destroy={handleDestroy} />
  {/if}
</main> 