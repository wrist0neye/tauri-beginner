<script>

  import { Command } from "@tauri-apps/api/shell";

</script>
<div>
    <a href="/">Home</a>
    <a href="/about" class="secondary">About</a>
    <!-- <a href="https://github.com/" class="contrast">go Github</a> -->
    <a on:click="{()=> {window.open("https://github.com")}}">Github</a>
</div>

<slot><!-- optional fallback --></slot>