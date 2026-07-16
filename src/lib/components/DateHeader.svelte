<script lang="ts">
  import { format, parse } from "date-fns";
  import CalendarPopover from "./CalendarPopover.svelte";

  let {
    selectedKey,
    isToday,
    onPickDate,
    onBackToToday,
    hasEntry,
  }: {
    selectedKey: string;
    isToday: boolean;
    onPickDate: (key: string) => void;
    onBackToToday: () => void;
    hasEntry: (key: string) => boolean;
  } = $props();

  let open = $state(false);

  const dateLabel = $derived(
    format(parse(selectedKey, "yyyy-MM-dd", new Date()), "EEEE, MMMM d")
  );

  function toggle() {
    open = !open;
  }

  function pick(key: string) {
    onPickDate(key);
    open = false;
  }

  function close() {
    open = false;
  }
</script>

<div class="date-header">
  <div class="anchor">
    <button type="button" class="date-btn" aria-label="Open date picker" onclick={toggle}>
      <time datetime={selectedKey}>{dateLabel}</time>
    </button>

    {#if open}
      <CalendarPopover
        onPick={pick}
        onClose={close}
        {hasEntry}
      />
    {/if}
  </div>

  {#if !isToday}
    <button type="button" class="back-btn" onclick={onBackToToday}>
      <svg viewBox="0 0 20 20" width="14" height="14" fill="currentColor">
        <path d="M10 3a1 1 0 0 1 .707.293l4 4a1 1 0 0 1-1.414 1.414L11 6.414V16a1 1 0 1 1-2 0V6.414L6.707 8.707a1 1 0 0 1-1.414-1.414l4-4A1 1 0 0 1 10 3z" />
      </svg>
      Today
    </button>
  {/if}
</div>

<style>
  .date-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 2.5rem 1rem 2.5rem;
  }
  .anchor {
    position: relative;
  }
  .date-btn {
    background: none;
    border: none;
    padding: 0;
    cursor: pointer;
    border-radius: 4px;
    font: inherit;
    transition: background-color 120ms ease;
  }
  .date-btn:hover {
    background-color: var(--paper-subtle);
  }
  .date-btn time {
    font-family: "Newsreader", Georgia, serif;
    font-style: italic;
    font-size: 16px;
    color: var(--ink-soft);
    display: block;
    padding: 0.125rem 0.25rem;
  }
  .back-btn {
    background: none;
    border: none;
    padding: 0.25rem 0.5rem;
    cursor: pointer;
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 12px;
    color: var(--ink-soft);
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    border-radius: 4px;
    transition: color 120ms ease, background-color 120ms ease;
  }
  .back-btn:hover {
    color: var(--accent);
    background-color: var(--paper-subtle);
  }
</style>
