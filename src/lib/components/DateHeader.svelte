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
    </button>
  {:else}
    <time datetime={selectedKey}>{dateLabel}</time>
  {/if}
</div>

<style>
  .date-heading {
    padding: 0.25rem 0.75rem 0;
  }
  .date-heading time,
  .date-btn time {
    font-family: "Newsreader", Georgia, serif;
    font-weight: 500;
    font-size: 26px;
    color: var(--ink);
    line-height: 1.2;
  }
  .date-btn {
    background: none;
    border: none;
    padding: 0;
    margin: 0;
    cursor: pointer;
    text-align: left;
    border-radius: 4px;
    transition: opacity 120ms ease;
  }
  .date-btn:hover {
    opacity: 0.7;
  }
  .date-btn:focus-visible {
    outline: none;
    box-shadow: 0 0 0 2px var(--accent);
  }
</style>
