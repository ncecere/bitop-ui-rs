// Button: loading turned on and off by a script, for a button whose action runs in the
// page, such as a fetch. The server renders the same state with `loading: true`.

window.bitopUi.button = {
  setLoading(button, loading) {
    button.toggleAttribute("data-loading", loading);
    button.disabled = loading;
    if (loading) button.setAttribute("aria-busy", "true");
    else button.removeAttribute("aria-busy");
  },
};
