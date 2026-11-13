// Controls for the gallery shell. Not part of the library.

const root = document.documentElement;

// Mark the current page in the sidebar. Paths are compared without a trailing
// ".html", so this also works in the static copy, where /c/button is c/button.html.
const page = (url) => new URL(url, location.href).pathname.replace(/\.html$/, "");
// Only the sidebar's links: a page's own navigation (breadcrumbs, pagination) links to
// the same path with a query and marks its current item itself.
for (const link of document.querySelectorAll('nav[aria-label="Components"] a[href]')) {
  if (page(link.href) === page(location.href)) link.setAttribute("aria-current", "page");
}

const theme = document.querySelector('[data-gallery-set="theme"]');
if (theme) theme.value = window.bitopUi.colorMode();

document.addEventListener("change", (event) => {
  const control = event.target;
  if (!(control instanceof HTMLSelectElement)) return;
  if (control.dataset.gallerySet === "dir") root.dir = control.value;
  if (control.dataset.gallerySet === "theme") window.bitopUi.setColorMode(control.value);
});

// Buttons that raise a toast: data-gallery-toast is the tone, with
// data-gallery-title, data-gallery-description and data-gallery-action (an action's
// label, which raises a success toast titled data-gallery-action-toast).
document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-gallery-toast]");
  if (!button) return;
  const { galleryToast: tone, galleryTitle: title, galleryDescription: description = "" } = button.dataset;
  const { galleryAction: label, galleryActionToast: done } = button.dataset;
  const action = label ? { label, onClick: () => window.bitopUi.toast.success(done) } : undefined;
  window.bitopUi.toast.add({ title, description, tone, action });
});

// The alert dialog on the dialog page: its action runs in the page for 900ms, then
// fails with the message in data-gallery-fail. Opening it again clears the error.
document.addEventListener("click", (event) => {
  const target = event.target instanceof Element ? event.target : null;
  const reset = target?.closest("[data-gallery-reset-dialog]");
  if (reset) window.bitopUi.dialog.setError(document.getElementById(reset.dataset.galleryResetDialog), "");
  const confirm = target?.closest("[data-gallery-fail]");
  if (!confirm) return;
  const dialog = confirm.closest("dialog");
  window.bitopUi.dialog.setError(dialog, "");
  window.bitopUi.dialog.setBusy(dialog, true);
  setTimeout(() => {
    window.bitopUi.dialog.setBusy(dialog, false);
    window.bitopUi.dialog.setError(dialog, confirm.dataset.galleryFail);
  }, 900);
});

// The goal stepper on the drawer page: data-gallery-step adds to the number in the
// element data-gallery-target names, between its data-min and data-max.
document.addEventListener("click", (event) => {
  const target = event.target instanceof Element ? event.target : null;
  const step = target?.closest("[data-gallery-step]");
  if (step) {
    const value = document.getElementById(step.dataset.galleryTarget);
    const next = Number(value.textContent) + Number(step.dataset.galleryStep);
    value.textContent = String(next);
    for (const button of document.querySelectorAll(`[data-gallery-target="${value.id}"]`)) {
      const delta = Number(button.dataset.galleryStep);
      button.disabled = delta < 0 ? next + delta < Number(value.dataset.min) : next + delta > Number(value.dataset.max);
    }
  }
  const save = target?.closest("[data-gallery-goal]");
  if (save) {
    const goal = document.getElementById(save.dataset.galleryGoal).textContent;
    window.bitopUi.toast.success(`Goal set to ${goal} calories`);
  }
});

// A form that raises a toast when it is submitted.
document.addEventListener("submit", (event) => {
  const title = event.target instanceof HTMLFormElement && event.target.dataset.gallerySubmitToast;
  if (title) window.bitopUi.toast.success(title);
});

// Buttons on the AI pages that stand in for a server streaming into the page: they set
// or remove an attribute on another element (data-gallery-attr="name",
// data-gallery-target="id", data-gallery-value="value" or none to remove it).
document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-gallery-attr]");
  if (!button) return;
  const target = document.getElementById(button.dataset.galleryTarget);
  if (!target) return;
  const name = button.dataset.galleryAttr;
  if (button.dataset.galleryValue === undefined) target.removeAttribute(name);
  else target.setAttribute(name, button.dataset.galleryValue);
});

// The response page's replay: each <template data-replay-stage> holds the blocks that
// changed at one step of a stream, as a server would send them.
document.addEventListener("click", async (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-gallery-replay]");
  if (!button) return;
  const root = document.getElementById(button.dataset.galleryReplay);
  const stages = [...root.parentElement.querySelectorAll(":scope > template[data-replay-stage]")];
  button.disabled = true;
  root.replaceChildren();
  root.setAttribute("data-streaming", "");
  for (const stage of stages) {
    const patches = [...stage.content.children].filter((t) => t.matches("template[data-index]")).map((t) => ({
      index: Number(t.dataset.index),
      html: t.innerHTML,
    }));
    window.bitopUi.patchResponse(root, patches, Number(stage.dataset.len));
    await new Promise((resolve) => setTimeout(resolve, 40));
  }
  root.removeAttribute("data-streaming");
  root.dataset.replayed = "";
  button.disabled = false;
});

