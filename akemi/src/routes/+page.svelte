<script lang="ts">
  import Waves from "@components/Bubbles.svelte";

  let showHanagana = true;

  import { base } from "$app/paths";
</script>

<Waves>
  <main class="height-limited-grid-container not-so-high">
    <div class="grid-content center-all">
      <h1 class="welcome">
        Welcome to
      </h1>
      <h1 class="project-name" on:click={() => showHanagana = !showHanagana} on:keydown on:keyup>
        {showHanagana ? "ともあけみ" : "Tomo Akemi"}
      </h1>
    </div>
    <div class="grid-content center-all">
      <a href="{base}/chip8" class="landing-link">Get started</a>
    </div>
  </main>
  <div class="designed-by center-all">
    <h2>Designed by<a href="https://github.com/lumisxh" target="_blank" rel="noreferrer" class="designer">LuMiSxh</a></h2>
  </div>
</Waves>
