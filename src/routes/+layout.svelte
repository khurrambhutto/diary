<script lang="ts">
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
  import TopBar from "$lib/components/TopBar.svelte";
  import { setViewFromPath } from "$lib/stores/view.svelte";

  type UnlistenFn = () => void;

  let maximized = $state(false);
  let unlisten: UnlistenFn | null = null;

  function routeIndex(pathname: string): number {
    if (pathname.startsWith("/monthly")) return 1;
    if (pathname.startsWith("/settings")) return 2;
    return 0;
  }

  // Keep the view rune in sync; swipe old page out / new page in.
  onNavigate((navigation) => {
    if (navigation.to) {
      setViewFromPath(navigation.to.url.pathname);
    }
    if (!("startViewTransition" in document)) return;
    if (!navigation.from || !navigation.to) return;

    const from = routeIndex(navigation.from.url.pathname);
    const to = routeIndex(navigation.to.url.pathname);
    if (from === to) return;

    const dir = to > from ? "forward" : "back";
    document.documentElement.dataset.navDir = dir;

    return new Promise((resolve) => {
      const transition = document.startViewTransition(async () => {
        resolve();
        await navigation.complete;
      });
      transition.finished.finally(() => {
        delete document.documentElement.dataset.navDir;
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

    <main class="flex-1 flex flex-col min-h-0" style="padding: 0;">
      <TopBar />

      <div class="page-surface" style="padding: 0 0.375rem 0.375rem;">
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
    padding: 0.25rem;
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
    overflow-y: auto;
    view-transition-name: page;
  }

  :global(::view-transition-old(root)),
  :global(::view-transition-new(root)) {
    animation: none;
    mix-blend-mode: normal;
  }

  :global(::view-transition-group(page)) {
    overflow: hidden;
  }

  :global(::view-transition-old(page)),
  :global(::view-transition-new(page)) {
    animation-duration: 280ms;
    animation-timing-function: cubic-bezier(0.32, 0.72, 0, 1);
    animation-fill-mode: both;
  }

  :global(html[data-nav-dir="forward"]::view-transition-old(page)) {
    animation-name: swipe-out-left;
  }
  :global(html[data-nav-dir="forward"]::view-transition-new(page)) {
    animation-name: swipe-in-right;
  }
  :global(html[data-nav-dir="back"]::view-transition-old(page)) {
    animation-name: swipe-out-right;
  }
  :global(html[data-nav-dir="back"]::view-transition-new(page)) {
    animation-name: swipe-in-left;
  }

  @keyframes swipe-out-left {
    to {
      transform: translateX(-28%);
      opacity: 0;
    }
  }
  @keyframes swipe-in-right {
    from {
      transform: translateX(28%);
      opacity: 0;
    }
  }
  @keyframes swipe-out-right {
    to {
      transform: translateX(28%);
      opacity: 0;
    }
  }
  @keyframes swipe-in-left {
    from {
      transform: translateX(-28%);
      opacity: 0;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    :global(::view-transition-old(page)),
    :global(::view-transition-new(page)) {
      animation-duration: 1ms;
    }
  }
</style>
