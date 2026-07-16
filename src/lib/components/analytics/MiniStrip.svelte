<script lang="ts">
  let {
    cells,
    cellWidth = 6,
    height = 14,
  }: {
    cells: { dateKey: string; level: number; label: string }[];
    cellWidth?: number;
    height?: number;
  } = $props();

  const gap = 2;
  const totalWidth = $derived(cells.length * (cellWidth + gap));

  function fill(level: number): string {
    if (level === 0) return "var(--paper-subtle)";
    if (level === 1) return "var(--accent-soft)";
    if (level === 2 || level === 3) return "var(--accent)";
    return "var(--paper-subtle)";
  }

  function cellOpacity(level: number): string {
    if (level === 2) return "0.5";
    return "1";
  }
</script>

<svg
  width={totalWidth}
  {height}
  role="img"
  aria-label="Activity mini-strip"
>
  {#each cells as cell, i}
    <g>
      <title>{cell.label}</title>
      <rect
        x={i * (cellWidth + gap)}
        y="0"
        width={cellWidth}
        height={height}
        rx="2"
        fill={fill(cell.level)}
        opacity={cellOpacity(cell.level)}
      />
    </g>
  {/each}
</svg>

<style>
  svg {
    display: block;
  }
</style>
