<script lang="ts">
  /**
   * HabitRow — one habit checkbox + label. 1.25rem Material-style SVG glyph:
   *  unchecked = hollow rounded square in --ink-faint;
   *  checked    = hollow rounded square + check, in --accent.
   * The toggle stays accessible via the visually-hidden native input.
   */
  let {
    label,
    checked,
    onToggle,
  }: {
    label: string;
    checked: boolean;
    onToggle: () => void;
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
</label>

<style>
  .row {
    display: flex;
    align-items: center;
    cursor: pointer;
    user-select: none;
  }
  /* visually hidden native checkbox — the SVG glyph is the affordance */
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
  }
  .row:focus-within .box {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }
</style>