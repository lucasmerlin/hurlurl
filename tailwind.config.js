/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./web/src/**/*.rs", "./web/**/*.{scss,css,html}",], theme: {
        container: {
            center: true, padding: {
                DEFAULT: '1rem', sm: '2rem', lg: '4rem', xl: '8rem', '2xl': '16rem',
            },
        },
    },

    daisyui: {
        themes: [{
            mytheme: {
                "primary": "#588E29",
                "secondary": "#a3e635",
                "accent": "#588e29",
                "neutral": "#412234",
                "base-100": "#f8ede1",
                "info": "#93E6FB",
                "success": "#80CED1",
                "warning": "#EFD8BD",
                "error": "#E58B8B",
            },
        },],
    },
    plugins: [require("daisyui"),
        require("@tailwindcss/typography")],
}
