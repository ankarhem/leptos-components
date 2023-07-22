module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {},
  },
  plugins: [require("./aria-plugin"), require("@tailwindcss/typography")],
};
