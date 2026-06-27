import { goto } from "$app/navigation";

/**
 * View store — owns which page is active: 'today' | 'monthly'.
 * The ViewSwitch pill reads `current` for its active side and calls
 * `setView` to navigate. The SvelteKit route mirrors the value, so a
 * refresh / deep-link lands on the right page.
 */

export type View = "today" | "monthly";

let current = $state<View>("today");

export function currentView(): View {
  return current;
}

/** Seed `current` from the URL on first load. Call once from the root layout. */
export function setViewFromPath(pathname: string): void {
  current = pathname.startsWith("/monthly") ? "monthly" : "today";
}

/** Switch to a view — updates the rune and navigates the route. */
export function setView(view: View): void {
  current = view;
  const href = view === "monthly" ? "/monthly" : "/";
  void goto(href);
}