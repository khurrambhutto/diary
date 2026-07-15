<script lang="ts">
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
  <svg
    class="box"
    class:accent={checked}
    viewBox="0 -960 960 960"
    width="1.25rem"
    height="1.25rem"
    fill="currentColor"
    aria-hidden="true"
  >
    {#if checked}
      <path
        d="m424-312 282-282-56-56-226 226-114-114-56 56 170 170Z M200-120q-33 0-56.5-23.5T120-200v-560q0-33 23.5-56.5T200-840h560q33 0 56.5 23.5T840-760v560q0 33-23.5 56.5T760-120H200Zm0-80h560v-560H200v560Zm0-560v560-560Z"
      />
    {:else}
      <path
        d="M200-120q-33 0-56.5-23.5T120-200v-560q0-33 23.5-56.5T200-840h560q33 0 56.5 23.5T840-760v560q0 33-23.5 56.5T760-120H200Zm0-80h560v-560H200v560Z"
      />
    {/if}
  </svg>
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
    color: var(--ink-faint);
    flex-shrink: 0;
    transition: color 150ms ease;
    border-radius: 4px;
  }
  .box.accent {
    color: var(--accent);
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
    transition: opacity 150ms ease, color 150ms ease;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .row:hover .delete {
    opacity: 1;
  }
  .delete:hover {
    color: #ef4444;
  }
  .row:focus-within .box {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }
</style>
