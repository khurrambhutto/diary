import { load, type Store } from "@tauri-apps/plugin-store";
import { differenceInCalendarDays, parseISO, startOfDay } from "date-fns";

const STORE_PATH = ".diary-store.json";
const TODOS_KEY = "todos";

export type TodoItem = { id: string; text: string; createdAt: string };

let store: Store | null = null;
let items = $state<TodoItem[]>([]);
let hydrated = $state(false);

function ageDaysOf(createdAt: string, now = new Date()): number {
  const created = startOfDay(parseISO(createdAt));
  return Math.max(0, differenceInCalendarDays(startOfDay(now), created));
}

/** Migrate legacy date-keyed lists into one open backlog. */
function normalize(raw: unknown): TodoItem[] {
  if (Array.isArray(raw)) {
    return (raw as TodoItem[]).filter(
      (t) => t && typeof t.id === "string" && typeof t.text === "string" && typeof t.createdAt === "string"
    );
  }
  if (raw && typeof raw === "object") {
    const byId = new Map<string, TodoItem>();
    for (const list of Object.values(raw as Record<string, TodoItem[]>)) {
      if (!Array.isArray(list)) continue;
      for (const t of list) {
        if (!t?.id || byId.has(t.id)) continue;
        byId.set(t.id, t);
      }
    }
    return [...byId.values()].sort((a, b) => a.createdAt.localeCompare(b.createdAt));
  }
  return [];
}

export async function initTodosStore(): Promise<void> {
  if (hydrated) return;
  store = await load(STORE_PATH);
  const raw = await store.get<unknown>(TODOS_KEY);
  items = normalize(raw);
  // Rewrite legacy shape once so later reads stay simple.
  if (raw && !Array.isArray(raw)) {
    await persist();
  }
  hydrated = true;
}

export function isTodosHydrated(): boolean {
  return hydrated;
}

export function todoItems(): TodoItem[] {
  return items;
}

/** Calendar days since the todo was created (0 = today). */
export function todoAgeDays(item: TodoItem, now = new Date()): number {
  return ageDaysOf(item.createdAt, now);
}

function capitalize(s: string): string {
  return s.charAt(0).toUpperCase() + s.slice(1);
}

export async function addTodo(text: string): Promise<void> {
  const trimmed = capitalize(text.trim());
  if (!trimmed) return;
  const todo: TodoItem = {
    id: `t-${Date.now().toString(36)}`,
    text: trimmed,
    createdAt: new Date().toISOString(),
  };
  items = [...items, todo];
  await persist();
}

export async function removeTodo(id: string): Promise<void> {
  items = items.filter((t) => t.id !== id);
  await persist();
}

async function persist(): Promise<void> {
  await store?.set(TODOS_KEY, items);
}
