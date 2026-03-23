import type { Metadata, Viewport } from "next";
import { Inter } from "next/font/google";
import "../styles/globals.css";

const inter = Inter({
    subsets: ["latin"],
    display: "swap",
    variable: "--font-inter",
});

// ─── Site-wide default metadata ───────────────────────────────────────────────
export const metadata: Metadata = {
    metadataBase: new URL(
        process.env.NEXT_PUBLIC_SITE_URL ?? "https://mortalbook.com"
    ),
    title: {
        default: "Mortalbook — In Memoriam",
        template: "%s | Mortalbook",
    },
    description:
        "A memorial site dedicated to honouring the lives of those who have passed.",
    openGraph: {
        type: "website",
        siteName: "Mortalbook",
        locale: "en_US",
        images: [
            {
                url: "/og-image.jpg",
                width: 1200,
                height: 630,
                alt: "Mortalbook — In Memoriam",
            },
        ],
    },
    twitter: {
        card: "summary_large_image",
        site: "@mortalbook",
    },
    robots: {
        index: true,
        follow: true,
        googleBot: { index: true, follow: true },
    },
};

export const viewport: Viewport = {
    themeColor: "#2a2a2a",
    width: "device-width",
    initialScale: 1,
};

export default function RootLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <html lang="en" className={inter.variable}>
            <body className="bg-memorial-50 text-gray-900 antialiased">
                {children}
            </body>
        </html>
    );
}
