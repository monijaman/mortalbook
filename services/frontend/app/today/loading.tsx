/**
 * loading.tsx — Streaming Skeleton for /today
 *
 * Shown by Next.js Suspense while the Server Component fetches data.
 * Mirrors the real page layout (header + 6 card skeletons) to prevent
 * layout shift and reassure users something is happening.
 */
export default function DiedTodayLoading() {
    return (
        <main className="min-h-screen bg-memorial-50">
            {/* Header skeleton */}
            <div className="border-b border-gray-200 bg-white px-6 py-12 text-center shadow-sm">
                <div className="mx-auto mb-2 h-4 w-24 animate-pulse rounded-full bg-gray-200" />
                <div className="mx-auto h-9 w-72 animate-pulse rounded-lg bg-gray-200 sm:w-96" />
                <div className="mx-auto mt-3 h-4 w-40 animate-pulse rounded-full bg-gray-200" />
            </div>

            {/* Card grid skeleton */}
            <div className="mx-auto max-w-7xl px-4 py-12 sm:px-6 lg:px-8">
                <ul
                    className="grid gap-8 sm:grid-cols-2 lg:grid-cols-3"
                    aria-label="Loading memorials…"
                    role="list"
                >
                    {Array.from({ length: 6 }).map((_, i) => (
                        // eslint-disable-next-line react/no-array-index-key
                        <li key={i}>
                            <CardSkeleton />
                        </li>
                    ))}
                </ul>
            </div>
        </main>
    );
}

function CardSkeleton() {
    return (
        <div
            className="flex flex-col overflow-hidden rounded-2xl bg-white shadow-md
                 ring-1 ring-gray-100"
            aria-hidden="true"
        >
            {/* Portrait placeholder */}
            <div className="h-64 w-full animate-pulse bg-gray-200" />

            <div className="flex flex-1 flex-col gap-4 p-6">
                {/* Name */}
                <div className="h-6 w-3/4 animate-pulse rounded-md bg-gray-200" />
                {/* Life span */}
                <div className="h-4 w-1/3 animate-pulse rounded-md bg-gray-200" />
                {/* Biography lines */}
                <div className="space-y-2">
                    <div className="h-3 w-full animate-pulse rounded bg-gray-100" />
                    <div className="h-3 w-full animate-pulse rounded bg-gray-100" />
                    <div className="h-3 w-5/6 animate-pulse rounded bg-gray-100" />
                    <div className="h-3 w-4/6 animate-pulse rounded bg-gray-100" />
                </div>
                {/* Date badge */}
                <div className="h-3 w-28 animate-pulse rounded-full bg-gray-100" />
                {/* Video placeholder */}
                <div
                    className="w-full animate-pulse rounded-xl bg-gray-200"
                    style={{ aspectRatio: "16 / 9" }}
                />
            </div>
        </div>
    );
}
