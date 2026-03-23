/**
 * JsonLd — Server Component
 *
 * Injects a <script type="application/ld+json"> tag into the document <head>
 * via Next.js metadata infrastructure. Accepts any Schema.org graph object.
 *
 * Usage (Server Component):
 *   <JsonLd data={personSchema} />
 *   <JsonLd data={[personSchema, obituarySchema]} />
 */
export function JsonLd({ data }: { data: object | object[] }) {
    return (
        <script
            type="application/ld+json"
            // biome-ignore lint — innerHTML required for JSON-LD
            dangerouslySetInnerHTML={{
                __html: JSON.stringify(Array.isArray(data) ? data : data, null, 0),
            }}
        />
    );
}
