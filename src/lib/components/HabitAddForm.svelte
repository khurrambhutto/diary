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
  <div class="input-capsule">
    <input
      type="text"
      class="input"
      placeholder="New habit…"
      bind:value={text}
      onkeydown={handleKeydown}
    />
  </div>
</form>

<style>
  .form {
    margin-top: 0.25rem;
  }
  .input-capsule {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    background: var(--paper);
    border: 1px solid var(--paper-line);
    border-radius: 8px;
    padding: 0.25rem 0.75rem;
    transition: border-color 150ms ease, box-shadow 150ms ease;
  }
  .input-capsule:focus-within {
    border-color: var(--line-strong);
  }
  .input {
    flex: 1;
    border: none;
    padding: 0.125rem 0;
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 13px;
    color: var(--ink);
    background: transparent;
    outline: none;
    box-shadow: none;
    min-width: 0;
  }
  .input:focus,
  .input:focus-visible {
    outline: none;
    box-shadow: none;
  }
  .input::placeholder {
    color: var(--ink-soft);
    opacity: 0.65;
  }
</style>
