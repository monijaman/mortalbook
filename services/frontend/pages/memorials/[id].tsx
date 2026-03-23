import { NextSeo } from 'next-seo';
import { useRouter } from 'next/router';
import { useEffect, useState } from 'react';
import Layout from '../../components/Layout';
import { memorialService } from '../../lib/api/memorials';
import { Memorial, MemorialMedia } from '../../lib/types';

export default function MemorialDetailPage() {
    const router = useRouter();
    const { id } = router.query;
    const [memorial, setMemorial] = useState<Memorial | null>(null);
    const [media, setMedia] = useState<MemorialMedia[]>([]);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        if (!id) return;

        const fetchMemorial = async () => {
            try {
                const data = await memorialService.getMemorialById(id as string);
                setMemorial(data);

                const mediaData = await memorialService.getMemorialMedia(id as string);
                setMedia(mediaData || []);
            } catch (error) {
                console.error('Failed to fetch memorial:', error);
            } finally {
                setLoading(false);
            }
        };

        fetchMemorial();
    }, [id]);

    if (loading) {
        return (
            <Layout>
                <div className="flex justify-center items-center h-screen">
                    <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-gray-900"></div>
                </div>
            </Layout>
        );
    }

    if (!memorial) {
        return (
            <Layout>
                <div className="container py-16 text-center">
                    <h1 className="text-2xl font-bold text-gray-900">Memorial not found</h1>
                </div>
            </Layout>
        );
    }

    return (
        <Layout>
            <NextSeo
                title={memorial.name}
                description={memorial.biography}
                openGraph={{
                    type: 'website',
                    url: `https://mortalbook.com/memorials/${memorial.id}`,
                    title: memorial.name,
                    description: memorial.biography,
                }}
            />

            <article className="container py-12">
                <div className="max-w-3xl mx-auto">
                    {/* Header */}
                    <header className="mb-8">
                        <h1 className="text-4xl font-bold text-gray-900 mb-2">
                            {memorial.name}
                        </h1>
                        <p className="text-xl text-gray-600">
                            {new Date(memorial.date_of_birth).getFullYear()} -{' '}
                            {new Date(memorial.date_of_death).getFullYear()}
                        </p>
                    </header>

                    {/* Biography */}
                    <section className="bg-white rounded-lg shadow-md p-8 mb-8">
                        <h2 className="text-2xl font-semibold mb-4">Biography</h2>
                        <p className="text-gray-700 text-lg leading-relaxed whitespace-pre-wrap">
                            {memorial.biography}
                        </p>
                    </section>

                    {/* Media Gallery */}
                    {media.length > 0 && (
                        <section className="mb-8">
                            <h2 className="text-2xl font-semibold mb-4">Photos & Media</h2>
                            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                                {media.map((m) => (
                                    <div key={m.id} className="bg-white rounded-lg shadow-md overflow-hidden">
                                        <div className="bg-gray-300 h-64 flex items-center justify-center">
                                            <span className="text-4xl">📷</span>
                                        </div>
                                        <div className="p-4">
                                            <p className="text-sm text-gray-600">{m.description}</p>
                                        </div>
                                    </div>
                                ))}
                            </div>
                        </section>
                    )}
                </div>
            </article>
        </Layout>
    );
}
