<script lang="ts">
  import { startOfWeek, addDays, format, subDays, startOfDay } from "date-fns";

  let {
    cells,
  }: {
    cells: { dateKey: string; level: number; label: string }[];
  } = $props();

  const cellSize = 12;
  const gap = 2;
  const labelWidth = 16;

  const cellMap = $derived.by(() => {
    const map = new Map<string, { level: number; label: string }>();
    for (const c of cells) {
      map.set(c.dateKey, c);
    }
    return map;
  });

  const grid = $derived.by(() => {
    const today = startOfDay(new Date());
    const start = subDays(today, 364);
    const weekStart = startOfWeek(start, { weekStartsOn: 1 });

    const rows: { dateKey: string; level: number; label: string; empty: boolean }[][] = [];

    let cursor = weekStart;
    while (cursor <= today) {
      const col: { dateKey: string; level: number; label: string; empty: boolean }[] = [];
      for (let d = 0; d < 7; d++) {
        const day = addDays(cursor, d);
        const dk = format(day, "yyyy-MM-dd");
        if (day < start || day > today) {
          col.push({ dateKey: dk, level: 0, label: "", empty: true });
        } else {
          const existing = cellMap.get(dk);
          col.push({
            dateKey: dk,
            level: existing?.level ?? 0,
            label: existing?.label ?? "",
            empty: false,
          });
        }
      }
      rows.push(col);
      cursor = addDays(cursor, 7);
    }
    return rows;
  });

  const cols = $derived(grid.length);
  const totalWidth = $derived(cols * (cellSize + gap) + labelWidth);
  const totalHeight = $derived(7 * (cellSize + gap));

  const dayLabels = ["Mon", "", "Wed", "", "Fri", "", ""];

  const monthLabels = $derived.by(() => {
    const labels: { col: number; label: string }[] = [];
    const monthNames = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let lastMonth = -1;
    for (let c = 0; c < grid.length; c++) {
      const nonEmpty = grid[c].find((cell) => !cell.empty);
      if (nonEmpty) {
        const d = new Date(nonEmpty.dateKey + "T00:00:00");
        const m = d.getMonth();
        if (m !== lastMonth) {
          labels.push({ col: c, label: monthNames[m] });
          lastMonth = m;
        }
      }
    }
    return labels;
  });

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

<div class="heatmap-wrap">
  <svg
    width={totalWidth}
    height={totalHeight + 24}
    role="img"
    aria-label="Habit activity over the last year"
  >
    {#each monthLabels as ml}
      <text
        x={labelWidth + ml.col * (cellSize + gap)}
        y="10"
        font-family="Atkinson Hyperlegible, system-ui, sans-serif"
        font-size="10"
        fill="var(--ink-faint)"
      >{ml.label}</text>
    {/each}

    {#each dayLabels as dl, ri}
      {#if dl}
        <text
          x="0"
          y={24 + ri * (cellSize + gap) + 10}
          font-family="Atkinson Hyperlegible, system-ui, sans-serif"
          font-size="10"
          fill="var(--ink-faint)"
        >{dl}</text>
      {/if}
    {/each}

    {#each grid as col, ci}
      {#each col as cell, ri}
        {#if !cell.empty}
          <g>
            <title>{cell.label}</title>
            <rect
              x={labelWidth + ci * (cellSize + gap)}
              y={24 + ri * (cellSize + gap)}
              width={cellSize}
              height={cellSize}
              rx="2"
              fill={fill(cell.level)}
              opacity={cellOpacity(cell.level)}
            />
          </g>
        {/if}
      {/each}
    {/each}
  </svg>
</div>

<style>
  .heatmap-wrap {
    overflow-x: auto;
  }
</style>
