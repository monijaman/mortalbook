<script>
  import '../app.css';
  import T from '$lib/T.svelte';
  import LanguagePicker from '$lib/LanguagePicker.svelte';

  let { children } = $props();
</script>

<header>
  <a class="brand" href="/">✦ Mortalbook</a>
  <nav>
    <a href="/"><T text="Today" /></a>
    <a href="/people"><T text="Remembered" /></a>
    <a href="/admin"><T text="Add a person" /></a>
  </nav>
  <LanguagePicker />
</header>

<main>{@render children()}</main>

<footer>
  <p class="muted"><T text="Gone from our sight, never from our hearts." /></p>
</footer>

<style>
  header {
    display: flex; flex-wrap: wrap; align-items: center; gap: 1rem 2rem; justify-content: space-between;
    padding: 0.9rem 1.5rem; border-bottom: 1px solid var(--line); background: rgba(16, 17, 20, 0.85);
    position: sticky; top: 0; backdrop-filter: blur(6px); z-index: 10;
  }
  .brand { font-size: 1.5rem; color: var(--text); text-decoration: none; letter-spacing: 0.06em; }
  nav { display: flex; gap: 1.4rem; flex: 1; }
  nav a { color: var(--muted); text-decoration: none; }
  nav a:hover { color: var(--accent); }
  main { max-width: 1600px; margin: 0 auto; padding: 2rem 1.5rem 4rem; }
  footer { text-align: center; padding: 2rem 1rem; border-top: 1px solid var(--line); font-style: italic; }
</style>
