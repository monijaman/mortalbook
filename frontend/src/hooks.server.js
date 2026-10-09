import { env } from '$env/dynamic/private';

const BACKEND = env.BACKEND_URL || 'http://localhost:8080';

export async function handle({ event, resolve }) {
  const { pathname, search } = event.url;
  if (pathname.startsWith('/api/') || pathname.startsWith('/uploads/')) {
    const headers = new Headers(event.request.headers);
    // fetch() sends the backend's own Host, so pass the public host separately; the backend's
    // same-origin check (Origin vs host) reads this header.
    headers.delete('host');
    headers.set('x-forwarded-host', event.url.host);
    headers.delete('connection');
    const hasBody = !['GET', 'HEAD'].includes(event.request.method);
    // Buffer small JSON bodies (streaming them can fail with "expected non-null body source");
    // keep streaming multipart uploads, which can be very large.
    const isJson = (headers.get('content-type') || '').startsWith('application/json');
    const body = !hasBody ? undefined : isJson ? await event.request.arrayBuffer() : event.request.body;
    if (isJson) headers.delete('content-length');
    return fetch(BACKEND + pathname + search, {
      method: event.request.method,
      headers,
      body,
      duplex: 'half'
    });
  }
  return resolve(event);
}
