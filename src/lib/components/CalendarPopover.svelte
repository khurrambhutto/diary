<script lang="ts">
  import { fade, scale } from "svelte/transition";
  import {
    startOfMonth,
    endOfMonth,
    startOfWeek,
    endOfWeek,
    eachDayOfInterval,
    isSameMonth,
    isSameDay,
    isToday,
    format,
    addMonths,
    subMonths,
    parse,
  } from "date-fns";

  let {
    onPick,
    onClose,
    hasEntry,
    initialMonth,
  }: {
    onPick: (dateKey: string) => void;
    onClose: () => void;
    hasEntry: (dateKey: string) => boolean;
    initialMonth?: Date;
  } = $props();

  let currentMonth = $state<Date>(new Date());
  let focusKey = $state(format(new Date(), "yyyy-MM-dd"));

  $effect(() => {
    if (initialMonth && initialMonth.getTime() !== currentMonth.getTime()) {
      currentMonth = initialMonth;
    }
  });

  const weeks = $derived.by(() => {
    const start = startOfWeek(startOfMonth(currentMonth), { weekStartsOn: 1 });
    const end = endOfWeek(endOfMonth(currentMonth), { weekStartsOn: 1 });
    const days = eachDayOfInterval({ start, end });
    const rows: Date[][] = [];
    for (let i = 0; i < days.length; i += 7) rows.push(days.slice(i, i + 7));
    return rows;
  });

  const monthLabel = $derived(format(currentMonth, "MMMM yyyy"));

  function shiftMonth(delta: number) {
    currentMonth = delta < 0 ? subMonths(currentMonth, -delta) : addMonths(currentMonth, delta);
  }

  function handlePick(d: Date) {
    focusKey = format(d, "yyyy-MM-dd");
    onPick(focusKey);
  }

  function moveFocus(dx: number, dy: number) {
    const d = parse(focusKey, "yyyy-MM-dd", new Date());
    d.setDate(d.getDate() + dx + dy * 7);
    focusKey = format(d, "yyyy-MM-dd");
    if (!isSameMonth(d, currentMonth)) currentMonth = d;
  }

  function handleKeydown(e: KeyboardEvent) {
    const map: Record<string, () => void> = {
      Escape: onClose,
      ArrowLeft: () => moveFocus(-1, 0),
      ArrowRight: () => moveFocus(1, 0),
      ArrowUp: () => moveFocus(0, -1),
      ArrowDown: () => moveFocus(0, 1),
      Enter: () => onPick(focusKey),
    };
    map[e.key]?.();
    if (map[e.key]) e.preventDefault();
  }
</script>

<div class="backdrop" role="presentation" onclick={onClose}></div>

<div class="popover" role="dialog" aria-label="Date picker" tabindex="-1" transition:fade={{ duration: 140 }} onkeydown={handleKeydown}>
  <div class="popover-inner" transition:scale={{ start: 0.95, duration: 140 }}>
    <div class="header">
      <button type="button" class="nav" aria-label="Previous month" onclick={() => shiftMonth(-1)}>
        <svg viewBox="0 0 20 20" width="14" height="14" fill="currentColor"><path d="M12.707 5.293a1 1 0 0 1 0 1.414L9.414 10l3.293 3.293a1 1 0 0 1-1.414 1.414l-4-4a1 1 0 0 1 0-1.414l4-4a1 1 0 0 1 1.414 0z"/></svg>
      </button>
      <span class="month-label">{monthLabel}</span>
      <button type="button" class="nav" aria-label="Next month" onclick={() => shiftMonth(1)}>
        <svg viewBox="0 0 20 20" width="14" height="14" fill="currentColor"><path d="M7.293 14.707a1 1 0 0 1 0-1.414L10.586 10 7.293 6.707a1 1 0 0 1 1.414-1.414l4 4a1 1 0 0 1 0 1.414l-4 4a1 1 0 0 1-1.414 0z"/></svg>
      </button>
    </div>

    <div class="weekdays">
      {#each ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"] as wd}
        <span class="weekday">{wd}</span>
      {/each}
    </div>

    <div class="grid" role="grid">
      {#each weeks as week}
        <div class="week-row" role="row">
          {#each week as day}
            {@const dk = format(day, "yyyy-MM-dd")}
            {@const inMonth = isSameMonth(day, currentMonth)}
            <button
              type="button"
              class="day"
              class:out-month={!inMonth}
              class:today={isToday(day)}
              class:focused={dk === focusKey && inMonth}
              aria-label={format(day, "EEEE, MMMM d, yyyy")}
              onclick={() => handlePick(day)}
            >
              {format(day, "d")}
              {#if hasEntry(dk)}
                <span class="dot" aria-hidden="true"></span>
              {/if}
            </button>
          {/each}
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .backdrop { position: fixed; inset: 0; z-index: 40; }
  .popover {
    position: absolute;
    top: calc(100% + 0.5rem);
    left: 0;
    z-index: 50;
    width: 16.5rem;
  }
  .popover-inner {
    background: var(--paper-raised);
    border: 1px solid var(--line);
    border-radius: 10px;
    box-shadow: var(--window-shadow);
    padding: 0.75rem;
  }
  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.25rem;
    margin-bottom: 0.5rem;
  }
  .month-label {
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 13px;
    font-weight: 700;
    color: var(--ink);
    white-space: nowrap;
  }
  .nav {
    background: none;
    border: none;
    padding: 0.25rem;
    cursor: pointer;
    color: var(--ink-soft);
    border-radius: 4px;
    display: flex;
    align-items: center;
    flex-shrink: 0;
    transition: color 120ms ease, background-color 120ms ease;
  }
  .nav:hover { color: var(--ink); background-color: var(--paper-subtle); }
  .weekdays {
    display: grid;
    grid-template-columns: repeat(7, minmax(0, 1fr));
    margin-bottom: 0.25rem;
  }
  .weekday {
    text-align: center;
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 11px;
    font-weight: 400;
    color: var(--ink-faint);
    text-transform: uppercase;
    padding: 0.125rem 0;
  }
  .grid { display: flex; flex-direction: column; gap: 0; }
  .week-row {
    display: grid;
    grid-template-columns: repeat(7, minmax(0, 1fr));
  }
  .day {
    position: relative;
    width: 100%;
    aspect-ratio: 1;
    max-width: 2rem;
    height: auto;
    display: flex;
    align-items: center;
    justify-content: center;
    background: none;
    border: 1px solid transparent;
    border-radius: 6px;
    cursor: pointer;
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 12px;
    font-weight: 400;
    color: var(--ink);
    transition: background-color 100ms ease;
    margin: 0 auto;
    padding: 0;
  }
  .day:hover { background-color: var(--paper-subtle); }
  .day.out-month { color: var(--ink-faint); }
  .day.today { font-weight: 700; border-color: var(--accent); background-color: var(--accent-soft); }
  .day.focused { box-shadow: 0 0 0 2px var(--accent); }
  .dot { position: absolute; bottom: 2px; left: 50%; transform: translateX(-50%); width: 3px; height: 3px; border-radius: 50%; background-color: var(--accent); }
</style>
