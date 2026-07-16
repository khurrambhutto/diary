<script lang="ts">
  import Check from "lucide-svelte/icons/check";
  import MoreHorizontal from "lucide-svelte/icons/more-horizontal";
  import HabitMenu from "./HabitMenu.svelte";

  let {
    label,
    checked,
    streak,
    onToggle,
    onDelete,
    onRename,
  }: {
    label: string;
    checked: boolean;
    streak: number;
    onToggle: () => void;
    onDelete: () => void;
    onRename: (label: string) => void;
  } = $props();

  let menuOpen = $state(false);
  let editing = $state(false);
  let draft = $state("");
  let inputEl: HTMLInputElement | undefined = $state();

  function openMenu() {
    menuOpen = true;
  }

  function closeMenu() {
    menuOpen = false;
  }

  function startRename() {
    draft = label;
    editing = true;
  }

  function commitRename() {
    const trimmed = draft.trim();
    if (trimmed && trimmed !== label) {
      onRename(trimmed);
    }
    editing = false;
  }

  function cancelRename() {
    editing = false;
  }

  function handleRenameKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      e.preventDefault();
      commitRename();
    } else if (e.key === "Escape") {
      e.preventDefault();
      cancelRename();
    }
  }

  $effect(() => {
    if (editing && inputEl) {
      inputEl.focus();
      inputEl.select();
    }
  });
</script>

<label class="row">
  <div class="box" class:accent={checked} aria-hidden="true">
    {#if checked}
      <Check size={14} color="#ffffff" strokeWidth={2.5} />
    {/if}
  </div>
  <input type="checkbox" checked={checked} onchange={onToggle} />
  {#if editing}
    <input
      type="text"
      class="edit-input"
      bind:value={draft}
      bind:this={inputEl}
      onblur={commitRename}
      onkeydown={handleRenameKeydown}
      aria-label="Rename habit"
    />
  {:else}
    <span class="label">{label}</span>
  {/if}
  {#if streak > 0}
    <span class="streak" title="Streak">{streak}&thinsp;🔥</span>
  {/if}
  <div class="menu-anchor">
    <button
      type="button"
      class="options-btn"
      aria-label="Habit options"
      onclick={openMenu}
    >
      <MoreHorizontal size={14} />
    </button>
    {#if menuOpen}
      <HabitMenu
        onRename={startRename}
        onDelete={onDelete}
        onClose={closeMenu}
      />
    {/if}
  </div>
</label>

<style>
  .row {
    display: flex;
    align-items: center;
    cursor: pointer;
    user-select: none;
    gap: 0;
    padding: 0.25rem 0.5rem;
    margin: 0 -0.5rem;
    border-radius: 6px;
    transition: background-color 120ms ease;
  }
  .row:hover {
    background-color: var(--paper-subtle);
  }
  input[type="checkbox"] {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
  }
  .box {
    flex-shrink: 0;
    width: 1.25rem;
    height: 1.25rem;
    border: 1px solid var(--ink-faint);
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    transform: scale(1);
    transition:
      background-color 150ms ease,
      border-color 150ms ease,
      transform 180ms cubic-bezier(0.34, 1.56, 0.64, 1);
  }
  .box.accent {
    background-color: var(--accent);
    border-color: var(--accent);
    transform: scale(1.12);
  }
  .label {
    margin-left: 0.625rem;
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 14px;
    font-weight: 400;
    color: var(--ink);
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .edit-input {
    margin-left: 0.625rem;
    flex: 1;
    border: none;
    border-bottom: 1px solid var(--line);
    padding: 0.125rem 0;
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 14px;
    font-weight: 400;
    color: var(--ink);
    background: transparent;
    outline: none;
    min-width: 0;
  }
  .edit-input:focus {
    border-color: var(--accent);
  }
  .streak {
    flex-shrink: 0;
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 11px;
    color: var(--ink-soft);
    margin-left: 0.25rem;
  }
  .menu-anchor {
    position: relative;
    flex-shrink: 0;
  }
  .options-btn {
    flex-shrink: 0;
    background: none;
    border: none;
    padding: 0.25rem;
    cursor: pointer;
    color: var(--ink-faint);
    opacity: 0;
    transform: translateX(4px);
    transition:
      opacity 150ms ease,
      transform 150ms cubic-bezier(0.34, 1.56, 0.64, 1),
      color 150ms ease;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
  }
  .row:hover .options-btn,
  .row:focus-within .options-btn {
    opacity: 1;
    transform: translateX(0);
  }
  .options-btn:hover {
    color: var(--accent);
    background-color: var(--paper-subtle);
  }
  .row:focus-within .box {
    outline: none;
    box-shadow: 0 0 0 2px var(--accent);
  }
</style>
