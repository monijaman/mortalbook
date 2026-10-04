<script>
  // Renders `text` translated into the visitor's language.
  // `from`: language of the text ('en' for UI strings, a person's detected language, or 'auto').
  import { lang, dict, key, want } from '$lib/i18n.js';

  let { text, from = 'en' } = $props();

  $effect(() => {
    want(text, $lang, from);
  });

  const out = $derived($dict[key($lang, from, text)] ?? text);
</script>

{out}
