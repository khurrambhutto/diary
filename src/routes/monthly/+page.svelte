<script lang="ts">
  import { onMount } from "svelte";
  import { format, subDays, startOfDay } from "date-fns";
  import WritingSummary from "$lib/components/analytics/WritingSummary.svelte";
  import Heatmap from "$lib/components/analytics/Heatmap.svelte";
  import HabitStatCard from "$lib/components/analytics/HabitStatCard.svelte";
  import {
    initJournalStore, isJournalHydrated,
    entryWordCount, writingStats,
  } from "$lib/stores/journal.svelte";
  import {
    initHabitsStore, isHabitsHydrated,
    habits, habitStreak, habitBestStreak,
    habitCheckCount, habitCompletionRate, allChecksForDate, isChecked,
  } from "$lib/stores/habits.svelte";
  import { initTodosStore, isTodosHydrated } from "$lib/stores/todos.svelte";

  let ready = $state(false);

  onMount(async () => {
    await Promise.all([initJournalStore(), initHabitsStore(), initTodosStore()]);
    ready = true;
  });

  const hydrated = $derived(isJournalHydrated() && isHabitsHydrated() && isTodosHydrated());
  const stats = $derived(writingStats());

  const today = startOfDay(new Date());

  function makeStripCells(daysBack: number, getLevel: (dk: string) => number, getLabel: (dk: string, d: Date) => string) {
    const cells: { dateKey: string; level: number; label: string }[] = [];
    for (let i = daysBack; i >= 0; i--) {
      const d = subDays(today, i);
      const dk = format(d, "yyyy-MM-dd");
      cells.push({ dateKey: dk, level: getLevel(dk), label: getLabel(dk, d) });
    }
    return cells;
  }

  const writingStripCells = $derived(makeStripCells(59,
    (dk) => { const w = entryWordCount(dk); return w === 0 ? 0 : w < 50 ? 1 : w < 250 ? 2 : 3; },
    (dk, d) => `${format(d, "EEEE, MMM d")}: ${entryWordCount(dk)} words`,
  ));

  const heatmapCells = $derived(makeStripCells(364,
    (dk) => { const { checked } = allChecksForDate(dk); return Math.min(checked, 10); },
    (dk, d) => { const { checked, total } = allChecksForDate(dk); return `${format(d, "EEEE, MMM d")}: ${checked}/${total} habits`; },
  ));

  const habitCards = $derived(habits().map((h) => ({
    id: h.id, label: h.label,
    streak: habitStreak(h.id), best: habitBestStreak(h.id),
    total: habitCheckCount(h.id), completionRate: habitCompletionRate(h.id, 30),
    miniStripCells: makeStripCells(29,
      (dk) => (isChecked(h.id, dk) ? 3 : 0),
      (dk, d) => `${format(d, "EEEE, MMM d")}: ${isChecked(h.id, dk) ? "checked" : "missed"}`,
    ),
  })));
</script>

{#if ready && hydrated}
  <div class="analytics">
    <WritingSummary totalEntries={stats.totalEntries} currentStreak={stats.currentStreak}
      bestStreak={stats.bestStreak} thisMonth={stats.thisMonth} stripCells={writingStripCells} />
    <div class="section">
      <h3 class="section-header">Habit heatmap</h3>
      <Heatmap cells={heatmapCells} />
    </div>
    <div class="section">
      <h3 class="section-header">Per-habit analytics</h3>
      {#if habitCards.length === 0}
        <p class="empty">Add a habit on the Today page to see analytics.</p>
      {:else}
        <div class="habit-grid">
          {#each habitCards as card}
            <HabitStatCard label={card.label} streak={card.streak} best={card.best}
              total={card.total} completionRate={card.completionRate} miniStripCells={card.miniStripCells} />
          {/each}
        </div>
      {/if}
    </div>
  </div>
{:else}
  <div class="flex-1 min-h-0"></div>
{/if}

<style>
  .analytics {
    max-width: 46rem;
    margin: 0 auto;
    width: 100%;
    height: 100%;
    overflow-y: auto;
    padding-bottom: 2rem;
  }
  .page-title { font-family: "Newsreader", Georgia, serif; font-style: italic; font-size: 16px; font-weight: 400; color: var(--ink-soft); margin: 0 0 1.5rem 0; }
  .section { margin-bottom: 1.5rem; }
  .section-header { font-family: "Atkinson Hyperlegible", system-ui, sans-serif; font-size: 14px; font-weight: 700; color: var(--ink); letter-spacing: 0.02em; text-transform: uppercase; margin: 0 0 0.75rem 0; }
  .empty { font-family: "Newsreader", Georgia, serif; font-style: italic; font-size: 15px; color: var(--ink-soft); margin: 0.5rem 0; }
  .habit-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 0.875rem;
  }
</style>
