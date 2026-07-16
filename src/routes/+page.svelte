<script lang="ts">
  import { onMount } from "svelte";
  import Editor from "$lib/components/Editor.svelte";
  import HabitList from "$lib/components/HabitList.svelte";
  import TodoList from "$lib/components/TodoList.svelte";
  import DateHeader from "$lib/components/DateHeader.svelte";
  import {
    initJournalStore,
    isJournalHydrated,
    getEntry,
  } from "$lib/stores/journal.svelte";
  import {
    initHabitsStore,
    isHabitsHydrated,
  } from "$lib/stores/habits.svelte";
  import {
    initTodosStore,
    isTodosHydrated,
  } from "$lib/stores/todos.svelte";
  import {
    initSelectedDate,
    selectedKey,
    selectDate,
    selectToday,
    isViewingToday,
  } from "$lib/stores/selected-date.svelte";

  let ready = $state(false);

  onMount(async () => {
    await Promise.all([
      initJournalStore(),
      initHabitsStore(),
      initTodosStore(),
    ]);
    initSelectedDate();
    ready = true;
  });

  const key = $derived(selectedKey());
  const dateKey = $derived(selectedKey());
  const readonly = $derived(!isViewingToday());

  const hydrated = $derived(
    isJournalHydrated() && isHabitsHydrated() && isTodosHydrated()
  );
</script>

{#if ready && hydrated}
  <div class="canvas">
    <section class="journal">
      <DateHeader
        selectedKey={key}
        isToday={isViewingToday()}
        onPickDate={(k) => selectDate(k)}
        onBackToToday={() => selectToday()}
        hasEntry={(k) => getEntry(k).trim().length > 0}
      />
      <div class="editor-wrap">
        <Editor {dateKey} {readonly} />
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
    padding: 0.25rem 0.5rem;
    overscroll-behavior: contain;
    scrollbar-width: thin;
    scrollbar-color: var(--ink-faint) transparent;
  }
  .sidebar-scroll::-webkit-scrollbar {
    width: 6px;
  }
  .sidebar-scroll::-webkit-scrollbar-track {
    background: transparent;
  }
  .sidebar-scroll::-webkit-scrollbar-thumb {
    background: transparent;
    border-radius: 3px;
  }
  .sidebar-scroll:hover::-webkit-scrollbar-thumb {
    background: var(--ink-faint);
  }
  .divider {
    height: 1px;
    background-color: var(--line);
    flex-shrink: 0;
    margin: 0.25rem 0;
  }
</style>
