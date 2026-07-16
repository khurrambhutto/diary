import { load, type Store } from "@tauri-apps/plugin-store";
import { format } from "date-fns";

const STORE_PATH = ".diary-store.json";
const TODOS_KEY = "todos";

export type TodoItem = { id: string; text: string; createdAt: string };

let store: Store | null = null;
let items = $state<TodoItem[]>([]);
let hydrated = $state(false);

function dateKey(date: Date = new Date()): string {
  return format(date, "yyyy-MM-dd");
}

export async function initTodosStore(): Promise<void> {
  if (hydrated) return;
  store = await load(STORE_PATH);
  const all = await store.get<Record<string, TodoItem[]>>(TODOS_KEY);
  const today = all?.[dateKey()];
  items = today ?? [];
  hydrated = true;
}

export function isTodosHydrated(): boolean {
  return hydrated;
}

export function todoItems(): TodoItem[] {
  return items;
}

export async function addTodo(text: string): Promise<void> {
  const trimmed = text.trim();
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
  const all = await store?.get<Record<string, TodoItem[]>>(TODOS_KEY) ?? {};
  all[dateKey()] = items;
  await store?.set(TODOS_KEY, all);
}
