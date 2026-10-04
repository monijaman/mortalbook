<script>
  import { onMount } from 'svelte';
  import { lang, setLang, detectLang } from '$lib/i18n.js';

  const fallback = [
    ['en', 'English'], ['bn', 'Bengali'], ['hi', 'Hindi'], ['ar', 'Arabic'], ['es', 'Spanish'],
    ['fr', 'French'], ['de', 'German'], ['pt', 'Portuguese'], ['ru', 'Russian'], ['zh', 'Chinese'],
    ['ja', 'Japanese'], ['ko', 'Korean'], ['tr', 'Turkish'], ['ur', 'Urdu'], ['id', 'Indonesian']
  ];
  let languages = $state(fallback);

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
  <span aria-hidden="true">🌐</span>
  <select value={$lang} onchange={(e) => setLang(e.currentTarget.value)} aria-label="Language">
    {#each languages as [code, name]}
      <option value={code}>{name}</option>
    {/each}
    {#if !languages.some(([c]) => c === $lang)}
      <option value={$lang}>{$lang}</option>
    {/if}
  </select>
</label>

<style>
  .picker { display: inline-flex; align-items: center; gap: 0.4rem; }
  select {
    background: var(--panel); color: var(--text); border: 1px solid var(--line);
    border-radius: 6px; padding: 0.35rem 0.5rem; font: inherit; font-size: 0.9rem;
  }
</style>
