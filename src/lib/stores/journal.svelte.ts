import { load, type Store } from "@tauri-apps/plugin-store";
import { format } from "date-fns";

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