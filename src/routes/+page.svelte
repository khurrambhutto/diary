<script lang="ts">
  import { onMount } from "svelte";
  import { format } from "date-fns";
  import Editor from "$lib/components/Editor.svelte";
  import HabitList from "$lib/components/HabitList.svelte";
  import {
    initJournalStore,
    isJournalHydrated,
    todayKey,
  } from "$lib/stores/journal.svelte";
  import {
    initHabitsStore,
    isHabitsHydrated,
  } from "$lib/stores/habits.svelte";

  /**
   * Today view — layout shell only. Composes Editor + HabitList and reads
   * from stores; no editor or habit logic lives here.
   *
   * Hydration rule: render nothing (blank white) until both stores are loaded.
   */
  let ready = $state(false);
  let key = $state(todayKey());
  const dateLabel = format(new Date(), "EEEE, MMMM d");

  onMount(async () => {
    await Promise.all([initJournalStore(), initHabitsStore()]);
    key = todayKey();
    ready = true;
  });

  const hydrated = $derived(isJournalHydrated() && isHabitsHydrated());
</script>

{#if ready && hydrated}
  <div class="canvas">
    <section class="journal">
      <header class="date-head">
        <time datetime={key}>{dateLabel}</time>
      </header>
      <div class="editor-wrap">
        <Editor dateKey={key} />
      </div>
    </section>

    <aside class="habits">
      <HabitList dateKey={key} />
    </aside>
  </div>
{:else}
  <!-- blank white until hydrated -->
  <div class="flex-1 min-h-0"></div>
{/if}

<style>
  .canvas {
    flex: 1 1 auto;
    min-height: 0;
    display: flex;
    gap: 2rem;
  }
  .journal {
    flex: 1 1 70%;
    min-width: 0;
    display: flex;
    flex-direction: column;
  }
  .date-head {
    padding: 0.5rem 2.5rem 1rem 2.5rem;
  }
  .date-head time {
    font-family: "Newsreader", Georgia, serif;
    font-style: italic;
    font-size: 13px;
    color: var(--ink-soft);
  }
  .editor-wrap {
    flex: 1 1 auto;
    min-height: 0;
  }
  .habits {
    flex: 0 0 30%;
    min-width: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>