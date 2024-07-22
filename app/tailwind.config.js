import twDefaultTheme from "tailwindcss/defaultTheme";
import twPlugin from "tailwindcss/plugin";
import twConfig from "./tailwind-config.json";

/** @type {import('tailwindcss').Config} */
export default {
  darkMode: ["selector", '[data-theme="dark"]'],
  content: ["./index.html", "./src/**/*.{js,ts}"],
  future: { hoverOnlyWhenSupported: true },
  theme: {
    screens: {
      "3xs": "24em", // @media (min-width: 384px) { ... }
      "2xs": "30em", // @media (min-width: 480px) { ... }
      ...twDefaultTheme.screens,
    },
    extend: {
      colors: twConfig.theme.colors,
      fontFamily: {
        sans: [
          twConfig.theme.fontFamily.sans,
          ...twDefaultTheme.fontFamily.sans,
        ],
      },
      cursor: twConfig.theme.cursor,
      screens: {
        xs: "36em", // @media (min-width: 576px) { ... },
        sm: "40em", // @media (min-width: 640px) { ... }
        md: "48em", // @media (min-width: 768px) { ... }
        lg: "64em", // @media (min-width: 1024px) { ... }
        xl: "80em", // @media (min-width: 1280px) { ... }
        "2xl": "96em", // @media (min-width: 1536px) { ... }
        "3xl": "112.5em", // @media (min-width: 1800px) { ... }
      },
    },
  },
  plugins: [
    twPlugin(function ({ addVariant }) {
      addVariant("neon", '&:where([data-theme="neon"], [data-theme="neon"] *)');
      addVariant("hocus", ["&:hover", "&:focus"]);
    }),
  ],
};
