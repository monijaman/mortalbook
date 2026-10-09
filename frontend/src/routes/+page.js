export async function load({ fetch, url }) {
  const today = new Date();
  const params = new URLSearchParams({
    year: String(today.getFullYear()),
    month: String(today.getMonth() + 1),
    day: String(today.getDate())
  });
  const country = url.searchParams.get('country')?.trim();
  if (country) params.set('country', country);

  const [todayResponse, weekResponse, recentResponse] = await Promise.all([
    fetch(`/api/people/today?${params}`),
    fetch(`/api/people/week?${params}`),
    fetch(`/api/people/recent?${params}`)
  ]);
  let recentlyLost = [];
  let recentPeopleFailed = false;
  if (recentResponse.ok) {
    recentlyLost = await recentResponse.json();
  } else {
    console.error(`Recently lost people request failed: ${recentResponse.status}`);
    recentPeopleFailed = true;
  }

  if (!todayResponse.ok) {
    console.error(`Today's people request failed: ${todayResponse.status}`);
    return {
      people: [],
      upcoming: [],
      recent: [],
      recentlyLost,
      recentPeopleFailed,
      date: today.toISOString().slice(0, 10),
      failed: true
    };
  }

  const people = await todayResponse.json();
  let upcoming = [];
  let recent = [];
  if (weekResponse.ok) {
    ({ upcoming, recent } = await weekResponse.json());
  } else {
    console.error(`Weekly people request failed: ${weekResponse.status}`);
  }

  return {
    people,
    upcoming,
    recent,
    recentlyLost,
    recentPeopleFailed,
    date: today.toISOString().slice(0, 10),
    failed: false
  };
}
