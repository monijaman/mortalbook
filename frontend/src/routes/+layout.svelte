<script>
  import { afterNavigate } from '$app/navigation';
  import { onMount } from 'svelte';
  import { get } from 'svelte/store';
  import '../app.css';
  import T from '$lib/T.svelte';
  import LanguagePicker from '$lib/LanguagePicker.svelte';
  import { selectedCountry, syncCountryUrl } from '$lib/country.js';

  let { children } = $props();
  let theme = $state('light');

  onMount(() => {
    const savedTheme = localStorage.getItem('theme');
    if (savedTheme === 'dark' || savedTheme === 'light') {
      theme = savedTheme;
      document.documentElement.dataset.theme = savedTheme;
    }
  });

  function toggleTheme() {
    theme = theme === 'light' ? 'dark' : 'light';
    document.documentElement.dataset.theme = theme;
    localStorage.setItem('theme', theme);
  }

  afterNavigate(({ to }) => {
    const urlCountry = to.url.searchParams.get('country');
    if (urlCountry !== null) selectedCountry.set(urlCountry);
    else syncCountryUrl(get(selectedCountry));
  });
</script>

<svelte:head>
  <meta name="description" content="Mortalbook is a place to remember people who have passed away. Discover their lives, stories, and anniversaries." />
  <meta name="application-name" content="Mortalbook" />
  <meta property="og:site_name" content="Mortalbook" />
  <meta property="og:type" content="website" />
  <meta property="og:image" content="https://mortalbook.com/og-image.svg" />
  <meta name="twitter:card" content="summary_large_image" />
  <meta name="twitter:image" content="https://mortalbook.com/og-image.svg" />
</svelte:head>

<header>
  <a class="brand" href="/">✦ Mortalbook</a>
  <nav>
    <a href="/"><T text="Today" /></a>
    <a href="/people"><T text="Remembered" /></a>
    <a href="/admin"><T text="Add a person" /></a>
  </nav>
  <div class="header-tools">
    <LanguagePicker />
    <button
      class="theme-toggle"
      type="button"
      aria-label={theme === 'light' ? 'Switch to dark theme' : 'Switch to light theme'}
      aria-pressed={theme === 'dark'}
      onclick={toggleTheme}
    >
      <span aria-hidden="true">{theme === 'light' ? '☾' : '☼'}</span>
      <T text={theme === 'light' ? 'Dark theme' : 'Light theme'} />
    </button>
  </div>
</header>

<main>{@render children()}</main>

<footer>
  <p class="muted"><T text="Gone from our sight, never from our hearts." /></p>
</footer>

<style>
  header {
    display: flex; flex-wrap: wrap; align-items: center; gap: 1rem 2rem; justify-content: space-between;
    padding: 0.9rem 1.5rem; border-bottom: 1px solid var(--line); background: var(--header-bg);
    position: sticky; top: 0; backdrop-filter: blur(6px); z-index: 10;
  }
  .brand { font-size: 1.5rem; color: var(--text); text-decoration: none; letter-spacing: 0.06em; }
  nav { display: flex; gap: 1.4rem; flex: 1; }
  nav a { color: var(--muted); text-decoration: none; }
  nav a:hover { color: var(--accent); }
  .header-tools { display: flex; align-items: center; gap: 0.75rem; }
  .theme-toggle {
    display: inline-flex; align-items: center; gap: 0.45rem; white-space: nowrap;
    padding: 0.35rem 0.7rem; font-size: 0.95rem;
  }
  main { max-width: 1600px; margin: 0 auto; padding: 2rem 1.5rem 4rem; }
  footer { text-align: center; padding: 2rem 1rem; border-top: 1px solid var(--line); font-style: italic; }
  @media (max-width: 640px) {
    header { gap: 0.8rem 1rem; }
    nav { order: 3; flex-basis: 100%; }
    .header-tools { margin-left: auto; }
  }
</style>
