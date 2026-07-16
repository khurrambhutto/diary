<script lang="ts">
  import MiniStrip from "./MiniStrip.svelte";

  let {
    totalEntries,
    currentStreak,
    bestStreak,
    thisMonth,
    stripCells,
  }: {
    totalEntries: number;
    currentStreak: number;
    bestStreak: number;
    thisMonth: number;
    stripCells: { dateKey: string; level: number; label: string }[];
  } = $props();
</script>

<div class="writing-section">
  <div class="section-header">Writing</div>
  <div class="stats">
    <span class="stat">
      <span class="stat-value">{totalEntries}</span>
      <span class="stat-label">entries</span>
    </span>
    <span class="sep">·</span>
    <span class="stat">
      <span class="stat-value">{currentStreak}</span>
      <span class="stat-label">day streak</span>
    </span>
    <span class="sep">·</span>
    <span class="stat">
      <span class="stat-value">{thisMonth}</span>
      <span class="stat-label">this month</span>
    </span>
    <span class="sep">·</span>
    <span class="stat">
      <span class="stat-value">{bestStreak}</span>
      <span class="stat-label">best streak</span>
    </span>
  </div>
  <div class="strip">
    <MiniStrip cells={stripCells} />
  </div>
</div>

<style>
  .writing-section {
    margin-bottom: 1.5rem;
  }
  .section-header {
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 14px;
    font-weight: 700;
    color: var(--ink);
    letter-spacing: 0.02em;
    text-transform: uppercase;
    margin-bottom: 0.5rem;
  }
  .stats {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    flex-wrap: wrap;
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 13px;
    margin-bottom: 0.5rem;
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
  .strip {
    margin-top: 0.25rem;
  }
</style>
