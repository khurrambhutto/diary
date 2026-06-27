import { load, type Store } from "@tauri-apps/plugin-store";

/**
 * Habits store — definitions + per-day check states.
 * Lives in the same `.diary-store.json` file as the journal, under the
 * `habits` key. One concern per store file — this one only owns habits.
 */

const STORE_PATH = ".diary-store.json";
const HABITS_KEY = "habits";

export type HabitDef = { id: string; label: string; createdAt: string };
export type HabitStoreShape = {
  habits: HabitDef[];
  /** outer key = habitId, inner key = YYYY-MM-DD, value = checked? */
  checks: Record<string, Record<string, boolean>>;
};

let store: Store | null = null;
let state = $state<HabitStoreShape>({ habits: [], checks: {} });
let hydrated = $state(false);

/** Default habits seeded on first run (store empty). */
const DEFAULT_HABITS: HabitDef[] = [
  { id: "study", label: "Study session", createdAt: new Date().toISOString() },
  { id: "gym", label: "Go to gym", createdAt: new Date().toISOString() },
  { id: "read", label: "Read 10 pages", createdAt: new Date().toISOString() },
];

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

/** Is the given habit checked on the given date key? */
export function isChecked(habitId: string, dateKey: string): boolean {
  return state.checks[habitId]?.[dateKey] === true;
}

/** Toggle a habit's check state for a date key and persist. */
export async function toggleCheck(habitId: string, dateKey: string): Promise<void> {
  const dayMap = state.checks[habitId] ?? {};
  const next = !dayMap[dateKey];
  dayMap[dateKey] = next;
  state.checks[habitId] = dayMap;
  await store?.set(HABITS_KEY, state);
}

/** Add a new habit definition. */
export async function addHabit(label: string): Promise<void> {
  const id = `h-${Date.now().toString(36)}`;
  state.habits = [
    ...state.habits,
    { id, label: label.trim(), createdAt: new Date().toISOString() },
  ];
  await store?.set(HABITS_KEY, state);
}