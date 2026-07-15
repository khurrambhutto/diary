<script lang="ts">
  import { onMount } from "svelte";
  import { format } from "date-fns";
  import Editor from "$lib/components/Editor.svelte";
  import HabitList from "$lib/components/HabitList.svelte";
  import TodoList from "$lib/components/TodoList.svelte";
  import {
    initJournalStore,
    isJournalHydrated,
    todayKey,
  } from "$lib/stores/journal.svelte";
  import {
    initHabitsStore,
    isHabitsHydrated,
  } from "$lib/stores/habits.svelte";
  import {
    initTodosStore,
    isTodosHydrated,
  } from "$lib/stores/todos.svelte";

  let ready = $state(false);
  let key = $state(todayKey());
  const dateLabel = format(new Date(), "EEEE, MMMM d");

  onMount(async () => {
    await Promise.all([
      initJournalStore(),
      initHabitsStore(),
      initTodosStore(),
    ]);
    key = todayKey();
    ready = true;
  });

  const hydrated = $derived(
    isJournalHydrated() && isHabitsHydrated() && isTodosHydrated()
  );
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

    <aside class="sidebar">
      <div class="sidebar-scroll">
        <HabitList dateKey={key} />
        <div class="divider"></div>
        <TodoList />
      </div>
    </aside>
  </div>
{:else}
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
    font-size: 16px;
    color: var(--ink-soft);
  }
  .editor-wrap {
    flex: 1 1 auto;
    min-height: 0;
  }
  .sidebar {
    flex: 0 0 30%;
    min-width: 0;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  .sidebar-scroll {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
    padding: 0.25rem 0;
  }
  .divider {
    height: 1px;
    background-color: var(--line);
    flex-shrink: 0;
    margin: 0.25rem 0;
  }
</style>
