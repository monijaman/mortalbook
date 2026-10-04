<script>
  import { onMount } from 'svelte';
  import T from '$lib/T.svelte';
  import { lang, setLang, detectLang } from '$lib/i18n.js';

  // most-spoken / most-requested languages, shown first
  const popular = ['en', 'bn', 'hi', 'ur', 'ar', 'es', 'fr', 'de', 'pt', 'ru', 'zh', 'ja', 'ko', 'tr', 'id', 'it'];
  const fallback = [
    ['en', 'English'], ['bn', 'Bengali'], ['hi', 'Hindi'], ['ur', 'Urdu'], ['ar', 'Arabic'], ['es', 'Spanish'],
    ['fr', 'French'], ['de', 'German'], ['pt', 'Portuguese'], ['ru', 'Russian'], ['zh', 'Chinese'],
    ['ja', 'Japanese'], ['ko', 'Korean'], ['tr', 'Turkish'], ['id', 'Indonesian'], ['it', 'Italian']
  ];
  let languages = $state(fallback);

  const top = $derived(popular.map((c) => languages.find(([code]) => code === c)).filter(Boolean));
  const rest = $derived(
    languages.filter(([c]) => !popular.includes(c)).sort((a, b) => a[1].localeCompare(b[1]))
  );

  onMount(async () => {
    setLang(detectLang());
    try {
      const res = await fetch('/api/languages');
      if (res.ok) {
        const list = await res.json();
        if (list.length) languages = list.map((l) => [l.code, l.name]);
      }
    } catch {}
  });
</script>

<label class="picker">
  <span class="label"><span aria-hidden="true">🌐</span> <T text="Translate" /></span>
  <select value={$lang} onchange={(e) => setLang(e.currentTarget.value)} aria-label="Language">
    <optgroup label="Popular">
      {#each top as [code, name]}
        <option value={code}>{name}</option>
      {/each}
    </optgroup>
    {#if rest.length}
      <optgroup label="All languages">
        {#each rest as [code, name]}
          <option value={code}>{name}</option>
        {/each}
      </optgroup>
    {/if}
    {#if !languages.some(([c]) => c === $lang)}
      <option value={$lang}>{$lang}</option>
    {/if}
  </select>
</label>

<style>
  .picker { display: inline-flex; align-items: center; gap: 0.5rem; margin-left: auto; }
  .label { color: var(--muted); font-size: 0.95rem; white-space: nowrap; }
  select {
    background: var(--panel); color: var(--text); border: 1px solid var(--accent);
    border-radius: 6px; padding: 0.4rem 0.6rem; font: inherit; font-size: 0.95rem; cursor: pointer;
  }
  @media (max-width: 520px) { .label { display: none; } }
</style>
