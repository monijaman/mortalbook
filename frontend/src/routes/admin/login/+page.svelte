<script>
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  let username = $state('');
  let password = $state('');
  let error = $state('');
  let busy = $state(false);

  onMount(async () => {
    const res = await fetch('/api/admin/me');
    if (res.ok) await goto('/admin');
  });

  async function submit(event) {
    event.preventDefault();
    error = '';
    busy = true;
    try {
      const res = await fetch('/api/admin/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ username, password })
      });
      const body = await res.json().catch(() => ({}));
      if (res.status === 401) throw new Error('Wrong username or password.');
      if (!res.ok) throw new Error(body.error || `Error ${res.status}`);
      password = '';
      await goto('/admin');
    } catch (err) {
      error = err.message;
    } finally {
      busy = false;
    }
  }
</script>

<svelte:head>
  <title>Admin sign in — Mortalbook</title>
  <meta name="robots" content="noindex,nofollow" />
</svelte:head>

<h1>Admin sign in</h1>

<form onsubmit={submit}>
  <label>
    <span>Username</span>
    <input bind:value={username} autocomplete="username" required />
  </label>
  <label>
    <span>Password</span>
    <input type="password" bind:value={password} autocomplete="current-password" required />
  </label>
  {#if error}<p class="error" role="alert">{error}</p>{/if}
  <button type="submit" disabled={busy}>{busy ? 'Signing in…' : 'Sign in'}</button>
</form>

<style>
  form { display: grid; gap: 1rem; max-width: 400px; margin-top: 1.5rem; }
  label span { display: block; margin-bottom: 0.3rem; color: var(--muted); }
  .error { color: var(--danger); margin: 0; }
  form > button { justify-self: start; }
</style>
