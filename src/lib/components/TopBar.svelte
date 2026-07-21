<script lang="ts">
  import Calendar from "lucide-svelte/icons/calendar";
  import Settings from "lucide-svelte/icons/settings";
  import ViewSwitch from "./ViewSwitch.svelte";
  import CalendarPopover from "./CalendarPopover.svelte";
  import { getEntry } from "$lib/stores/journal.svelte";
  import {
    selectDate,
  } from "$lib/stores/selected-date.svelte";
  import { currentView, setView } from "$lib/stores/view.svelte";

  let calendarOpen = $state(false);

  function toggleCalendar() {
    calendarOpen = !calendarOpen;
  }

  function pickDate(key: string) {
    selectDate(key);
    if (currentView() !== "today") setView("today");
    calendarOpen = false;
  }

  function hasEntry(key: string): boolean {
    return getEntry(key).trim().length > 0;
  }
</script>

<div class="topbar">
  <div class="cal-anchor">
    <button
      type="button"
      class="icon-btn"
      aria-label="Open date picker"
      aria-expanded={calendarOpen}
      onclick={toggleCalendar}
    >
      <Calendar size={16} />
    </button>
    {#if calendarOpen}
      <CalendarPopover
        onPick={pickDate}
        onClose={() => (calendarOpen = false)}
        {hasEntry}
      />
    {/if}
  </div>

  <ViewSwitch />

  <button
    type="button"
    class="icon-btn"
    aria-label="Settings"
    onclick={() => setView("settings")}
  >
    <Settings size={16} />
  </button>
</div>

<style>
  .topbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.25rem 0.5rem;
    flex-shrink: 0;
  }
  .cal-anchor {
    position: relative;
    z-index: 50;
  }
  .icon-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    height: 2rem;
    border-radius: 8px;
    border: none;
    background: transparent;
    color: var(--ink-soft);
    cursor: pointer;
    transition: color 120ms ease, background-color 120ms ease;
  }
  .icon-btn:hover {
    color: var(--ink);
    background-color: var(--paper-subtle);
  }
  .icon-btn:focus-visible {
    outline: none;
    box-shadow: 0 0 0 2px var(--accent);
  }
</style>
