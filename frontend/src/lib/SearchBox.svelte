<script>
  import T from '$lib/T.svelte';
  import { lang, dict, key, want } from '$lib/i18n.js';

  let { value = '', large = false } = $props();

  // placeholders are attributes, so translate them by hand
  const hint = 'Search by name or profession…';
  $effect(() => {
    want(hint, $lang, 'en');
  });
  const placeholder = $derived($dict[key($lang, 'en', hint)] ?? hint);
</script>

<form class="search" class:large method="GET" action="/people" role="search">
  <input type="search" name="q" {value} {placeholder} aria-label={placeholder} autocomplete="off" />
  <button type="submit"><T text="Search" /></button>
</form>

<style>
  .search { display: flex; gap: 0.6rem; max-width: 560px; }
  .search.large { max-width: 760px; margin: 0 auto; }
  .large input {
    font-size: 1.35rem; padding: 1rem 1.4rem; border-radius: 999px; border-color: var(--accent);
    background: rgba(23, 25, 29, 0.9); box-shadow: 0 0 0 4px rgba(185, 167, 121, 0.08);
  }
  .large button { font-size: 1.15rem; padding: 0 1.8rem; border-radius: 999px; }
  @media (max-width: 520px) {
    .large input { font-size: 1.1rem; padding: 0.8rem 1.1rem; }
    .large button { padding: 0 1.1rem; }
  }
</style>
