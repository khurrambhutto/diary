<script lang="ts">
  import Check from "lucide-svelte/icons/check";

  let {
    label,
    checked,
    streak,
    onToggle,
    onDelete,
  }: {
    label: string;
    checked: boolean;
    streak: number;
    onToggle: () => void;
    onDelete: () => void;
  } = $props();
</script>

<label class="row">
  <div class="box" class:accent={checked} aria-hidden="true">
    {#if checked}
      <Check size={14} color="#ffffff" strokeWidth={2.5} />
    {/if}
  </div>
  <input type="checkbox" checked={checked} onchange={onToggle} />
  <span class="label">{label}</span>
  {#if streak > 0}
    <span class="streak" title="Streak">{streak}&thinsp;🔥</span>
  {/if}
  <button type="button" class="delete" aria-label="Delete habit" onclick={onDelete}>
    <svg viewBox="0 0 16 16" width="12" height="12" fill="currentColor">
      <path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z" />
    </svg>
  </button>
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
  input {
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
  .streak {
    flex-shrink: 0;
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 11px;
    color: var(--ink-soft);
    margin-left: 0.25rem;
  }
  .delete {
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
  .row:hover .delete,
  .row:focus-within .delete {
    opacity: 1;
    transform: translateX(0);
  }
  .delete:hover {
    color: #ef4444;
    background-color: rgba(239, 68, 68, 0.08);
  }
  .row:focus-within .box {
    outline: none;
    box-shadow: 0 0 0 2px var(--accent);
  }
</style>
