<script lang="ts">
  import { flip } from "svelte/animate";
  import { slide } from "svelte/transition";
  import {
    habits,
    isChecked,
    toggleCheck,
    deleteHabit,
    habitStreak,
  } from "$lib/stores/habits.svelte";
  import HabitRow from "./HabitRow.svelte";
  import HabitAddForm from "./HabitAddForm.svelte";

  let { dateKey = "" } = $props();

  const list = $derived(habits());
</script>

<div class="habit-section">
  <div class="header">
    <h3 class="title">Habits</h3>
  </div>

  <div class="list">
    {#if list.length === 0}
      <p class="empty">No habits yet</p>
    {:else}
      {#each list as habit (habit.id)}
        <div
          class="row-wrap"
          animate:flip={{ duration: 200 }}
          out:slide={{ duration: 200 }}
        >
          <HabitRow
            label={habit.label}
            checked={isChecked(habit.id, dateKey)}
            streak={habitStreak(habit.id)}
            onToggle={() => void toggleCheck(habit.id, dateKey)}
            onDelete={() => void deleteHabit(habit.id)}
          />
        </div>
      {/each}
    {/if}
  </div>

  <HabitAddForm />
</div>

<style>
  .habit-section {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .title {
    margin: 0;
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 14px;
    font-weight: 700;
    color: var(--ink);
    letter-spacing: 0.02em;
    text-transform: uppercase;
  }
  .list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  .row-wrap {
    display: flex;
  }
  .empty {
    margin: 0;
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 13px;
    color: var(--ink-soft);
    font-style: italic;
  }
</style>
