// Web preview: the bar like a browser's. The address field goes to an address; back and
// forward walk the addresses visited through the bar; reload loads the address again.
// A spinner covers the frame while it loads.

const previewHistory = new WeakMap();

// As normalize_url on the server: https:// for a bare host, http:// for localhost,
// absolute and relative URLs kept, javascript: refused ("").
function normalizePreviewUrl(input) {
  const value = input.trim();
  if (!value || /^javascript:/i.test(value)) return "";
  if (/^[a-z][a-z\d+.-]*:\/\//i.test(value) || /^(about|data|blob):/i.test(value)) return value;
  if (/^[/.?#]/.test(value)) return value;
  if (/^(localhost|127\.0\.0\.1|\[::1\])(:\d+)?(\/|$)/i.test(value)) return `http://${value}`;
  return `https://${value}`;
}

function previewState(root) {
  let state = previewHistory.get(root);
  if (!state) {
    const url = root.dataset.url ?? "";
    state = { entries: url ? [url] : [], index: url ? 0 : -1 };
    previewHistory.set(root, state);
  }
  return state;
}

function previewParts(root) {
  const own = (selector) => [...root.querySelectorAll(selector)].filter((el) => el.closest("[data-web-preview]") === root);
  return {
    frame: own("[data-web-preview-body] > iframe")[0],
    loading: own("[data-web-preview-loading]")[0],
    empty: own("[data-web-preview-empty]")[0],
    body: own("[data-web-preview-body]")[0],
    address: own("[data-web-preview-address]")[0],
    open: own("[data-web-preview-open]"),
    actions: own("[data-web-preview-action]"),
  };
}

function setPreviewLoading(root, loading) {
  const { loading: cover, body } = previewParts(root);
  if (cover) cover.hidden = !loading;
  if (loading) body?.setAttribute("aria-busy", "true");
  else body?.removeAttribute("aria-busy");
}

// Shows the current address in the bar and loads it in the frame.
function showPreview(root) {
  const state = previewState(root);
  const url = state.entries[state.index] ?? "";
  const parts = previewParts(root);
  root.dataset.url = url;
  if (parts.address) parts.address.value = url;
  for (const link of parts.open) {
    link.hidden = !url;
    if (url) link.href = url;
    else link.removeAttribute("href");
  }
  for (const button of parts.actions) {
    const action = button.dataset.webPreviewAction;
    if (action === "back") button.disabled = state.index <= 0;
    else if (action === "forward") button.disabled = state.index >= state.entries.length - 1;
    else if (action === "reload") button.disabled = !url;
  }
  if (parts.frame && url) {
    parts.frame.hidden = false;
    if (parts.empty) parts.empty.hidden = true;
    parts.frame.removeAttribute("srcdoc");
    setPreviewLoading(root, true);
    // Setting src loads the address again even when it has not changed.
    parts.frame.src = url;
  }
  root.dispatchEvent(new CustomEvent("bitop:web-preview-navigate", { bubbles: true, detail: { url } }));
}

function navigatePreview(root, raw) {
  const url = normalizePreviewUrl(raw);
  if (!url) return false;
  const state = previewState(root);
  state.entries = [...state.entries.slice(0, state.index + 1), url];
  state.index = state.entries.length - 1;
  showPreview(root);
  return true;
}

document.addEventListener("submit", (event) => {
  const form = event.target;
  if (!(form instanceof HTMLFormElement) || !form.hasAttribute("data-web-preview-form")) return;
  event.preventDefault();
  const root = form.closest("[data-web-preview]");
  const address = form.querySelector("[data-web-preview-address]");
  if (root && address && !navigatePreview(root, address.value)) address.value = root.dataset.url ?? "";
});

document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-web-preview-action]");
  const root = button?.closest("[data-web-preview]");
  if (!root || button.disabled) return;
  const state = previewState(root);
  const action = button.dataset.webPreviewAction;
  if (action === "back" && state.index > 0) state.index -= 1;
  else if (action === "forward" && state.index < state.entries.length - 1) state.index += 1;
  else if (action !== "reload" || state.index < 0) return;
  showPreview(root);
});

// The whole address is selected when the field is focused, ready to be replaced.
document.addEventListener("focusin", (event) => {
  const address = event.target;
  if (address instanceof HTMLInputElement && address.hasAttribute("data-web-preview-address")) address.select();
});

// load does not bubble.
document.addEventListener(
  "load",
  (event) => {
    const frame = event.target;
    if (!(frame instanceof HTMLIFrameElement)) return;
    const root = frame.closest("[data-web-preview]");
    if (root && frame.parentElement?.hasAttribute("data-web-preview-body")) setPreviewLoading(root, false);
  },
  { capture: true },
);

// The back and forward buttons start disabled; the reload button follows the address.
document.querySelectorAll("[data-web-preview]").forEach((root) => {
  const state = previewState(root);
  for (const button of previewParts(root).actions) {
    const action = button.dataset.webPreviewAction;
    if (action === "back") button.disabled = state.index <= 0;
    else if (action === "forward") button.disabled = state.index >= state.entries.length - 1;
  }
});
