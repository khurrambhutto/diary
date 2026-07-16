import { todayKey } from "./journal.svelte";

let selected = $state<string>(todayKey());
let hydrated = $state(false);

export function initSelectedDate(): void {
  selected = todayKey();
  hydrated = true;
}

export function isCalendarHydrated(): boolean {
  return hydrated;
}

export function selectedKey(): string {
  return selected;
}

export function selectDate(key: string): void {
  selected = key;
}

export function selectToday(): void {
  selected = todayKey();
}

export function isToday(key: string): boolean {
  return key === todayKey();
}

export function isViewingToday(): boolean {
  return selected === todayKey();
}
