(function () {
  const themes = ["light", "dark", "neon", "auto"] as const;

  type Theme = (typeof themes)[number];

  /** Key in `localStorage` to store color theme preference at. */
  const storageKey = "uno-theme";

  /** Get a typesafe theme string from any JS value (unknown values are coerced to `'auto'`). */
  const parseTheme = (theme: unknown): Theme =>
    theme === "neon" || theme === "dark" || theme === "light" ? theme : "auto";

  /** Load the user’s preference from `localStorage`. */
  const loadTheme = (): Theme =>
    parseTheme(
      typeof localStorage !== "undefined" && localStorage.getItem(storageKey)
    );

  /** Store the user’s preference in `localStorage`. */
  function storeTheme(theme: Theme): void {
    if (typeof localStorage !== "undefined") {
      localStorage.setItem(
        storageKey,
        theme === "light" || theme === "dark" || theme === "neon"
          ? theme
          : "dark"
      );
    }
  }

  const mql = matchMedia("(prefers-color-scheme: light)");

  /** Get the preferred system color scheme. */
  const getPreferredColorScheme = (): Theme => (mql.matches ? "light" : "dark");

  /** Update theme widget, document theme, and local storage state. */
  function onThemeChange(theme: Theme): void {
    window.ThemeProvider.updateWidget(theme);
    document.documentElement.dataset.theme =
      theme === "auto" ? getPreferredColorScheme() : theme;
    storeTheme(theme);
  }

  // React to changes in system color scheme.
  mql.addEventListener("change", () => {
    if (loadTheme() === "auto") onThemeChange("auto");
  });

  class ThemeWidget extends HTMLFieldSetElement {
    constructor() {
      super();
      onThemeChange(loadTheme());
      this.addEventListener("change", (e) => {
        if (e.target instanceof HTMLInputElement && e.target.type === "radio") {
          onThemeChange(parseTheme(e.target.value));
        }
      });
    }
  }

  if (!customElements.get("theme-widget")) {
    customElements.define("theme-widget", ThemeWidget, { extends: "fieldset" });
  }
})();