// The conversation page's stand-in for a streamed reply: words are appended to an
// element one at a time, and the announcer's status follows.
document.addEventListener("click", async (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-gallery-stream]");
  if (!button) return;
  const target = document.getElementById(button.dataset.galleryStream);
  const announcer = document.getElementById(button.dataset.galleryAnnouncer);
  const words = (button.dataset.galleryWords ?? "").split(" ");
  const wait = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
  button.disabled = true;
  if (announcer) announcer.dataset.status = "submitted";
  await wait(300);
  if (announcer) announcer.dataset.status = "streaming";
  for (const word of words) {
    target.append(`${word} `);
    await wait(30);
  }
  if (announcer) announcer.dataset.status = "ready";
  button.disabled = false;
});

// The confirmation page's card that is answered in script: the page cancels the form
// and sets the state itself, as an app that calls its server with fetch would.
document.addEventListener("bitop:confirmation", (event) => {
  const card = event.target;
  if (!(card instanceof Element) || !card.hasAttribute("data-gallery-confirm")) return;
  event.preventDefault();
  card.dataset.state = event.detail.value === "approve" ? "accepted" : "rejected";
});

// The question page's card that is answered in script.
document.addEventListener("bitop:question-submit", (event) => {
  const form = event.target;
  if (!(form instanceof Element) || !form.hasAttribute("data-gallery-answer")) return;
  event.preventDefault();
  form.dataset.state = event.detail.skipped ? "skipped" : "answered";
});

// The questionnaire page's forms that are sent in script.
document.addEventListener("bitop:questionnaire-submit", (event) => {
  const form = event.target;
  if (!(form instanceof Element) || !form.hasAttribute("data-gallery-questionnaire")) return;
  event.preventDefault();
  form.dataset.sent = JSON.stringify(event.detail.answers);
});

// The voice selector page's list whose previews the page plays itself (here it only
// marks the voice as playing).
document.addEventListener("bitop:voice-preview", (event) => {
  const root = event.target;
  if (!(root instanceof Element) || !root.hasAttribute("data-gallery-voice")) return;
  event.preventDefault();
  if (event.detail.playing) delete root.dataset.previewing;
  else root.dataset.previewing = event.detail.id;
});

// The stack trace page's frames that send events: the page writes what it heard.
document.addEventListener("bitop:stack-frame", (event) => {
  const root = event.target;
  if (!(root instanceof Element) || !root.hasAttribute("data-gallery-frames")) return;
  const out = root.parentElement.querySelector("[data-frame-clicked]");
  if (out) out.textContent = `${event.detail.file}:${event.detail.line}:${event.detail.column}`;
});

// The file tree page's tree: the page writes the last event it heard.
for (const name of ["bitop:file-tree-select", "bitop:file-tree-toggle"]) {
  document.addEventListener(name, (event) => {
    const tree = event.target;
    if (!(tree instanceof Element) || !tree.hasAttribute("data-gallery-tree")) return;
    const out = tree.parentElement.querySelector("[data-tree-event]");
    const detail = event.detail;
    if (out) out.textContent = name.endsWith("select") ? `Selected ${detail.path} (${detail.kind})` : `${detail.open ? "Opened" : "Closed"} ${detail.path}`;
  });
}

// The terminal page's stand-in for a server streaming output: each template holds a
// chunk of spans rendered by the server, appended one at a time.
document.addEventListener("click", async (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-gallery-terminal]");
  if (!button) return;
  const id = button.dataset.galleryTerminal;
  const root = document.getElementById(id);
  const output = document.getElementById(`${id}-output`);
  const chunks = [...document.querySelectorAll(`[data-terminal-chunks="${id}"] > template`)];
  button.disabled = true;
  output.replaceChildren();
  root.setAttribute("data-streaming", "");
  for (const chunk of chunks) {
    await new Promise((resolve) => setTimeout(resolve, 60));
    output.append(chunk.content.cloneNode(true));
  }
  root.removeAttribute("data-streaming");
  root.dataset.streamed = "";
  button.disabled = false;
});

// The web preview page's preview: the page writes the last address it went to.
document.addEventListener("bitop:web-preview-navigate", (event) => {
  const root = event.target;
  if (!(root instanceof Element) || !root.hasAttribute("data-gallery-preview")) return;
  const out = root.parentElement.querySelector("[data-preview-event]");
  if (out) out.textContent = `Went to ${event.detail.url}`;
});

// The button page's "Save changes": loading for a moment, as while a request runs.
document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-gallery-loading]");
  if (!button) return;
  window.bitopUi.button.setLoading(button, true);
  button.firstChild.textContent = "Saving…";
  setTimeout(() => {
    window.bitopUi.button.setLoading(button, false);
    button.firstChild.textContent = "Save changes";
  }, 1600);
});
