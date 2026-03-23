
export default function Footer() {
    const currentYear = new Date().getFullYear();

    return (
        <footer className="bg-gray-900 text-white mt-16">
            <div className="container py-12">
                <div className="grid grid-cols-3 gap-8 mb-8">
                    <div>
                        <h3 className="text-lg font-semibold mb-4">Mortalbook</h3>
                        <p className="text-gray-400">
                            A memorial site to honor those we've lost.
                        </p>
                    </div>
                    <div>
                        <h3 className="text-lg font-semibold mb-4">Quick Links</h3>
                        <ul className="space-y-2 text-gray-400">
                            <li><a href="/" className="hover:text-white">Home</a></li>
                            <li><a href="/memorials" className="hover:text-white">Memorials</a></li>
                            <li><a href="/privacy" className="hover:text-white">Privacy</a></li>
                        </ul>
                    </div>
                    <div>
                        <h3 className="text-lg font-semibold mb-4">Contact</h3>
                        <p className="text-gray-400">
                            Email: info@mortalbook.com
                        </p>
                    </div>
                </div>
                <div className="border-t border-gray-800 pt-8 text-center text-gray-400">
                    <p>&copy; {currentYear} Mortalbook. All rights reserved.</p>
                </div>
            </div>
        </footer>
    );
}
