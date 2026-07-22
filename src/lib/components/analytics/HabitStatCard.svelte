<script lang="ts">
  import MiniStrip from "./MiniStrip.svelte";

  let {
    label,
    streak,
    best,
    total,
    completionRate,
    miniStripCells,
  }: {
    label: string;
    streak: number;
    best: number;
    total: number;
    completionRate: number;
    miniStripCells: { dateKey: string; level: number; label: string }[];
  } = $props();

  const pct = $derived(Math.round(completionRate * 100));
</script>

<div class="habit-card">
  <div class="card-top">
    <span class="habit-name">{label}</span>
    <span class="streak-badge" class:active={streak > 0}>
      {#if streak > 0}🔥 {streak}d streak{:else}0d streak{/if}
    </span>
  </div>

  <div class="metrics-row">
    <div class="metric-item">
      <span class="metric-num">{pct}%</span>
      <span class="metric-label">30d rate</span>
    </div>
    <div class="metric-sep"></div>
    <div class="metric-item">
      <span class="metric-num">{best}d</span>
      <span class="metric-label">best streak</span>
    </div>
    <div class="metric-sep"></div>
    <div class="metric-item">
      <span class="metric-num">{total}</span>
      <span class="metric-label">total done</span>
    </div>
  </div>

  <div class="strip-section">
    <MiniStrip cells={miniStripCells} cellWidth={7} height={14} />
  </div>
</div>

<style>
  .habit-card {
    background: var(--paper-subtle);
    border: 1px solid var(--paper-line);
    border-radius: 12px;
    padding: 0.875rem 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.625rem;
    transition: transform 140ms ease, border-color 140ms ease;
  }
  .habit-card:hover {
    border-color: var(--accent-soft);
    transform: translateY(-1px);
  }
  .card-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
  }
  .habit-name {
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 14px;
    font-weight: 700;
    color: var(--ink);
  }
  .streak-badge {
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 11px;
    font-weight: 700;
    padding: 2px 8px;
    border-radius: 999px;
    background: var(--paper);
    color: var(--ink-soft);
    border: 1px solid var(--paper-line);
  }
  .streak-badge.active {
    background: color-mix(in srgb, var(--accent) 15%, transparent);
    color: var(--accent);
    border-color: color-mix(in srgb, var(--accent) 30%, transparent);
  }
  .metrics-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: var(--paper);
    border-radius: 8px;
    padding: 0.375rem 0.625rem;
  }
  .metric-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    flex: 1;
  }
  .metric-num {
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 13px;
    font-weight: 700;
    color: var(--ink);
    font-variant-numeric: tabular-nums;
  }
  .metric-label {
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 10px;
    color: var(--ink-soft);
    text-transform: uppercase;
    letter-spacing: 0.03em;
  }
  .metric-sep {
    width: 1px;
    height: 18px;
    background: var(--paper-line);
  }
  .strip-section {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
    width: 100%;
  }
  .strip-meta {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .strip-title {
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 10px;
    font-weight: 700;
    color: var(--ink-soft);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }
</style>
