<script lang="ts">
  let {
    text,
    onComplete,
  }: {
    text: string;
    onComplete: () => void;
  } = $props();
</script>

<label class="row">
  <button
    type="button"
    class="circle"
    aria-label="Complete task"
    onclick={onComplete}
  >
    <svg viewBox="0 0 20 20" width="20" height="20" fill="currentColor">
      <circle cx="10" cy="10" r="8" stroke="currentColor" stroke-width="1.5" fill="none" />
    </svg>
  </button>
  <span class="text">{text}</span>
</label>

<style>
  .row {
    display: flex;
    align-items: center;
    cursor: default;
    user-select: none;
    gap: 0.625rem;
  }
  .circle {
    flex-shrink: 0;
    background: none;
    border: none;
    padding: 0;
    cursor: pointer;
    color: var(--ink-faint);
    transition: color 150ms ease;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .circle:hover {
    color: var(--accent);
  }
  .text {
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 14px;
    color: var(--ink);
    line-height: 1.4;
  }
</style>
