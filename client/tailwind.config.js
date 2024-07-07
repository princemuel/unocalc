import defaultTheme from "tailwindcss/defaultTheme";
import twConfig from "./config.json";

/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts}"],
  corePlugins: {
    float: false,
    container: false,
  },
  future: {
    hoverOnlyWhenSupported: true,
  },
  theme: {
    extend: {
      fontFamily: {
        sans: [
          twConfig.theme.fonts.family.sans,
          ...defaultTheme.fontFamily.sans,
        ],
      },
      ...twConfig.theme.cursor,
    },
  },
  plugins: [],
};
