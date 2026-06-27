<script lang="ts">
  import {
    habits,
    isChecked,
    toggleCheck,
  } from "$lib/stores/habits.svelte";
  import HabitRow from "./HabitRow.svelte";

  /**
   * HabitList — the right 30% rail. Reads habit defs + check states from the
   * habits store and wires each row's toggle. Layout shell only — no store
   * logic lives here beyond calling the store accessors.
   */
  let { dateKey = "" } = $props();

  const list = $derived(habits());
</script>

<div class="list">
  {#each list as habit (habit.id)}
    <HabitRow
      label={habit.label}
      checked={isChecked(habit.id, dateKey)}
      onToggle={() => void toggleCheck(habit.id, dateKey)}
    />
  {/each}
</div>

<style>
  .list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
</style>