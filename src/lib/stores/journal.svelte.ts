import { load, type Store } from "@tauri-apps/plugin-store";
import { format, subDays, startOfDay, startOfMonth, endOfMonth, eachDayOfInterval } from "date-fns";

/**
 * Journal store — one plain-text entry per day, keyed `YYYY-MM-DD`.
 * Thin wrapper over @tauri-apps/plugin-store. Components never touch the
 * plugin-store API directly.
 *
 * Keys are always `YYYY-MM-DD` from `date-fns/format` — never invent your
 * own date string formats.
 */

const STORE_PATH = ".diary-store.json";
const JOURNAL_KEY = "journal";

export type JournalStore = Record<string /* YYYY-MM-DD */, string /* entry body */>;

let store: Store | null = null;
let entries = $state<JournalStore>({});
let hydrated = $state(false);

function dateKey(date: Date = new Date()): string {
  return format(date, "yyyy-MM-dd");
}

/** Load the store file and read the journal map into memory.
 *  Call exactly once on app launch before rendering the writing surface. */
export async function initJournalStore(): Promise<void> {
  if (hydrated) return;
  store = await load(STORE_PATH);
  const data = await store.get<JournalStore>(JOURNAL_KEY);
  entries = data ?? {};
  hydrated = true;
}

/** True once `initJournalStore` has completed. Render nothing until this is. */
export function isJournalHydrated(): boolean {
  return hydrated;
}

/** Today's date key in `YYYY-MM-DD`. */
export function todayKey(): string {
  return dateKey();
}

/** Read an entry by date key. Returns "" when none exists. */
export function getEntry(key: string): string {
  return entries[key] ?? "";
}

/** Write an entry by date key and persist it.
 *  Callers should debounce — this does not debounce on its own. */
export async function setEntry(key: string, body: string): Promise<void> {
  entries[key] = body;
  await store?.set(JOURNAL_KEY, entries);
}

/** Reactive snapshot of all entries (for future analytics). */
export function allEntries(): JournalStore {
  return entries;
}

/** Word count for a given day's entry. */
export function entryWordCount(key: string): number {
  const body = entries[key] ?? "";
  if (!body.trim()) return 0;
  return body.trim().split(/\s+/).length;
}

/** Writing stats computed from all entries. */
export function writingStats(): {
  totalEntries: number;
  currentStreak: number;
  bestStreak: number;
  thisMonth: number;
} {
  const keys = Object.keys(entries)
    .filter((k) => (entries[k] ?? "").trim().length > 0)
    .sort();

  const totalEntries = keys.length;

  // current streak: consecutive non-empty days ending today or yesterday
  let currentStreak = 0;
  const today = startOfDay(new Date());
  const todayKey = format(today, "yyyy-MM-dd");
  const yesterdayKey = format(subDays(today, 1), "yyyy-MM-dd");

  if (entries[todayKey]?.trim()) {
    currentStreak++;
  }
  if (!entries[todayKey]?.trim() && entries[yesterdayKey]?.trim()) {
    currentStreak++;
  }

  const startFrom = entries[todayKey]?.trim() ? todayKey : yesterdayKey;
  if (currentStreak > 0) {
    const cursor = subDays(
      new Date(startFrom + "T00:00:00"),
      1
    );
    for (let i = 0; i < 365; i++) {
      const k = format(cursor, "yyyy-MM-dd");
      if (entries[k]?.trim()) {
        currentStreak++;
      } else {
        break;
      }
      cursor.setDate(cursor.getDate() - 1);
    }
  }

  // best streak
  let bestStreak = 0;
  let run = 0;
  for (const k of keys) {
    if ((entries[k] ?? "").trim().length > 0) {
      run++;
      if (run > bestStreak) bestStreak = run;
    } else {
      run = 0;
    }
  }
  // also check if keys are consecutive (they're sorted above)
  // Recalculate properly: walk every day in the date range
  if (keys.length > 0) {
    const firstKey = keys[0];
    const lastKey = keys[keys.length - 1];
    const start = new Date(firstKey + "T00:00:00");
    const end = new Date(lastKey + "T00:00:00");
    const allDays = eachDayOfInterval({ start, end });
    let walkRun = 0;
    let walkBest = 0;
    for (const d of allDays) {
      const k = format(d, "yyyy-MM-dd");
      if ((entries[k] ?? "").trim().length > 0) {
        walkRun++;
        if (walkRun > walkBest) walkBest = walkRun;
      } else {
        walkRun = 0;
      }
    }
    bestStreak = walkBest;
  }

  // this month
  const monthStart = startOfMonth(today);
  const monthEnd = endOfMonth(today);
  let thisMonth = 0;
  const monthDays = eachDayOfInterval({ start: monthStart, end: monthEnd });
  for (const d of monthDays) {
    const k = format(d, "yyyy-MM-dd");
    if ((entries[k] ?? "").trim().length > 0) thisMonth++;
  }

  return { totalEntries, currentStreak, bestStreak, thisMonth };
}