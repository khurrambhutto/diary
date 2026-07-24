<script lang="ts">
  import { onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  // macOS puts traffic lights on the left; Windows/Linux get them on the right.
  let isMac = $state(false);

  onMount(() => {
    isMac =
      typeof navigator !== "undefined" &&
      /Mac|iPhone|iPad|iPod/i.test(navigator.platform);
  });

  async function minimize() {
    await getCurrentWindow().minimize();
  }

  async function toggleMaximize() {
    const w = getCurrentWindow();
    if (await w.isMaximized()) {
      await w.unmaximize();
    } else {
      await w.maximize();
    }
  }

  async function close() {
    await getCurrentWindow().close();
  }
</script>

<div class="titlebar" data-tauri-drag-region class:mac={isMac} class:win={!isMac}>
  {#if isMac}
    <div class="lights" data-tauri-drag-region>
      <button type="button" class="light close" aria-label="Close" onclick={close}></button>
      <button type="button" class="light minimize" aria-label="Minimize" onclick={minimize}></button>
      <button type="button" class="light maximize" aria-label="Maximize" onclick={toggleMaximize}></button>
    </div>
    <div class="brand mac-brand" data-tauri-drag-region>
      <span class="app-title">Diary</span>
    </div>
    <div class="spacer" data-tauri-drag-region></div>
  {:else}
    <div class="brand" data-tauri-drag-region>
      <span class="app-title">Diary</span>
    </div>
    <div class="spacer" data-tauri-drag-region></div>
    <div class="lights" data-tauri-drag-region>
      <button type="button" class="light minimize" aria-label="Minimize" onclick={minimize}></button>
      <button type="button" class="light maximize" aria-label="Maximize" onclick={toggleMaximize}></button>
      <button type="button" class="light close" aria-label="Close" onclick={close}></button>
    </div>
  {/if}
</div>

<style>
  .titlebar {
    height: 2.5rem;
    display: flex;
    align-items: center;
    padding: 0 0.75rem;
    flex-shrink: 0;
  }
  .spacer {
    flex: 1;
  }
  .lights {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    -webkit-app-region: no-drag;
  }

  .light {
    width: 0.75rem;
    height: 0.75rem;
    border-radius: 9999px;
    border: none;
    padding: 0;
    cursor: pointer;
    transition:
      transform 120ms cubic-bezier(0.34, 1.56, 0.64, 1),
      opacity 120ms ease,
      background-color 120ms ease;
  }
  .light:hover {
    transform: scale(1.12);
  }
  .light:hover::after {
    color: rgba(0, 0, 0, 0.55);
    font-size: 0.625rem;
    line-height: 0.75rem;
    font-weight: 700;
    position: relative;
    display: block;
    text-align: center;
  }

  /* macOS: classic colored traffic lights, symbols appear on hover */
  .mac .light.close {
    background-color: #ff5f57;
  }
  .mac .light.minimize {
    background-color: #ffbd2e;
  }
  .mac .light.maximize {
    background-color: #28c840;
  }
  .mac .light {
    opacity: 0.9;
  }
  .mac .light:hover {
    opacity: 1;
  }
  .mac .light.close:hover::after { content: "×"; }
  .mac .light.minimize:hover::after { content: "−"; }
  .mac .light.maximize:hover::after { content: "+"; }

  /* Windows/Linux: muted dots, color on hover */
  .win .light {
    background-color: var(--ink-faint);
  }
  .win .light.close:hover {
    background-color: #e81123;
  }
  .win .light.minimize:hover,
  .win .light.maximize:hover {
    background-color: var(--ink-soft);
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    user-select: none;
    pointer-events: none;
  }
  .mac-brand {
    margin-left: 0.5rem;
  }
  .app-title {
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 11px;
    font-weight: 700;
    color: var(--ink-soft);
    letter-spacing: 0.05em;
    text-transform: uppercase;
  }
</style>
