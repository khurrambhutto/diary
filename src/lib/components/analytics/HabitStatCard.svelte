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

<div class="card">
  <div class="card-header">{label}</div>
  <div class="strip">
    <MiniStrip cells={miniStripCells} cellWidth={6} height={14} />
  </div>
  <div class="stats">
    <span class="stat">
      <span class="stat-value">{streak}</span>
      <span class="stat-label">streak</span>
    </span>
    <span class="sep">·</span>
    <span class="stat">
      <span class="stat-value">{best}</span>
      <span class="stat-label">best</span>
    </span>
    <span class="sep">·</span>
    <span class="stat">
      <span class="stat-value">{total}</span>
      <span class="stat-label">total</span>
    </span>
    <span class="sep">·</span>
    <span class="stat">
      <span class="stat-value">{pct}%</span>
      <span class="stat-label">30d</span>
    </span>
  </div>
</div>

<style>
  .card {
    border-top: 1px solid var(--line);
    padding: 1rem 0;
  }
  .card-header {
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 14px;
    font-weight: 700;
    color: var(--ink);
    margin-bottom: 0.5rem;
  }
  .strip {
    margin-bottom: 0.375rem;
  }
  .stats {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    flex-wrap: wrap;
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 13px;
  }
  .stat {
    display: inline-flex;
    align-items: baseline;
    gap: 0.125rem;
  }
  .stat-value {
    color: var(--ink);
    font-variant-numeric: tabular-nums;
  }
  .stat-label {
    color: var(--ink-soft);
  }
  .sep {
    color: var(--ink-faint);
  }
</style>
