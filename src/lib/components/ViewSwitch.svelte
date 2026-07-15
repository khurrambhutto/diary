<script lang="ts">
  import { currentView, setView, type View } from "$lib/stores/view.svelte";

  function pick(v: View) {
    if (currentView() !== v) setView(v);
  }
</script>

<div class="switch" role="tablist" aria-label="View switch">
  <span
    class="thumb"
    class:right={currentView() === "monthly"}
    aria-hidden="true"
  ></span>
  <button
    type="button"
    role="tab"
    aria-selected={currentView() === "today"}
    class="slot"
    class:active={currentView() === "today"}
    onclick={() => pick("today")}
  >
    Today
  </button>
  <button
    type="button"
    role="tab"
    aria-selected={currentView() === "monthly"}
    class="slot"
    class:active={currentView() === "monthly"}
    onclick={() => pick("monthly")}
  >
    More
  </button>
</div>

<style>
  .switch {
    position: relative;
    width: 9rem;
    height: 2rem;
    border-radius: 9999px;
    box-shadow: none;
    background: transparent;
    display: grid;
    grid-template-columns: 1fr 1fr;
    overflow: hidden;
  }

  .thumb {
    position: absolute;
    top: 0;
    left: 0;
    width: 50%;
    height: 100%;
    border-radius: 9999px;
    background-color: var(--track);
    transform: translateX(0);
    transition: transform 150ms ease;
    z-index: 0;
  }
  .thumb.right {
    transform: translateX(100%);
  }

  .slot {
    position: relative;
    z-index: 1;
    appearance: none;
    border: none;
    background: transparent;
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 14px;
    font-weight: 400;
    color: var(--ink-soft);
    cursor: pointer;
    transition: color 150ms ease, font-weight 150ms ease;
  }
  .slot.active {
    color: var(--ink);
    font-weight: 700;
  }
  .slot:focus-visible {
    outline: none;
  }
</style>