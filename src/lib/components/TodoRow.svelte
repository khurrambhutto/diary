<script lang="ts">
  import Clock from "lucide-svelte/icons/clock";

  let {
    text,
    ageDays = 0,
    onComplete,
  }: {
    text: string;
    ageDays?: number;
    onComplete: () => void;
  } = $props();

  let completing = $state(false);

  function complete() {
    if (completing) return;
    completing = true;
    setTimeout(() => {
      onComplete();
    }, 360);
  }

  const ageLabel = $derived(
    ageDays === 1 ? "1 day old" : `${ageDays} days old`
  );
</script>

<label class="row" class:completing>
  <button
    type="button"
    class="circle"
    class:completing
    aria-label="Complete task"
    onclick={complete}
  >
    <svg viewBox="0 0 20 20" width="18" height="18" class="check" class:completing>
      <path d="M5.5 10l3 3 6-6" stroke="#ffffff" stroke-width="2.5" fill="none" stroke-linecap="round" stroke-linejoin="round" />
    </svg>
  </button>
  <span class="text">{text}</span>
  {#if ageDays > 0}
    <span class="age" title={ageLabel} aria-label={ageLabel}>
      <Clock size={11} strokeWidth={2.25} />
      <span class="age-n">{ageDays}</span>
    </span>
  {/if}
</label>

<style>
  .row {
    display: flex;
    align-items: center;
    cursor: pointer;
    user-select: none;
    gap: 0.625rem;
    padding: 0.35rem 0.5rem;
    margin: 0 -0.5rem;
    border-radius: 8px;
    transition: opacity 180ms ease, background-color 140ms ease;
    width: 100%;
    min-width: 0;
  }
  .row:hover {
    background-color: color-mix(in srgb, var(--ink) 4%, transparent);
  }
  .row.completing {
    opacity: 0.5;
  }
  .circle {
    position: relative;
    flex-shrink: 0;
    background-color: transparent;
    border: 1.5px solid var(--ink-soft);
    padding: 0;
    cursor: pointer;
    color: #ffffff;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    outline: none;
    -webkit-appearance: none;
    appearance: none;
    transition:
      background-color 160ms ease,
      border-color 160ms ease,
      transform 180ms cubic-bezier(0.34, 1.56, 0.64, 1);
  }
  .row:hover .circle:not(.completing) {
    border-color: var(--accent);
    background-color: color-mix(in srgb, var(--accent) 12%, transparent);
  }
  .circle:focus-visible {
    outline: none;
    box-shadow: 0 0 0 2px var(--accent);
  }
  .circle:active {
    transform: scale(0.92);
  }
  .circle.completing {
    background-color: var(--accent);
    border-color: var(--accent);
    transform: scale(1.08);
  }
  .check {
    position: absolute;
    top: -1px;
    left: -1px;
    opacity: 0;
    transform: scale(0.5) rotate(-10deg);
    transition:
      opacity 160ms ease,
      transform 200ms cubic-bezier(0.34, 1.56, 0.64, 1);
  }
  .check.completing {
    opacity: 1;
    transform: scale(1) rotate(0deg);
  }
  .text {
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 14px;
    color: var(--ink);
    line-height: 1.4;
    transition: color 150ms ease, text-decoration 150ms ease;
    min-width: 0;
    flex: 1 1 auto;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .row.completing .text {
    color: var(--ink-soft);
    text-decoration: line-through;
    text-decoration-color: var(--ink-soft);
  }
  .age {
    flex-shrink: 0;
    display: inline-flex;
    align-items: center;
    gap: 0.2rem;
    margin-left: auto;
    padding: 0.1rem 0.4rem;
    border-radius: 9999px;
    background: color-mix(in srgb, var(--ink) 6%, transparent);
    border: 1px solid var(--paper-line);
    color: var(--ink-soft);
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 11px;
    font-weight: 600;
    line-height: 1;
    letter-spacing: 0.01em;
  }
  .age-n {
    font-variant-numeric: tabular-nums;
  }
</style>
