<script lang="ts">
  import { format, parse } from "date-fns";
  import {
    isViewingToday,
    selectToday,
  } from "$lib/stores/selected-date.svelte";

  let { selectedKey }: { selectedKey: string } = $props();

  const dateLabel = $derived(
    format(parse(selectedKey, "yyyy-MM-dd", new Date()), "EEEE, MMMM d")
  );
  const canJumpToday = $derived(!isViewingToday());

  function handleClick() {
    if (canJumpToday) selectToday();
  }
</script>

<div class="date-heading">
  {#if canJumpToday}
    <button
      type="button"
      class="date-btn"
      aria-label="Go to today"
      title="Go to today"
      onclick={handleClick}
    >
      <time datetime={selectedKey}>{dateLabel}</time>
      <span class="jump-hint">← Jump to today</span>
    </button>
  {:else}
    <time datetime={selectedKey}>{dateLabel}</time>
  {/if}
</div>

<style>
  .date-heading {
    padding: 0.125rem 0.25rem 0.25rem;
    display: flex;
    align-items: center;
  }
  .date-heading time,
  .date-btn time {
    font-family: "Newsreader", Georgia, serif;
    font-weight: 500;
    font-size: 26px;
    color: var(--ink);
    line-height: 1.2;
    letter-spacing: -0.01em;
  }
  .date-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.625rem;
    background: none;
    border: none;
    padding: 0;
    margin: 0;
    cursor: pointer;
    text-align: left;
    border-radius: 6px;
    transition: opacity 140ms ease;
  }
  .date-btn:hover {
    opacity: 0.8;
  }
  .jump-hint {
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 11px;
    font-weight: 600;
    color: var(--ink-soft);
    background: var(--paper-subtle);
    border: 1px solid var(--paper-line);
    border-radius: 999px;
    padding: 2px 8px;
  }
  .date-btn:focus-visible {
    outline: none;
    box-shadow: 0 0 0 2px var(--line-strong);
  }
</style>
