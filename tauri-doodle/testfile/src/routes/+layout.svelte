<script>
  import {appWindow} from "@tauri-apps/api/window"
  // appWindow.title()

</script>

<style>
</style>

<div data-tauri-resize-region class="titlebar">
  <div data-tauri-drag-region id="drag_region"></div>

  <div class="titlebar-icon" on:click={()=>{window.open("https://github.com")}}>
    <div id="go-github"></div>
  </div>
  <div class="titlebar-icon" on:click={()=>{window.open("https://tauri.app/ko/")}}>
    <div id="go-tauri"></div>
  </div>
  <div class="titlebar-button" id="titlebar-minimize" on:click={()=>{appWindow.minimize()}}>
    <img
      src="https://api.iconify.design/mdi:window-minimize.svg"
      alt="minimize"
    />
  </div>
  <div class="titlebar-button" id="titlebar-maximize" on:click={()=>{appWindow.maximize()}}>
    <img
      src="https://api.iconify.design/mdi:window-maximize.svg"
      alt="maximize"
    />
  </div>
  <div class="titlebar-button" id="titlebar-close" on:click={()=>{appWindow.close()}}>
    <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
  </div>
</div>
<header>
  <div class = "row guideline" style = "margin-top : 30px;">
      <a href="/">Home</a>
      <a href="/about" class="secondary">About</a>
      <!-- <a href="https://github.com/" class="contrast">go Github</a> -->
      <a href="/todos"></a>
  </div>
</header>
<section>
  <div>
    <slot><!-- optional fallback --></slot>  
  </div>
</section>
<footer></footer>