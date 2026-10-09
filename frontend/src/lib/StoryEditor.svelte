<script>
  // Story editor: a roomy auto-growing textarea with a live preview that renders paragraphs
  // exactly like the public page (blocks separated by a blank line), counters and a tidy-up button.
  let { value = $bindable(''), max = 10000, rows = 14, label = 'Story' } = $props();

  let area;
  let showPreview = $state(true);

  const paragraphs = $derived(value.split(/\n{2,}/).map((s) => s.trim()).filter(Boolean));
  const chars = $derived(value.length);
  const words = $derived(value.trim() ? value.trim().split(/\s+/).length : 0);
  const readMinutes = $derived(Math.max(1, Math.round(words / 200)));
  const over = $derived(chars > max);

  function grow() {
    if (!area) return;
    area.style.height = 'auto';
    area.style.height = `${Math.max(area.scrollHeight, rows * 24)}px`;
  }

  $effect(() => {
    value;
    grow();
  });

  /** Normalize spacing: trim lines, join hard-wrapped lines, one blank line between paragraphs. */
  function tidy() {
    value = value
      .replace(/\r\n?/g, '\n')
      .split(/\n{2,}/)
      .map((block) =>
        block
          .split('\n')
          .map((line) => line.trim())
          .filter(Boolean)
          .join(' ')
          .replace(/[ \t]{2,}/g, ' ')
      )
      .filter(Boolean)
      .join('\n\n');
  }

  function curlyQuotes() {
    value = value
      .replace(/(^|[\s([{—-])"/g, '$1“')
      .replace(/"/g, '”')
      .replace(/(^|[\s([{—-])'/g, '$1‘')
      .replace(/'/g, '’');
  }

  /** Insert text at the cursor (used by the toolbar). */
  function insert(text) {
    const start = area.selectionStart;
    const end = area.selectionEnd;
    value = value.slice(0, start) + text + value.slice(end);
    queueMicrotask(() => {
      area.focus();
      area.selectionStart = area.selectionEnd = start + text.length;
    });
  }
</script>

<div class="story">
  <div class="top">
    <span class="label">{label}</span>
    <div class="tools">
      <button type="button" class="tool" onclick={tidy} title="Fix spacing: one blank line between paragraphs, no stray line breaks">Tidy spacing</button>
      <button type="button" class="tool" onclick={curlyQuotes} title="Convert straight quotes to typographic quotes">Smart quotes</button>
      <button type="button" class="tool" onclick={() => insert('—')} title="Insert an em dash">—</button>
      <button type="button" class="tool" onclick={() => (showPreview = !showPreview)} aria-pressed={showPreview}>
        {showPreview ? 'Hide preview' : 'Show preview'}
      </button>
    </div>
  </div>

  <div class="panes" class:single={!showPreview}>
    <textarea
      bind:this={area}
      bind:value
      oninput={grow}
      spellcheck="true"
      aria-label={label}
      placeholder="Write the story. Leave a blank line between paragraphs."
    ></textarea>

    {#if showPreview}
      <div class="preview" aria-label="Preview">
        {#each paragraphs as para}
          <p>{para}</p>
        {:else}
          <p class="empty">The preview appears here as you write.</p>
        {/each}
      </div>
    {/if}
  </div>

  <div class="stats" class:over>
    {words} words · {paragraphs.length} {paragraphs.length === 1 ? 'paragraph' : 'paragraphs'} ·
    ~{readMinutes} min read · {chars.toLocaleString()} / {max.toLocaleString()} characters
    {#if over}— too long, shorten it before saving{/if}
  </div>
</div>

<style>
  .story { display: grid; gap: 0.5rem; }
  .top { display: flex; flex-wrap: wrap; gap: 0.5rem; align-items: center; justify-content: space-between; }
  .label { color: var(--muted); font-size: 0.95rem; }
  .tools { display: flex; flex-wrap: wrap; gap: 0.4rem; }
  .tool { padding: 0.25rem 0.6rem; font-size: 0.85rem; background: transparent; color: var(--text); border: 1px solid var(--line); }
  .panes { display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; align-items: start; }
  .panes.single { grid-template-columns: 1fr; }
  textarea {
    width: 100%; box-sizing: border-box; resize: vertical; min-height: 20rem;
    font: inherit; font-size: 1.05rem; line-height: 1.6; overflow: hidden;
  }
  .preview {
    border: 1px solid var(--line); border-radius: 8px; padding: 0.75rem 1rem;
    font-size: 1.05rem; line-height: 1.6; background: var(--panel); min-height: 20rem; box-sizing: border-box;
  }
  .preview p { margin: 0 0 1em; }
  .empty { color: var(--muted); font-style: italic; }
  .stats { color: var(--muted); font-size: 0.85rem; }
  .stats.over { color: var(--danger); }
  @media (max-width: 800px) { .panes { grid-template-columns: 1fr; } }
</style>
