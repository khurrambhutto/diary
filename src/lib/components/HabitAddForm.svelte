<script lang="ts">
  import { addHabit } from "$lib/stores/habits.svelte";

  let text = $state("");

  function handleSubmit() {
    if (!text.trim()) return;
    void addHabit(text);
    text = "";
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSubmit();
    }
  }
</script>

<form class="form" onsubmit={handleSubmit}>
  <input
    type="text"
    class="input"
    placeholder="New habit…"
    bind:value={text}
    onkeydown={handleKeydown}
  />
  <button type="submit" class="btn" aria-label="Add habit" disabled={!text.trim()}>
    <svg viewBox="0 0 20 20" width="16" height="16" fill="currentColor">
      <path d="M10 3a1 1 0 0 1 1 1v5h5a1 1 0 1 1 0 2h-5v5a1 1 0 1 1-2 0v-5H4a1 1 0 1 1 0-2h5V4a1 1 0 0 1 1-1z" />
    </svg>
  </button>
</form>

<style>
  .form {
    display: flex;
    gap: 0.375rem;
    align-items: center;
  }
  .input {
    flex: 1;
    border: none;
    border-bottom: 1px solid var(--line);
    padding: 0.375rem 0;
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 13px;
    color: var(--ink);
    background: transparent;
    outline: none;
    transition: border-color 150ms ease;
  }
  .input::placeholder {
    color: var(--ink-faint);
  }
  .input:focus {
    border-color: var(--accent);
  }
  .btn {
    flex-shrink: 0;
    background: none;
    border: none;
    padding: 0.25rem;
    cursor: pointer;
    color: var(--ink-soft);
    transition: color 150ms ease;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .btn:hover:not(:disabled) {
    color: var(--accent);
  }
  .btn:disabled {
    opacity: 0.3;
    cursor: default;
  }
</style>
