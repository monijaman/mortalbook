import { NextSeo } from 'next-seo';
import { useEffect, useState } from 'react';
import Layout from '../components/Layout';
import MemorialCard from '../components/MemorialCard';
import SearchBar from '../components/SearchBar';
import { memorialService } from '../lib/api/memorials';
import { Memorial } from '../lib/types';

export default function HomePage() {
    const [memorials, setMemorials] = useState<Memorial[]>([]);
    const [loading, setLoading] = useState(true);
    const [total, setTotal] = useState(0);

    useEffect(() => {
        const fetchMemorials = async () => {
            try {
                const data = await memorialService.getMemorials(0, 12);
                setMemorials(data.data || []);
                setTotal(data.total || 0);
            } catch (error) {
                console.error('Failed to fetch memorials:', error);
            } finally {
                setLoading(false);
            }
        };

        fetchMemorials();
    }, []);

    const handleSearch = async (query: string) => {
        setLoading(true);
        try {
            const data = await memorialService.searchMemorials(query);
            setMemorials(data.data || []);
            setTotal(data.total || 0);
        } catch (error) {
            console.error('Search failed:', error);
        } finally {
            setLoading(false);
        }
    };

    return (
        <Layout>
            <NextSeo
                title="Mortalbook - Memorial Site"
                description="A memorial site to honor those we've lost"
                openGraph={{
                    type: 'website',
                    url: 'https://mortalbook.com',
                    title: 'Mortalbook',
                    description: "A memorial site to honor those we've lost",
                    images: [
                        {
                            url: 'https://mortalbook.com/og-image.jpg',
                            width: 1200,
                            height: 630,
                            alt: 'Mortalbook',
                        },
                    ],
                }}
            />

            {/* Hero Section */}
            <section className="bg-gradient-to-r from-gray-800 to-gray-900 text-white py-20">
                <div className="container">
                    <h1 className="text-5xl font-bold mb-4">
                        Honoring Those We've Lost
                    </h1>
                    <p className="text-xl text-gray-300 mb-8">
                        A place to remember, celebrate, and share memories of loved ones
                    </p>
                    <SearchBar onSearch={handleSearch} />
                </div>
            </section>

            {/* Memorials Grid */}
            <section className="container py-16">
                <div className="mb-8">
                    <h2 className="text-3xl font-bold mb-2">Memorials</h2>
                    <p className="text-gray-600">
                        Showing {memorials.length} of {total} memorials
                    </p>
                </div>

                {loading ? (
                    <div className="flex justify-center items-center h-64">
                        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-gray-900"></div>
                    </div>
                ) : memorials.length > 0 ? (
                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        {memorials.map((memorial) => (
                            <MemorialCard key={memorial.id} memorial={memorial} />
                        ))}
                    </div>
                ) : (
                    <div className="text-center py-12">
                        <p className="text-gray-600 text-lg">
                            No memorials found. Try a different search.
                        </p>
                    </div>
                )}
            </section>
        </Layout>
    );
}
