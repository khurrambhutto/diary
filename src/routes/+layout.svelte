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
  import { onNavigate } from "$app/navigation";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import TitleBar from "$lib/components/TitleBar.svelte";
  import ViewSwitch from "$lib/components/ViewSwitch.svelte";
  import { setViewFromPath } from "$lib/stores/view.svelte";

  type UnlistenFn = () => void;

  let maximized = $state(false);
  let unlisten: UnlistenFn | null = null;

  // Keep the view rune in sync with the route on every SPA navigation.
  onNavigate((navigation) => {
    if (navigation.to) {
      setViewFromPath(navigation.to.url.pathname);
    }
    if (!("startViewTransition" in document)) return;
    return new Promise((resolve) => {
      document.startViewTransition(async () => {
        resolve();
        await navigation.complete;
      });
    });
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

      <div class="page-surface">
        {@render children()}
      </div>
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
    border-radius: 16px;
    overflow: hidden;
    box-shadow: var(--window-shadow);
    transition: border-radius 120ms ease, box-shadow 120ms ease;
  }
  .maximized .app-shell {
    border-radius: 0;
    box-shadow: none;
  }
  .page-surface {
    flex: 1 1 auto;
    min-height: 0;
    display: flex;
    flex-direction: column;
    view-transition-name: page;
  }

  :global(::view-transition-old(page)),
  :global(::view-transition-new(page)) {
    animation-duration: 220ms;
  }
</style>
