<script lang="ts">
  type MiniStripProps = {
    cells: { dateKey: string; level: number; label: string }[];
    cellWidth?: number;
    height?: number;
  };

  let { cells, cellWidth = 6, height = 14 }: MiniStripProps = $props();

  const gap = 2.5;
  const totalWidth = $derived(cells.length * (cellWidth + gap));

  function fill(level: number): string {
    if (level === 0) return "var(--line)";
    if (level === 1) return "var(--accent-strong)";
    return "var(--accent)";
  }

  function cellOpacity(level: number): string {
    if (level === 0) return "0.6";
    return "1";
  }
</script>

<svg
  viewBox="0 0 {totalWidth} {height}"
  style="width: 100%; height: auto;"
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
        rx="2.5"
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
