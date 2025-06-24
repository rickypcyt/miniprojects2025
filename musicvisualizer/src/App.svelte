<script lang="ts">
  let count: number = 0;
</script>

<main>
  <h1>Hello Svelte + TypeScript + Bun</h1>
  <button on:click={() => count++}>
    Clicks: {count}
  </button>
</main> 