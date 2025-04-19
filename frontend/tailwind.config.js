/** @type {import('tailwindcss').Config} */
module.exports = {
    darkMode: 'class',
    content: [
        "./index.html",
        "./src/**/*.{rs,html}",
    ],
    theme: {
        extend: {
            fontFamily: {
                sans: ['Poppins', 'ui-sans-serif', 'system-ui'],
            },
            colors: {
                primary: {
                    light: '#dbeafe', // indigo-100
                    DEFAULT: '#6366f1', // indigo-500
                    dark: '#4338ca', // indigo-700
                },
            },
        },
    },
    plugins: [],
};
