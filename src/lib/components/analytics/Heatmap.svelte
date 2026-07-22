<script lang="ts">
  import { startOfWeek, addDays, format, subDays, startOfDay } from "date-fns";

  let {
    cells,
  }: {
    cells: { dateKey: string; level: number; label: string }[];
  } = $props();

  const cellSize = 9.5;
  const gap = 2;
  const labelWidth = 26;

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

  const dayLabels = ["Mon", "", "Wed", "", "Fri", "", "Sun"];

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
    if (level === 0) return "var(--line)";
    if (level === 1) return "color-mix(in srgb, var(--accent) 20%, var(--paper-subtle))";
    if (level === 2) return "color-mix(in srgb, var(--accent) 30%, var(--paper-subtle))";
    if (level === 3) return "color-mix(in srgb, var(--accent) 40%, var(--paper-subtle))";
    if (level === 4) return "color-mix(in srgb, var(--accent) 50%, var(--paper-subtle))";
    if (level === 5) return "color-mix(in srgb, var(--accent) 60%, var(--paper-subtle))";
    if (level === 6) return "color-mix(in srgb, var(--accent) 70%, var(--paper-subtle))";
    if (level === 7) return "color-mix(in srgb, var(--accent) 80%, var(--paper-subtle))";
    if (level === 8) return "color-mix(in srgb, var(--accent) 90%, var(--paper-subtle))";
    if (level === 9) return "var(--accent)";
    return "var(--accent-strong)";
  }

  function cellOpacity(level: number): string {
    if (level === 0) return "0.5";
    return "1";
  }
</script>

<div class="heatmap-wrap">
  <svg
    viewBox="0 0 {totalWidth} {totalHeight + 24}"
    style="width: 100%; height: auto; max-width: {totalWidth}px;"
    role="img"
    aria-label="Habit activity over the last year"
  >
    {#each monthLabels as ml}
      <text
        x={labelWidth + ml.col * (cellSize + gap)}
        y="10"
        font-family="Atkinson Hyperlegible, system-ui, sans-serif"
        font-size="9.5"
        font-weight="600"
        fill="var(--ink-soft)"
      >{ml.label}</text>
    {/each}

    {#each dayLabels as dl, ri}
      {#if dl}
        <text
          x="0"
          y={24 + ri * (cellSize + gap) + cellSize / 2}
          dominant-baseline="central"
          font-family="Atkinson Hyperlegible, system-ui, sans-serif"
          font-size="9"
          font-weight="600"
          fill="var(--ink-soft)"
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
    overflow-x: hidden;
    width: 100%;
    display: flex;
    justify-content: center;
  }
  svg {
    display: block;
    margin: 0 auto;
  }
</style>
