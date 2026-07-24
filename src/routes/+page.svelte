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
    allChecksForDate,
  } from "$lib/stores/habits.svelte";
  import {
    initTodosStore,
    isTodosHydrated,
    todoItems,
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

  const habitStats = $derived(allChecksForDate(key));
  const todosCount = $derived(todoItems().length);

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
            <div class="card-header">
              <h3 class="card-title">Habits</h3>
              {#if habitStats.total > 0}
                <span class="card-badge">{habitStats.checked}/{habitStats.total} done</span>
              {/if}
            </div>
            <HabitList dateKey={key} />
          </section>

          <section class="card todo-card">
            <div class="card-header">
              <h3 class="card-title">To Do</h3>
              {#if todosCount > 0}
                <span class="card-badge">{todosCount} {todosCount === 1 ? 'task' : 'tasks'}</span>
              {/if}
            </div>
            <TodoList />
          </section>
        </div>

        <section class="card note-card">
          <div class="card-header note-header">
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
    gap: 0.5rem;
  }
  .panels-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.5rem;
    flex-shrink: 0;
  }
  .card {
    background: var(--paper-subtle);
    border: 1px solid var(--paper-line);
    border-radius: 14px;
    padding: 0.75rem 1rem;
    display: flex;
    flex-direction: column;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.03), 0 4px 12px rgba(0, 0, 0, 0.02);
    transition: border-color 150ms ease, box-shadow 150ms ease;
  }
  .card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.5rem;
  }
  .card-title {
    margin: 0;
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 11px;
    font-weight: 700;
    color: var(--ink-soft);
    letter-spacing: 0.05em;
    text-transform: uppercase;
  }
  .card-badge {
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 11px;
    font-weight: 600;
    color: var(--ink-soft);
    background: var(--paper);
    border: 1px solid var(--paper-line);
    border-radius: 999px;
    padding: 1px 7px;
    font-variant-numeric: tabular-nums;
  }
  .note-card {
    flex: 1 1 auto;
    min-height: 0;
    padding: 0.5rem 0.5rem 0.5rem;
  }
  .note-header {
    margin-bottom: 0.375rem;
    flex-shrink: 0;
  }
  .editor-wrap {
    flex: 1 1 auto;
    min-height: 0;
    background: var(--paper);
    border: 1px solid var(--paper-line);
    border-radius: 10px;
    overflow: hidden;
  }
</style>
