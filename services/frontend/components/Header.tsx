import Link from 'next/link';

export default function Header() {
    return (
        <header className="bg-white shadow-sm sticky top-0 z-50">
            <div className="container flex items-center justify-between h-16">
                <Link href="/" className="text-2xl font-bold text-gray-900">
                    🕊️ Mortalbook
                </Link>
                <nav className="flex items-center gap-6">
                    <Link href="/" className="text-gray-600 hover:text-gray-900">
                        Home
                    </Link>
                    <Link href="/memorials" className="text-gray-600 hover:text-gray-900">
                        Memorials
                    </Link>
                    <Link href="/about" className="text-gray-600 hover:text-gray-900">
                        About
                    </Link>
                </nav>
            </div>
        </header>
    );
}
