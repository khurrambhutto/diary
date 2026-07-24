<div class="settings-page">
  <h2 class="title">Settings</h2>

  <div class="about-card">
    <img src="/favicon.svg" alt="Diary App Icon" class="app-hero-icon" />
    <div class="about-details">
      <h3 class="app-name">Diary</h3>
      <p class="app-version">v0.1.2</p>
      <p class="app-desc">A minimal, cozy desktop app for daily journaling and habit tracking.</p>
    </div>
  </div>
</div>

<style>
  .settings-page {
    flex: 1 1 auto;
    min-height: 0;
    padding: 1.5rem 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }
  .title {
    font-family: "Newsreader", Georgia, serif;
    font-weight: 500;
    font-size: 28px;
    color: var(--ink);
    margin: 0;
  }
  .about-card {
    background: var(--paper-subtle);
    border-radius: 14px;
    padding: 1.25rem 1.5rem;
    display: flex;
    align-items: center;
    gap: 1.25rem;
    max-width: 480px;
  }
  .app-hero-icon {
    width: 64px;
    height: 64px;
    border-radius: 14px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
  }
  .about-details {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }
  .app-name {
    font-family: "Newsreader", Georgia, serif;
    font-size: 20px;
    font-weight: 500;
    color: var(--ink);
    margin: 0;
  }
  .app-version {
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 12px;
    font-weight: 700;
    color: var(--ink-soft);
    margin: 0 0 0.25rem;
  }
  .app-desc {
    font-family: "Atkinson Hyperlegible", system-ui, sans-serif;
    font-size: 13px;
    color: var(--ink-soft);
    margin: 0;
    line-height: 1.4;
  }
</style>
