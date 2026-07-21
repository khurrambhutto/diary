<script lang="ts">
  import { onMount } from "svelte";
  import { fly } from "svelte/transition";
  import { cubicOut } from "svelte/easing";
  import Editor from "$lib/components/Editor.svelte";
  import HabitList from "$lib/components/HabitList.svelte";
  import TodoList from "$lib/components/TodoList.svelte";
  import DateHeader from "$lib/components/DateHeader.svelte";
  import {
    initJournalStore,
    isJournalHydrated,
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
    isViewingToday,
  } from "$lib/stores/selected-date.svelte";

  let ready = $state(false);
  let swipeDir = $state(1);
  let lastKey = $state<string | null>(null);
  let allowSwipe = $state(false);

  onMount(async () => {
    await Promise.all([
      initJournalStore(),
      initHabitsStore(),
      initTodosStore(),
    ]);
    initSelectedDate();
    ready = true;
    requestAnimationFrame(() => {
      allowSwipe = true;
    });
  });

  const key = $derived(selectedKey());
  const dateKey = $derived(key);
  const readonly = $derived(!isViewingToday());

  const hydrated = $derived(
    isJournalHydrated() && isHabitsHydrated() && isTodosHydrated()
  );

  $effect.pre(() => {
    const next = selectedKey();
    if (lastKey !== null && next !== lastKey) {
      swipeDir = next > lastKey ? 1 : -1;
    }
    lastKey = next;
  });

  const swipeIn = $derived(
    allowSwipe
      ? { x: swipeDir * 56, duration: 280, opacity: 0 as number, easing: cubicOut }
      : { x: 0, duration: 0, opacity: 1 as number, easing: cubicOut }
  );
  const swipeOut = $derived({
    x: swipeDir * -56,
    duration: allowSwipe ? 280 : 0,
    opacity: 0 as number,
    easing: cubicOut,
  });
</script>

{#if ready && hydrated}
  <div class="slide-viewport">
    {#key key}
      <div class="today-page" in:fly={swipeIn} out:fly={swipeOut}>
        <DateHeader selectedKey={key} />

        <div class="panels-row">
          <section class="card habits-card">
            <h3 class="card-title">Habits</h3>
            <HabitList dateKey={key} />
          </section>

          <section class="card todo-card">
            <h3 class="card-title">To Do</h3>
            <TodoList />
          </section>
        </div>

        <section class="card note-card">
          <div class="note-header">
            <h3 class="card-title">Daily Note</h3>
          </div>
          <div class="editor-wrap">
            <Editor {dateKey} {readonly} />
          </div>
        </section>
      </div>
    {/key}
  </div>
{:else}
  <div class="flex-1 min-h-0"></div>
{/if}

<style>
  .slide-viewport {
    position: relative;
    flex: 1 1 auto;
    min-height: 0;
    overflow: hidden;
  }
  .today-page {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }
  .panels-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.375rem;
    flex-shrink: 0;
  }
  .card {
    background: var(--paper-subtle);
    border-radius: 10px;
    padding: 0.625rem 0.875rem;
    display: flex;
    flex-direction: column;
  }
  .card-title {
    margin: 0 0 0.5rem;
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 13px;
    font-weight: 700;
    color: var(--ink);
    letter-spacing: 0.02em;
    text-transform: uppercase;
  }
  .note-card {
    flex: 1 1 auto;
    min-height: 0;
  }
  .note-header {
    margin-bottom: 0.25rem;
    flex-shrink: 0;
  }
  .editor-wrap {
    flex: 1 1 auto;
    min-height: 0;
    background: var(--paper);
    border-radius: 8px;
    overflow: hidden;
  }
</style>
