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
    <svg viewBox="0 0 20 20" width="20" height="20" class="check" class:completing>
      <path d="M6 10l3 3 5-5" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round" />
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
    cursor: default;
    user-select: none;
    gap: 0.625rem;
    transition: opacity 180ms ease;
    width: 100%;
    min-width: 0;
  }
  .row.completing {
    opacity: 0.5;
  }
  .circle {
    position: relative;
    flex-shrink: 0;
    background-color: var(--ink-faint);
    border: none;
    padding: 0;
    cursor: pointer;
    color: var(--ink);
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    outline: none;
    -webkit-appearance: none;
    appearance: none;
    transition:
      background-color 150ms ease,
      color 150ms ease,
      transform 120ms cubic-bezier(0.34, 1.56, 0.64, 1);
  }
  .circle:hover {
    background-color: var(--accent);
  }
  .circle:focus-visible {
    outline: none;
    box-shadow: none;
  }
  .circle:active {
    transform: scale(0.92);
  }
  .circle.completing {
    background-color: var(--accent);
    transform: scale(1.05);
  }
  .check {
    position: absolute;
    top: 0;
    left: 0;
    opacity: 0;
    transform: scale(0.5) rotate(-12deg);
    transition:
      opacity 160ms ease,
      transform 220ms cubic-bezier(0.34, 1.56, 0.64, 1);
  }
  .check.completing {
    opacity: 1;
    transform: scale(1) rotate(0deg);
  }
  .circle:hover .check {
    opacity: 1;
    transform: scale(1) rotate(0deg);
  }
  .text {
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 14px;
    color: var(--ink);
    line-height: 1.4;
    transition: color 150ms ease;
    min-width: 0;
    flex: 1 1 auto;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .row.completing .text {
    color: var(--ink-soft);
    text-decoration: line-through;
  }
  .age {
    flex-shrink: 0;
    display: inline-flex;
    align-items: center;
    gap: 0.2rem;
    margin-left: auto;
    padding: 0.1rem 0.35rem 0.1rem 0.3rem;
    border-radius: 9999px;
    background: var(--paper);
    color: var(--ink-soft);
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 11px;
    font-weight: 700;
    line-height: 1;
    letter-spacing: 0.01em;
  }
  .age-n {
    font-variant-numeric: tabular-nums;
  }
</style>
