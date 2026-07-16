import { load, type Store } from "@tauri-apps/plugin-store";
import { format, startOfDay } from "date-fns";

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

export function isChecked(habitId: string, dateKey: string): boolean {
  return state.checks[habitId]?.[dateKey] === true;
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

/** Compute the current streak for a habit: consecutive days checked.
 *  Starts from yesterday and goes backwards; today is counted only if already checked.
 *  Breaks at the first missed day (max 365 days back).
 *  This way a user with a long streak sees it even before checking today. */
export function habitStreak(habitId: string): number {
  const checks = state.checks[habitId] ?? {};
  let streak = 0;
  const cursor = startOfDay(new Date());
  const today = format(cursor, "yyyy-MM-dd");

  // count today only if already checked
  if (checks[today] === true) {
    streak++;
  }
  // always start backwards from yesterday
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
