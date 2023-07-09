<script lang="ts">
  import Greet from './lib/Greet.svelte'
</script>

<main class="container">
  

  <div class="row">
    <img src="/EB.png" class="logo earthbound" alt="EarthBound Logo" />
    <h1>EarthBound<br>Patcher</h1>
  </div>

  <p>
    Create and apply EBP patches to an EarthBound ROM.
  </p>

  <div class="row">
    <Greet />
  </div>


</main>

<style>
  .logo.earthbound:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>