import { env } from '$env/dynamic/private';

const BACKEND = env.BACKEND_URL || 'http://localhost:8080';

export async function handle({ event, resolve }) {
  const { pathname, search } = event.url;
  if (pathname.startsWith('/api/') || pathname.startsWith('/uploads/')) {
    const headers = new Headers(event.request.headers);
    // Keep the public host so the backend's same-origin check (Origin vs Host) passes.
    headers.set('host', event.url.host);
    headers.delete('connection');
    const hasBody = !['GET', 'HEAD'].includes(event.request.method);
    return fetch(BACKEND + pathname + search, {
      method: event.request.method,
      headers,
      body: hasBody ? event.request.body : undefined,
      duplex: 'half'
    });
  }
  return resolve(event);
}
