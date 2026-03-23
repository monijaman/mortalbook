module.exports = {
    content: [
        './pages/**/*.{js,ts,jsx,tsx}',
        './app/**/*.{js,ts,jsx,tsx,mdx}',
        './components/**/*.{js,ts,jsx,tsx}',
    ],
    theme: {
        extend: {
            colors: {
                memorial: {
                    50: '#f9f7f4',
                    100: '#f3efe9',
                    900: '#2a2a2a',
                },
            },
        },
    },
    plugins: [],
};
