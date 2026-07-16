import { load, type Store } from "@tauri-apps/plugin-store";
import { format, startOfDay, addDays, subDays } from "date-fns";

const STORE_PATH = ".diary-store.json";
const HABITS_KEY = "habits";

export type HabitDef = { id: string; label: string; createdAt: string };
export type HabitStoreShape = {
  habits: HabitDef[];
  checks: Record<string, Record<string, boolean>>;
};

let store: Store | null = null;
let state = $state<HabitStoreShape>({ habits: [], checks: {} });
let hydrated = $state(false);

const DEFAULT_HABITS: HabitDef[] = [
  { id: "study", label: "Study session", createdAt: new Date().toISOString() },
  { id: "gym", label: "Go to gym", createdAt: new Date().toISOString() },
  { id: "read", label: "Read 10 pages", createdAt: new Date().toISOString() },
];

function dateKey(date: Date = new Date()): string {
  return format(date, "yyyy-MM-dd");
}

export async function initHabitsStore(): Promise<void> {
  if (hydrated) return;
  store = await load(STORE_PATH);
  const data = await store.get<HabitStoreShape>(HABITS_KEY);
  if (data && data.habits) {
    state = { habits: data.habits, checks: data.checks ?? {} };
  } else {
    state = { habits: [...DEFAULT_HABITS], checks: {} };
    await store.set(HABITS_KEY, state);
  }
  hydrated = true;
}

export function isHabitsHydrated(): boolean {
  return hydrated;
}

export function habits(): HabitDef[] {
  return state.habits;
}

export function isChecked(habitId: string, key: string): boolean {
  return state.checks[habitId]?.[key] === true;
}

export async function toggleCheck(habitId: string, dayKey: string): Promise<void> {
  const dayMap = state.checks[habitId] ?? {};
  const next = !dayMap[dayKey];
  dayMap[dayKey] = next;
  state.checks[habitId] = dayMap;
  await store?.set(HABITS_KEY, state);
}

export async function addHabit(label: string): Promise<void> {
  const id = `h-${Date.now().toString(36)}`;
  state.habits = [
    ...state.habits,
    { id, label: label.trim(), createdAt: new Date().toISOString() },
  ];
  await store?.set(HABITS_KEY, state);
}

export async function deleteHabit(id: string): Promise<void> {
  state.habits = state.habits.filter((h) => h.id !== id);
  delete state.checks[id];
  await store?.set(HABITS_KEY, state);
}

export async function renameHabit(id: string, label: string): Promise<void> {
  const trimmed = label.trim();
  if (!trimmed) return;
  state.habits = state.habits.map((h) => (h.id === id ? { ...h, label: trimmed } : h));
  await store?.set(HABITS_KEY, state);
}

/** Compute the current streak for a habit: consecutive days checked. */
export function habitStreak(habitId: string): number {
  const checks = state.checks[habitId] ?? {};
  let streak = 0;
  const cursor = startOfDay(new Date());
  const today = format(cursor, "yyyy-MM-dd");

  if (checks[today] === true) {
    streak++;
  }
  cursor.setDate(cursor.getDate() - 1);

  for (let i = 0; i < 365; i++) {
    const k = format(cursor, "yyyy-MM-dd");
    if (checks[k] === true) {
      streak++;
    } else {
      break;
    }
    cursor.setDate(cursor.getDate() - 1);
  }

  return streak;
}

/** Best streak for a habit over the last year. */
export function habitBestStreak(habitId: string): number {
  const checks = state.checks[habitId] ?? {};
  let current = 0;
  let best = 0;
  const cursor = startOfDay(new Date());
  const end = addDays(cursor, 1);

  for (let i = 0; i < 365; i++) {
    const d = subDays(end, i);
    const k = format(d, "yyyy-MM-dd");
    if (checks[k] === true) {
      current++;
      if (current > best) best = current;
    } else {
      current = 0;
    }
  }

  return best;
}

/** Total number of days this habit was checked. */
export function habitCheckCount(habitId: string): number {
  const checks = state.checks[habitId] ?? {};
  return Object.values(checks).filter(Boolean).length;
}

/** Completion rate (0..1) over the last `sinceDays` days, excluding days before createdAt. */
export function habitCompletionRate(habitId: string, sinceDays = 30): number {
  const habit = state.habits.find((h) => h.id === habitId);
  if (!habit) return 0;
  const created = format(new Date(habit.createdAt), "yyyy-MM-dd");
  const checks = state.checks[habitId] ?? {};
  let checked = 0;
  let total = 0;
  const today = startOfDay(new Date());

  for (let i = 0; i < sinceDays; i++) {
    const d = subDays(today, i);
    const k = format(d, "yyyy-MM-dd");
    if (k < created) continue;
    total++;
    if (checks[k] === true) checked++;
  }

  return total === 0 ? 0 : checked / total;
}

/** For a specific habit, return an array of { dateKey, checked } for a range. */
export function habitChecksForRange(
  habitId: string,
  startDate: Date,
  endDate: Date
): { dateKey: string; checked: boolean }[] {
  const checks = state.checks[habitId] ?? {};
  const result: { dateKey: string; checked: boolean }[] = [];
  const cursor = startOfDay(new Date(startDate));

  while (cursor <= endDate) {
    const k = format(cursor, "yyyy-MM-dd");
    result.push({ dateKey: k, checked: checks[k] === true });
    cursor.setDate(cursor.getDate() + 1);
  }

  return result;
}

/** For a given day, count how many habits were checked and total habits. */
export function allChecksForDate(dayKey: string): { checked: number; total: number } {
  let checked = 0;
  let total = state.habits.length;
  for (const h of state.habits) {
    if (state.checks[h.id]?.[dayKey] === true) checked++;
  }
  return { checked, total };
}
