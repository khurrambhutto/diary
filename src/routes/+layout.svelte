<script lang="ts">
  import "../app.css";

  // Newsreader — journal body (serif)
  import "@fontsource/newsreader/400.css";
  import "@fontsource/newsreader/400-italic.css";
  import "@fontsource/newsreader/500.css";
  // Atkinson Hyperlegible — UI chrome (sans)
  import "@fontsource/atkinson-hyperlegible/400.css";
  import "@fontsource/atkinson-hyperlegible/700.css";

  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import TitleBar from "$lib/components/TitleBar.svelte";
  import ViewSwitch from "$lib/components/ViewSwitch.svelte";
  import { setViewFromPath } from "$lib/stores/view.svelte";

  type UnlistenFn = () => void;

  let maximized = $state(false);
  let unlisten: UnlistenFn | null = null;

  // Seed the view rune from the URL once on first render.
  $effect(() => {
    setViewFromPath($page.url.pathname);
  });

  onMount(() => {
    const win = getCurrentWindow();
    win.isMaximized().then((m) => (maximized = m));
    win.onResized(() => {
      win.isMaximized().then((m) => (maximized = m));
    }).then((fn) => {
      unlisten = fn as UnlistenFn;
    });
    return () => {
      unlisten?.();
    };
  });

  let { children } = $props();
</script>

<div class="window-frame" class:maximized>
  <div class="app-shell">
    <TitleBar />

    <main class="flex-1 flex flex-col min-h-0" style="padding: 1.5rem;">
      <div class="h-8 flex items-center justify-center">
        <ViewSwitch />
      </div>

      {@render children()}
    </main>
  </div>
</div>

<style>
  .window-frame {
    height: 100vh;
    width: 100vw;
    background: transparent;
    padding: 0.375rem;
    box-sizing: border-box;
    transition: padding 120ms ease;
  }
  .window-frame.maximized {
    padding: 0;
  }
  .app-shell {
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
    background-color: var(--paper);
    color: var(--ink);
    border-radius: 12px;
    overflow: hidden;
    box-shadow:
      0 0 0 1px var(--line),
      0 10px 40px rgba(0, 0, 0, 0.18);
    transition: border-radius 120ms ease, box-shadow 120ms ease;
  }
  .maximized .app-shell {
    border-radius: 0;
    box-shadow: none;
  }
</style>