<script lang="ts">
  import { flip } from "svelte/animate";
  import { slide } from "svelte/transition";
  import {
    habits,
    isChecked,
    toggleCheck,
    deleteHabit,
    renameHabit,
    habitStreak,
  } from "$lib/stores/habits.svelte";
  import HabitRow from "./HabitRow.svelte";
  import HabitAddForm from "./HabitAddForm.svelte";

  let { dateKey = "" } = $props();

  const list = $derived(habits());
</script>

<div class="habit-section">
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
            onRename={(l) => void renameHabit(habit.id, l)}
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
    gap: 0.5rem;
  }
  .list {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
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
    padding: 0.25rem 0;
  }
</style>
