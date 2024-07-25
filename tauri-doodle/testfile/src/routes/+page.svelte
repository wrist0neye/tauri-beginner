<script>
  import { invoke } from "@tauri-apps/api/tauri";

  export let name = "";
  export let greetMsg = "";

  export async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });
  }

  document.addEventListener("DOMContentLoaded", ()=>{
    invoke("close_splashscreen");
  })
</script>

<h1>Welcome to Tauri!</h1>
<div class="container column guideline">
  <div class="bodybox row guideline">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://kit.svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" />
    </a>
  </div>

  <p class="guideline">Click on the Tauri, Vite, and SvelteKit logos to learn more.</p>

  <form class="row guideline" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>

  <section id="typograph">
    <h2>Typography</h2>
    <p>
      Aliquam lobortis vitae nibh nec rhoncus. Morbi mattis neque eget efficitur feugiat.
      Vivamus porta nunc a erat mattis, mattis feugiat turpis pretium. Quisque sed tristique
      felis.
    </p>

    <!-- Blockquote-->
    <blockquote>
      "Maecenas vehicula metus tellus, vitae congue turpis hendrerit non. Nam at dui sit amet
      ipsum cursus ornare."
      <footer>
        <cite>- Phasellus eget lacinia</cite>
      </footer>
  </section>

  <p>{greetMsg}</p>
</div>
