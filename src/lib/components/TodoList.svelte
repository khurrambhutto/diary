<script lang="ts">
  import { todoItems, addTodo, removeTodo } from "$lib/stores/todos.svelte";
  import TodoRow from "./TodoRow.svelte";

  let text = $state("");

  function handleSubmit() {
    if (!text.trim()) return;
    void addTodo(text);
    text = "";
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSubmit();
    }
  }

  const list = $derived(todoItems());
</script>

<div class="todo-section">
  <div class="header">
    <h3 class="title">To do</h3>
  </div>

  <div class="list">
    {#if list.length === 0}
      <p class="empty">Nothing for today</p>
    {:else}
      {#each list as item (item.id)}
        <TodoRow text={item.text} onComplete={() => void removeTodo(item.id)} />
      {/each}
    {/if}
  </div>

  <form class="add-form" onsubmit={handleSubmit}>
    <input
      type="text"
      class="add-input"
      placeholder="Add a task…"
      bind:value={text}
      onkeydown={handleKeydown}
    />
  </form>
</div>

<style>
  .todo-section {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .title {
    margin: 0;
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 14px;
    font-weight: 700;
    color: var(--ink);
    letter-spacing: 0.02em;
    text-transform: uppercase;
  }
  .list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  .empty {
    margin: 0;
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 13px;
    color: var(--ink-soft);
    font-style: italic;
  }
  .add-form {
    display: flex;
  }
  .add-input {
    flex: 1;
    border: none;
    border-bottom: 1px solid var(--line);
    padding: 0.375rem 0;
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 14px;
    color: var(--ink);
    background: transparent;
    outline: none;
    transition: border-color 150ms ease;
  }
  .add-input::placeholder {
    color: var(--ink-faint);
  }
  .add-input:focus {
    border-color: var(--accent);
  }
</style>
