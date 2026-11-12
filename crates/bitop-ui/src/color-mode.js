// Applies the colour mode before the page paints, so there is no flash of the wrong
// theme. Load it with a plain blocking <script> in <head>; bitop_ui::head() does.
//
// The mode is "light", "dark" or "system" and is kept in localStorage. A page that
// sets data-theme on <html> itself, for example from a cookie, is left alone until a
// script calls bitopUi.setColorMode().
(() => {
  const KEY = "bitop-color-mode";
  const root = document.documentElement;
  const system = matchMedia("(prefers-color-scheme: dark)");
  const fixed = root.hasAttribute("data-theme");

  const stored = () => {
    try {
      return localStorage.getItem(KEY) ?? "system";
    } catch {
      return "system";
    }
  };

  const apply = (mode) => {
    root.dataset.theme = mode === "system" ? (system.matches ? "dark" : "light") : mode;
    root.dataset.colorMode = mode;
    // For controls that show the mode, such as the colour mode toggle.
    document.dispatchEvent(new CustomEvent("bitop:color-mode", { detail: { mode, theme: root.dataset.theme } }));
  };

  let managed = !fixed;
  if (managed) apply(stored());

  system.addEventListener("change", () => {
    if (managed && stored() === "system") apply("system");
  });
  // Another tab changed the mode.
  addEventListener("storage", (event) => {
    if (managed && event.key === KEY) apply(stored());
  });

  window.bitopUi ??= {};
  window.bitopUi.colorMode = () => (managed ? stored() : root.dataset.theme);
  window.bitopUi.setColorMode = (mode) => {
    managed = true;
    try {
      if (mode === "system") localStorage.removeItem(KEY);
      else localStorage.setItem(KEY, mode);
    } catch {
      // Storage is blocked; the mode still applies to this page.
    }
    apply(mode);
  };
})();
