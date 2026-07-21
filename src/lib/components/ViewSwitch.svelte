<script lang="ts">
  import { subDays, format } from "date-fns";
  import { currentView, setView } from "$lib/stores/view.svelte";
  import {
    selectedKey,
    selectDate,
    selectToday,
    isViewingToday,
  } from "$lib/stores/selected-date.svelte";

  const todayKey = format(new Date(), "yyyy-MM-dd");
  const yesterdayKey = format(subDays(new Date(), 1), "yyyy-MM-dd");

  type Segment = "yesterday" | "today" | "analytics";

  const activeIndex = $derived.by<Segment | null>(() => {
    if (currentView() === "monthly") return "analytics";
    if (isViewingToday()) return "today";
    if (selectedKey() === yesterdayKey) return "yesterday";
    return null;
  });

  function pick(seg: Segment) {
    if (seg === "analytics") {
      setView("monthly");
    } else {
      const key = seg === "yesterday" ? yesterdayKey : todayKey;
      selectDate(key);
      if (currentView() !== "today") setView("today");
    }
  }
</script>

<div class="switch" role="tablist" aria-label="Navigation">
  <span
    class="thumb"
    class:idx-0={activeIndex === "yesterday"}
    class:idx-1={activeIndex === "today"}
    class:idx-2={activeIndex === "analytics"}
    class:hidden={activeIndex === null}
    aria-hidden="true"
  ></span>
  <button
    type="button"
    role="tab"
    aria-selected={activeIndex === "yesterday"}
    class="slot"
    class:active={activeIndex === "yesterday"}
    onclick={() => pick("yesterday")}
  >
    Yesterday
  </button>
  <button
    type="button"
    role="tab"
    aria-selected={activeIndex === "today"}
    class="slot"
    class:active={activeIndex === "today"}
    onclick={() => pick("today")}
  >
    Today
  </button>
  <button
    type="button"
    role="tab"
    aria-selected={activeIndex === "analytics"}
    class="slot"
    class:active={activeIndex === "analytics"}
    onclick={() => pick("analytics")}
  >
    Analytics
  </button>
</div>

<style>
  .switch {
    position: relative;
    width: 14rem;
    height: 2rem;
    border-radius: 9999px;
    background: transparent;
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    overflow: hidden;
    box-shadow: inset 0 0 0 1px var(--ink-faint);
  }

  .thumb {
    position: absolute;
    top: 0;
    left: 0;
    width: 33.333%;
    height: 100%;
    border-radius: 9999px;
    background-color: var(--track);
    transform: translateX(0);
    transition: transform 220ms cubic-bezier(0.32, 0.72, 0, 1),
                opacity 220ms ease;
    will-change: transform;
    z-index: 0;
  }
  .thumb.idx-1 { transform: translateX(100%); }
  .thumb.idx-2 { transform: translateX(200%); }
  .thumb.hidden { opacity: 0; }

  .slot {
    position: relative;
    z-index: 1;
    appearance: none;
    border: none;
    background: transparent;
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 13px;
    font-weight: 400;
    color: var(--ink-soft);
    cursor: pointer;
    transition: color 150ms ease, font-weight 150ms ease;
  }
  .slot:hover:not(.active) {
    color: var(--ink);
  }
  .slot.active {
    color: var(--ink);
    font-weight: 700;
  }
  .slot:focus-visible {
    outline: none;
  }
</style>
