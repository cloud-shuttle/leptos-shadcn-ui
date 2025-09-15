// const {fontFamily} = require('tailwindcss/defaultTheme');

/** @type {import('tailwindcss').Config} */
module.exports = {
    darkMode: ['class'],
    content: [
        '*.html', 
        './src/**/*.rs', 
        '../../packages/leptos/*/src/*.rs', 
        '../../packages/leptos/*/src/**/*.rs',
        // Include all Tailwind classes we might use
        './src/**/*.rs'
    ],
    safelist: [
        // Static classes
        'min-h-screen', 'bg-white', 'text-gray-900', 'p-8', 'p-4', 'rounded-lg', 'space-y-4',
        'text-4xl', 'text-xl', 'font-bold', 'font-semibold', 'mb-8', 'mb-2',
        // Dynamic color classes
        'bg-blue-100', 'bg-green-100', 'bg-red-100', 'bg-gray-100',
        'text-blue-800', 'text-green-800', 'text-red-800', 'text-gray-700',
        'text-blue-700', 'text-green-700', 'text-red-700',
        'text-blue-600', 'text-green-600', 'text-red-600',
        // All color shades for dynamic generation
        'bg-blue-50', 'bg-blue-200', 'bg-blue-300', 'bg-blue-400', 'bg-blue-500', 'bg-blue-600', 'bg-blue-700', 'bg-blue-800', 'bg-blue-900',
        'bg-green-50', 'bg-green-200', 'bg-green-300', 'bg-green-400', 'bg-green-500', 'bg-green-600', 'bg-green-700', 'bg-green-800', 'bg-green-900',
        'bg-red-50', 'bg-red-200', 'bg-red-300', 'bg-red-400', 'bg-red-500', 'bg-red-600', 'bg-red-700', 'bg-red-800', 'bg-red-900',
        'bg-gray-50', 'bg-gray-200', 'bg-gray-300', 'bg-gray-400', 'bg-gray-500', 'bg-gray-600', 'bg-gray-700', 'bg-gray-800', 'bg-gray-900',
        'text-blue-50', 'text-blue-200', 'text-blue-300', 'text-blue-400', 'text-blue-500', 'text-blue-600', 'text-blue-700', 'text-blue-800', 'text-blue-900',
        'text-green-50', 'text-green-200', 'text-green-300', 'text-green-400', 'text-green-500', 'text-green-600', 'text-green-700', 'text-green-800', 'text-green-900',
        'text-red-50', 'text-red-200', 'text-red-300', 'text-red-400', 'text-red-500', 'text-red-600', 'text-red-700', 'text-red-800', 'text-red-900',
        'text-gray-50', 'text-gray-200', 'text-gray-300', 'text-gray-400', 'text-gray-500', 'text-gray-600', 'text-gray-700', 'text-gray-800', 'text-gray-900',
    ],
    theme: {
        container: {
            center: true,
            padding: '2rem',
            screens: {
                '2xl': '1400px'
            }
        },
        extend: {
            colors: {
                border: 'hsl(var(--border))',
                input: 'hsl(var(--input))',
                ring: 'hsl(var(--ring))',
                background: 'hsl(var(--background))',
                foreground: 'hsl(var(--foreground))',
                primary: {
                    DEFAULT: 'hsl(var(--primary))',
                    foreground: 'hsl(var(--primary-foreground))'
                },
                secondary: {
                    DEFAULT: 'hsl(var(--secondary))',
                    foreground: 'hsl(var(--secondary-foreground))'
                },
                destructive: {
                    DEFAULT: 'hsl(var(--destructive))',
                    foreground: 'hsl(var(--destructive-foreground))'
                },
                muted: {
                    DEFAULT: 'hsl(var(--muted))',
                    foreground: 'hsl(var(--muted-foreground))'
                },
                accent: {
                    DEFAULT: 'hsl(var(--accent))',
                    foreground: 'hsl(var(--accent-foreground))'
                },
                popover: {
                    DEFAULT: 'hsl(var(--popover))',
                    foreground: 'hsl(var(--popover-foreground))'
                },
                card: {
                    DEFAULT: 'hsl(var(--card))',
                    foreground: 'hsl(var(--card-foreground))'
                }
            },
            borderRadius: {
                lg: `var(--radius)`,
                md: `calc(var(--radius) - 2px)`,
                sm: 'calc(var(--radius) - 4px)'
            },
            fontFamily: {
                // sans: ['var(--font-sans)', ...fontFamily.sans]
            },
            keyframes: {
                'accordion-down': {
                    from: {height: '0'},
                    to: {height: 'var(--radix-accordion-content-height)'}
                },
                'accordion-up': {
                    from: {height: 'var(--radix-accordion-content-height)'},
                    to: {height: '0'}
                }
            },
            animation: {
                'accordion-down': 'accordion-down 0.2s ease-out',
                'accordion-up': 'accordion-up 0.2s ease-out'
            }
        }
    },
    plugins: [require('tailwindcss-animate')]
};
