import Layout from '../components/Layout';

export default function AboutPage() {
    return (
        <Layout>
            <div className="container py-16">
                <div className="max-w-2xl mx-auto">
                    <h1 className="text-4xl font-bold mb-8">About Mortalbook</h1>
                    <div className="prose prose-lg max-w-none">
                        <p>
                            Mortalbook is a digital memorial platform dedicated to honoring and celebrating
                            the lives of those who have passed away.
                        </p>
                        <h2>Our Mission</h2>
                        <p>
                            To provide a respectful, accessible space where families and friends can
                            share memories, celebrate lives, and keep the memory of loved ones alive.
                        </p>
                        <h2>Features</h2>
                        <ul>
                            <li>Create and manage memorial pages</li>
                            <li>Share photos and memories</li>
                            <li>Leave tributes and messages</li>
                            <li>Search and discover memorials</li>
                            <li>Preserve digital legacies</li>
                        </ul>
                    </div>
                </div>
            </div>
        </Layout>
    );
}
