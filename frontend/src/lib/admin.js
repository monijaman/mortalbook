import { goto } from '$app/navigation';

/** Call an admin API endpoint with the session cookie; send the user to /admin/login on 401. */
export async function adminApi(path, options = {}) {
  const response = await fetch(path, {
    ...options,
    headers: {
      ...(options.body && !(options.body instanceof FormData)
        ? { 'Content-Type': 'application/json' }
        : {}),
      ...options.headers
    }
  });
  const body = await response.json().catch(() => ({}));
  if (!response.ok) {
    if (response.status === 401) {
      await goto('/admin/login');
      throw new Error('Please sign in.');
    }
    throw new Error(body.error || `Request failed (${response.status})`);
  }
  return body;
}

export async function logout() {
  await fetch('/api/admin/logout', { method: 'POST' }).catch(() => {});
  await goto('/admin/login');
}
