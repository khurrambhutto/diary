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
    <svg viewBox="0 0 20 20" width="20" height="20" class="ring">
      <circle cx="10" cy="10" r="8" stroke="currentColor" stroke-width="1.5" fill="none" />
    </svg>
    <svg viewBox="0 0 20 20" width="20" height="20" class="check">
      <path d="M6 10l3 3 5-5" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round" />
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
    position: relative;
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
    width: 20px;
    height: 20px;
  }
  .circle:hover {
    color: var(--accent);
  }
  .ring, .check {
    position: absolute;
    top: 0;
    left: 0;
  }
  .check {
    opacity: 0;
    transform: scale(0.5);
    transition: opacity 120ms ease, transform 120ms ease;
  }
  .circle:hover .check {
    opacity: 1;
    transform: scale(1);
  }
  .text {
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 14px;
    color: var(--ink);
    line-height: 1.4;
  }
</style>
