/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "jit",
  content: [
    "./css/*.{js,ts,jsx,tsx,css,scss,html}",
    "./css/**/*.{js,ts,jsx,tsx}",
    "./src/**/*.{js,ts,jsx,tsx,rs,scss,css,html,rs}",
    "./index.html",
    "./src/main.rs",
  ],
  safelist: ["h-sm", "h-base", "h-lg", "h-xl", "h-2xl"],
  plugins: [require("daisyui"), require("@tailwindcss/typography")],
  theme: {
    fontFamily: {
      //   sans: ["Open Sans", "Noto Color Emoji"],
      //   display: ["Comfortaa", "Noto Color Emoji"],
      //   mono: ["Fira Mono", "Noto Color Emoji"],
    },
    fontSize: {
      sm: "0.875rem", // 14px
      base: "1rem", // 16px
      lg: "1.125rem", // 18px
      xl: "1.25rem", // 20px
      "2xl": "1.5rem", // 24px
    },
    extend: {
      // Define custom utilities for SVG height and width based on font size
      height: (theme) => ({
        ...theme("fontSize"),
      }),
      width: (theme) => ({
        ...theme("fontSize"),
      }),
    },
  },
  daisyui: {
    themes: [
      {
        green: {
          primary: "#15803d",
          "primary-content": "#F2FFF2", // CMYK 5-0-5-0
          secondary: "#22c55e",
          "secondary-content": "#000D00", // CMYK 100-0-100-95
          accent: "#14532d",
          "accent-content": "#F2FFF2", // CMYK 5-0-5-0
          neutral: "#193319", // CMYK 50-0-50-80
          "neutral-content": "#F2FFF2", // CMYK 5-0-5-0
          "base-100": "#F2FFF2", // CMYK 5-0-5-0
          "base-200": "#D9FFD9", // CMYK 15-0-15-0
          "base-300": "#CCFFCC", // CMYK 20-0-20-0
          "base-content": "#000D00", // CMYK 100-0-100-95
          info: "#4169e1",
          "info-content": "#d6e2fc",
          success: "#b9ff66",
          "success-content": "#0d1603",
          warning: "#e9d543",
          "warning-content": "#131001",
          error: "#ff4500",
          "error-content": "#160200",
        },
        "green-dark": {
          primary: "#15803d",
          "primary-content": "#F2FFF2",
          secondary: "#22c55e",
          "secondary-content": "#000D00",
          accent: "#14532d",
          "accent-content": "#F2FFF2",
          neutral: "#193319",
          "neutral-content": "#F2FFF2",
          "base-100": "#001900",
          "base-200": "#003300",
          "base-300": "#004D00",
          "base-content": "#F2FFF2",
          info: "#4169e1",
          "info-content": "#d6e2fc",
          success: "#b9ff66",
          "success-content": "#0d1603",
          warning: "#e9d543",
          "warning-content": "#131001",
          error: "#ff4500",
          "error-content": "#160200",
        },
        purple: {
          primary: "#6d28d9",
          "primary-content": "#F2F2FF", // CMYK 5-5-0-0
          secondary: "#8b5cf6",
          "secondary-content": "#00000D", // CMYK 100-100-0-95
          accent: "#4c1d95",
          "accent-content": "#F2F2FF", // CMYK 5-5-0-0
          neutral: "#191933", // CMYK 50-50-0-80
          "neutral-content": "#F2F2FF", // CMYK 5-5-0-0
          "base-100": "#F2F2FF", // CMYK 5-5-0-0
          "base-200": "#D9D9FF", // CMYK 15-15-0-0
          "base-300": "#CCCCFF", // CMYK 20-20-0-0
          "base-content": "00000D", // CMYK 100-100-0-95
          info: "#4169e1",
          "info-content": "#d6e2fc",
          success: "#b9ff66",
          "success-content": "#0d1603",
          warning: "#e9d543",
          "warning-content": "#131001",
          error: "#ff4500",
          "error-content": "#160200",
        },
        "purle-dark": {
          primary: "#6d28d9",
          "primary-content": "#F2F2FF",
          secondary: "#8b5cf6",
          "secondary-content": "#00000D",
          accent: "#4c1d95",
          "accent-content": "#F2F2FF",
          neutral: "#191933",
          "neutral-content": "#F2F2FF",
          "base-100": "#000019",
          "base-200": "#000033",
          "base-300": "#00004D",
          "base-content": "#F2F2FF",
          info: "#4169e1",
          "info-content": "#d6e2fc",
          success: "#b9ff66",
          "success-content": "#0d1603",
          warning: "#e9d543",
          "warning-content": "#131001",
          error: "#ff4500",
          "error-content": "#160200",
        },
        gold: {
          primary: "#a16207",
          "primary-content": "#FFFFF2", // CMYK 0-0-5-0
          secondary: "#eab308",
          "secondary-content": "#0D0D00", // CMYK 0-0-100-95
          accent: "#713f12",
          "accent-content": "#FFFFF2", // CMYK 0-0-5-0
          neutral: "#333319", // CMYK 0-0-50-80
          "neutral-content": "#FFFFF2", // CMYK 0-0-5-0
          "base-100": "#FFFFF2", // CMYK 0-0-5-0
          "base-200": "#FFFFD9", // CMYK 0-0-5-0
          "base-300": "#FFFFCC", // CMYK 0-0-5-0
          "base-content": "#0D0D00", // CMYK 0-0-100-95
          info: "#4169e1",
          "info-content": "#d6e2fc",
          success: "#b9ff66",
          "success-content": "#0d1603",
          warning: "#e9d543",
          "warning-content": "#131001",
          error: "#ff4500",
          "error-content": "#160200",
        },
        "gold-dark": {
          primary: "#a16207",
          "primary-content": "#FFFFF2",
          secondary: "#aeb308",
          "secondary-content": "#0D0D00",
          accent: "#713f12",
          "accent-content": "#FFFFF2",
          neutral: "#333319",
          "neutral-content": "#FFFFF2",
          "base-100": "#191900",
          "base-200": "#333300",
          "base-300": "#4D4D00",
          "base-content": "#FFFFF2",
          info: "#4169e1",
          "info-content": "#d6e2fc",
          success: "#b9ff66",
          "success-content": "#0d1603",
          warning: "#e9d543",
          "warning-content": "#131001",
          error: "#ff4500",
          "error-content": "#160200",
        },
        red: {
          primary: "#b91c1c",
          "primary-content": "#FFF2F2", // CMYK 0-5-5-0
          secondary: "#f87171",
          "secondary-content": "#0D0000", // CMYK 0-100-100-95
          accent: "#7f1d1d",
          "accent-content": "#FFF2F2", // CMYK 0-5-5-0
          neutral: "#331919", // CMYK 0-50-50-80
          "neutral-content": "#FFF2F2", // CMYK 0-5-5-0
          "base-100": "#FFF2F2", // CMYK 0-5-5-0
          "base-200": "#FFD9D9", // CMYK 0-15-15-0
          "base-300": "#FFCCCC", // CMYK 0-20-20-0
          "base-content": "#0D0000", // CMYK 0-100-100-95
          info: "#4169e1",
          "info-content": "#d6e2fc",
          success: "#b9ff66",
          "success-content": "#0d1603",
          warning: "#e9d543",
          "warning-content": "#131001",
          error: "#ff4500",
          "error-content": "#160200",
        },
        "red-dark": {
          primary: "#b91c1c",
          "primary-content": "#FFF2F2",
          secondary: "#f87171",
          "secondary-content": "#0D0000",
          accent: "#7f1d1d",
          "accent-content": "#FFF2F2",
          neutral: "#331919",
          "neutral-content": "#0D0000",
          "base-100": "#190000",
          "base-200": "#330000",
          "base-300": "#4D0000",
          "base-content": "#FFF2F2",
          info: "#4169e1",
          "info-content": "#d6e2fc",
          success: "#b9ff66",
          "success-content": "#0d1603",
          warning: "#e9d543",
          "warning-content": "#131001",
          error: "#ff4500",
          "error-content": "#160200",
        },
        black: {
          primary: "#404040", // CMYK 0-0-0-75
          "primary-content": "#FFFFFF",
          secondary: "#808080", // CMYK 0-0-0-50
          "secondary-content": "#000000",
          accent: "#000000", // CMYK 0-0-0-100
          "accent-content": "#FFFFFF",
          neutral: "#808080", // CMYK 0-0-0-50
          "neutral-content": "#FFFFFF",
          "base-100": "#CCCCCC", // CMYK 0-0-0-20
          "base-200": "#B3B3B3", // CMYK 0-0-0-30
          "base-300": "#999999", // CMYK 0-0-0-40
          "base-content": "#000000",
          info: "#4169e1",
          "info-content": "#d6e2fc",
          success: "#b9ff66",
          "success-content": "#0d1603",
          warning: "#e9d543",
          "warning-content": "#131001",
          error: "#ff4500",
          "error-content": "#160200",
        },
        "black-dark": {
          primary: "#404040",
          "primary-content": "#ffffff",
          secondary: "#808080",
          "secondary-content": "#FFFFFF",
          accent: "#000000",
          "accent-content": "#FFFFFF",
          neutral: "#808080",
          "neutral-content": "#FFFFFF",
          "base-100": "#191919",
          "base-200": "#333333",
          "base-300": "#4D4D4D",
          "base-content": "#FFFFFF",
          info: "#4169e1",
          "info-content": "#d6e2fc",
          success: "#b9ff66",
          "success-content": "#0d1603",
          warning: "#e9d543",
          "warning-content": "#131001",
          error: "#ff4500",
          "error-content": "#160200",
        },
        white: {
          primary: "#D9D9D9",
          "primary-content": "#000000",
          secondary: "#E6E6E6",
          "secondary-content": "#000000",
          accent: "#CCCCCC",
          "accent-content": "#000000",
          neutral: "#808080",
          "neutral-content": "#FFFFFF",
          "base-100": "#FFFFFF",
          "base-200": "#E6E6E6",
          "base-300": "#D9D9D9",
          "base-content": "#000000",
          info: "#4169e1",
          "info-content": "#d6e2fc",
          success: "#b9ff66",
          "success-content": "#0d1603",
          warning: "#e9d543",
          "warning-content": "#131001",
          error: "#ff4500",
          "error-content": "#160200",
        },
        "white-dark": {
          primary: "#D9D9D9",
          "primary-content": "#000000",
          secondary: "#E6E6E6",
          "secondary-content": "#000000",
          accent: "#CCCCCC",
          "accent-content": "#000000",
          neutral: "#808080",
          "neutral-content": "#FFFFFF",
          "base-100": "#191919",
          "base-200": "#333333",
          "base-300": "#4D4D4D",
          "base-content": "#FFFFFF",
          info: "#4169e1",
          "info-content": "#d6e2fc",
          success: "#b9ff66",
          "success-content": "#0d1603",
          warning: "#e9d543",
          "warning-content": "#131001",
          error: "#ff4500",
          "error-content": "#160200",
        },
      },
    ],
  },
};
