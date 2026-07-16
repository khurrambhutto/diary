<script lang="ts">
  import { fade, fly } from "svelte/transition";

  let {
    onRename,
    onDelete,
    onClose,
  }: {
    onRename: () => void;
    onDelete: () => void;
    onClose: () => void;
  } = $props();

  function handleRename() {
    onRename();
    onClose();
  }

  function handleDelete() {
    onDelete();
    onClose();
  }
</script>

<div class="backdrop" role="presentation" onclick={onClose}></div>

<div
  class="menu"
  role="menu"
  aria-label="Habit options"
  transition:fly={{ y: -4, duration: 120, opacity: 0 }}
>
  <button type="button" class="item" role="menuitem" onclick={handleRename}>
    Rename
  </button>
  <button type="button" class="item delete" role="menuitem" onclick={handleDelete}>
    Delete
  </button>
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    z-index: 40;
  }
  .menu {
    position: absolute;
    right: 0;
    top: 100%;
    margin-top: 0.25rem;
    z-index: 50;
    min-width: 7rem;
    background: var(--paper-raised);
    border: 1px solid var(--line);
    border-radius: 8px;
    box-shadow: var(--window-shadow);
    padding: 0.25rem;
    display: flex;
    flex-direction: column;
  }
  .item {
    background: none;
    border: none;
    padding: 0.375rem 0.625rem;
    cursor: pointer;
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 13px;
    font-weight: 400;
    color: var(--ink);
    text-align: left;
    border-radius: 4px;
    transition: background-color 100ms ease;
  }
  .item:hover {
    background-color: var(--paper-subtle);
  }
  .item.delete:hover {
    color: #ef4444;
  }
</style>
