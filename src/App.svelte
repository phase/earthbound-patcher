<script lang="ts">
  import Patcher from './lib/Patcher.svelte'
</script>

<main class="container">

  <img src="/EB.png" class="logo earthbound" alt="EarthBound Logo" />
  <h1>EarthBound Patcher</h1>
  <p>Apply an EBP patch to an EarthBound ROM.</p>

  <div class="row">
    <Patcher />
  </div>
</main>

<style>
  .logo.earthbound:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }

  img {
    width: auto;
    height: auto;
    max-width: 100px;
    display: block;
    margin-left: auto;
    margin-right: auto;
  }
</style>