<script lang="ts">
  let {
    value,
    onCommit,
    onCancel,
  }: {
    value: string;
    onCommit: (val: string) => void;
    onCancel: () => void;
  } = $props();

  let draft = $state("");
  let ready = $state(false);
  let inputEl: HTMLInputElement | undefined = $state();

  $effect(() => {
    if (!ready) {
      draft = value;
      ready = true;
    }
  });

  function commit() {
    onCommit(draft);
  }

  function cancel() {
    onCancel();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      e.preventDefault();
      commit();
    } else if (e.key === "Escape") {
      e.preventDefault();
      cancel();
    }
  }

  $effect(() => {
    if (inputEl) {
      inputEl.focus();
      inputEl.select();
    }
  });
</script>

<input
  type="text"
  class="edit-input"
  bind:value={draft}
  bind:this={inputEl}
  onblur={commit}
  onkeydown={handleKeydown}
  aria-label="Rename habit"
/>

<style>
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
</style>
