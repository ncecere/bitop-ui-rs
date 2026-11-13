// Prompt input: Enter to send, a growing text box, a counter, a submit button that
// follows the form's data-status, and files with previews.

const growsByItself = CSS.supports("field-sizing", "content");
const composerFiles = new WeakMap();
let fileIds = 0;

function composerParts(target) {
  const form = target instanceof Element && target.closest("[data-prompt-input]");
  if (!form) return undefined;
  return {
    form,
    textarea: form.querySelector("[data-prompt-textarea]"),
    submit: form.querySelector("[data-prompt-submit]"),
    input: form.querySelector(":scope > [data-prompt-files]"),
    list: form.querySelector("[data-prompt-attachments]"),
    template: form.querySelector("[data-prompt-attachment-template]"),
    error: form.querySelector(":scope > [data-prompt-file-error]"),
  };
}

const filesOf = (form) => composerFiles.get(form) ?? [];
const busy = (form) => ["submitted", "streaming"].includes(form.dataset.status);
const disabledComposer = (form) => form.hasAttribute("data-disabled");

function grow(textarea) {
  if (growsByItself || !textarea) return;
  textarea.style.blockSize = "auto";
  textarea.style.blockSize = `${textarea.scrollHeight}px`;
}

function updateCount(textarea) {
  const wrap = textarea.parentElement;
  const max = Number(textarea.getAttribute("maxlength"));
  if (!max) return;
  const length = textarea.value.length;
  const counter = wrap.querySelector("[data-prompt-count]");
  if (counter) {
    const mode = textarea.dataset.showCount;
    counter.hidden = !(mode === "always" || (mode === "auto" && length >= Math.floor(max * 0.8)));
    counter.toggleAttribute("data-limit", length >= max);
    counter.querySelector("[data-prompt-count-text]").textContent = `${length.toLocaleString()} / ${max.toLocaleString()}`;
  }
  const status = wrap.querySelector("[data-prompt-limit]");
  if (status) status.textContent = length >= max ? status.dataset.promptLimit : "";
}

// The submit button's state: what it does (send, sending, stop), its name and whether
// it can be pressed.
function updateSubmit(parts) {
  const { form, textarea, submit } = parts;
  if (!submit) return;
  const status = form.dataset.status ?? "ready";
  const early = submit.hasAttribute("data-stop-early");
  const mode = status === "streaming" || (status === "submitted" && early) ? "stop" : status === "submitted" ? "sending" : "send";
  submit.dataset.mode = mode;
  submit.dataset.status = status;
  submit.setAttribute("aria-label", submit.dataset[`label${mode[0].toUpperCase()}${mode.slice(1)}`]);
  const text = submit.querySelector("[data-prompt-submit-text]");
  if (text) text.textContent = text.dataset[mode];
  if (mode === "stop") {
    submit.name = submit.dataset.stopName;
    submit.value = submit.dataset.stopValue;
  } else {
    submit.removeAttribute("name");
    submit.removeAttribute("value");
  }
  const empty = !(textarea?.value.trim()) && filesOf(form).length === 0;
  submit.disabled = disabledComposer(form) || mode === "sending" || (mode === "send" && empty);
}

function sync(parts) {
  if (parts.textarea) {
    grow(parts.textarea);
    updateCount(parts.textarea);
  }
  updateSubmit(parts);
}

// ---------- files ----------

function matchesAccept(file, accept) {
  const rules = (accept ?? "").split(",").map((rule) => rule.trim().toLowerCase()).filter(Boolean);
  if (!rules.length) return true;
  const name = file.name.toLowerCase();
  const type = file.type.toLowerCase();
  return rules.some((rule) => {
    if (rule.startsWith(".")) return name.endsWith(rule);
    if (rule.endsWith("/*")) return type.startsWith(rule.slice(0, -1));
    return type === rule;
  });
}

function formatBytes(bytes) {
  if (bytes < 1024) return `${bytes} B`;
  const units = ["KB", "MB", "GB", "TB"];
  let value = bytes / 1024;
  let unit = 0;
  while (value >= 1024 && unit < units.length - 1) {
    value /= 1024;
    unit += 1;
  }
  return `${value >= 10 ? Math.round(value) : Math.round(value * 10) / 10} ${units[unit]}`;
}

// Replaces the composer's files, revoking the preview URL of every file that leaves.
function commitFiles(parts, next) {
  const { form, input } = parts;
  const kept = new Set(next.map((f) => f.id));
  for (const f of filesOf(form)) if (f.url && !kept.has(f.id)) URL.revokeObjectURL(f.url);
  composerFiles.set(form, next);
  if (input) {
    const transfer = new DataTransfer();
    for (const f of next) transfer.items.add(f.file);
    input.files = transfer.files;
  }
  renderChips(parts);
  updateSubmit(parts);
}

function renderChips(parts) {
  const { form, list, template } = parts;
  if (!list || !template) return;
  const chips = filesOf(form).map((f) => {
    const chip = template.content.firstElementChild.cloneNode(true);
    chip.dataset.attachmentId = f.id;
    const name = chip.querySelector("[data-attachment-name]");
    name.textContent = f.file.name;
    name.title = f.file.name;
    const dot = f.file.name.lastIndexOf(".");
    const extension = dot > 0 ? f.file.name.slice(dot + 1).toUpperCase() : "";
    chip.querySelector("[data-attachment-meta]").textContent = [extension, formatBytes(f.file.size)].filter(Boolean).join(" · ");
    const preview = chip.querySelector("[data-attachment-preview]");
    const kind = f.file.type.startsWith("image/") ? "image" : /^text\/|pdf|word|document/.test(f.file.type) ? "file-text" : "file";
    preview.querySelector("[data-icon]")?.setAttribute("data-icon", kind);
    if (f.url) {
      const img = document.createElement("img");
      img.src = f.url;
      img.alt = "";
      const imageClass = template.dataset.imageClass;
      if (imageClass) img.className = imageClass;
      preview.replaceChildren(img);
    }
    const remove = chip.querySelector("[data-attachment-remove]");
    if (remove) {
      remove.setAttribute("aria-label", template.dataset.removeLabel.replace("{name}", f.file.name));
      remove.disabled = disabledComposer(form);
    }
    return chip;
  });
  list.replaceChildren(...chips);
}

function addFiles(parts, incoming) {
  const { form, input, error } = parts;
  if (!input || input.disabled || disabledComposer(form)) return;
  const maxSize = Number(form.dataset.maxSize ?? Infinity);
  const maxFiles = Number(form.dataset.maxFiles ?? Infinity);
  const messages = [];
  let accepted = [];
  let count = input.multiple ? filesOf(form).length : 0;
  for (const file of incoming) {
    if (!matchesAccept(file, input.accept)) {
      messages.push(form.dataset.typeError.replace("{name}", file.name));
      continue;
    }
    if (file.size > maxSize) {
      messages.push(form.dataset.sizeError.replace("{name}", file.name));
      continue;
    }
    if (count >= maxFiles) {
      messages.push(form.dataset.countError);
      break;
    }
    count += 1;
    accepted.push(file);
  }
  if (error) error.textContent = messages.join(" ");
  if (!accepted.length) return;
  // One file at a time: the first replaces the current one.
  if (!input.multiple) accepted = accepted.slice(0, 1);
  // Preview URLs only for the files that are kept.
  const added = accepted.map((file) => ({
    id: `prompt-file-${++fileIds}`,
    file,
    url: file.type.startsWith("image/") ? URL.createObjectURL(file) : undefined,
  }));
  commitFiles(parts, input.multiple ? [...filesOf(form), ...added] : added);
}

// ---------- events ----------

document.addEventListener("change", (event) => {
  const parts = composerParts(event.target);
  if (!parts || event.target !== parts.input) return;
  // The picker replaced the input's files: add them to those already chosen.
  const picked = [...parts.input.files];
  commitFiles(parts, filesOf(parts.form));
  addFiles(parts, picked);
  parts.textarea?.focus();
});

document.addEventListener("input", (event) => {
  const parts = composerParts(event.target);
  if (parts && event.target === parts.textarea) sync(parts);
});

// Composition: the Enter that confirms it must not send. Safari reports keyCode 229.
const composing = new WeakSet();
document.addEventListener("compositionstart", (event) => event.target instanceof Element && composing.add(event.target));
document.addEventListener("compositionend", (event) => composing.delete(event.target));

document.addEventListener("keydown", (event) => {
  const parts = composerParts(event.target);
  if (!parts || event.target !== parts.textarea || event.key !== "Enter" || event.defaultPrevented) return;
  if (composing.has(event.target) || event.isComposing || event.keyCode === 229) return;
  const modifier = event.metaKey || event.ctrlKey;
  if (event.shiftKey || (parts.textarea.dataset.submitOnEnter === "false" && !modifier)) return;
  event.preventDefault();
  if (busy(parts.form)) return;
  const send = parts.submit?.dataset.mode === "send" ? parts.submit : undefined;
  parts.form.requestSubmit(send && !send.disabled ? send : undefined);
});

document.addEventListener("click", (event) => {
  if (!(event.target instanceof Element)) return;
  const attach = event.target.closest("[data-prompt-attach]");
  if (attach) {
    const parts = composerParts(attach);
    if (parts?.input && !attach.disabled && !disabledComposer(parts.form)) parts.input.click();
    return;
  }
  const submit = event.target.closest("[data-prompt-submit]");
  if (submit?.dataset.mode === "stop") {
    const proceed = submit.dispatchEvent(new CustomEvent("bitop:prompt-stop", { bubbles: true, cancelable: true }));
    if (!proceed) event.preventDefault();
  }
});

// A chip's remove button: the composer drops the file and returns focus to the text box.
document.addEventListener("bitop:attachment-remove", (event) => {
  const parts = composerParts(event.target);
  if (!parts?.list || event.target !== parts.list) return;
  event.preventDefault();
  if (disabledComposer(parts.form)) return;
  commitFiles(parts, filesOf(parts.form).filter((f) => f.id !== event.detail.id));
  parts.textarea?.focus();
});

document.addEventListener("paste", (event) => {
  const parts = composerParts(event.target);
  if (!parts?.input || event.target !== parts.textarea) return;
  const pasted = [...(event.clipboardData?.files ?? [])];
  if (!pasted.length) return;
  event.preventDefault();
  addFiles(parts, pasted);
});

function dragOver(event) {
  const parts = composerParts(event.target);
  if (!parts?.input || disabledComposer(parts.form) || parts.input.disabled) return;
  if (!event.dataTransfer?.types.includes("Files")) return;
  // Without this the browser opens the dropped file instead.
  event.preventDefault();
  parts.form.setAttribute("data-dragging", "");
}
document.addEventListener("dragenter", dragOver);
document.addEventListener("dragover", dragOver);
document.addEventListener("dragleave", (event) => {
  const parts = composerParts(event.target);
  if (parts && !parts.form.contains(event.relatedTarget)) parts.form.removeAttribute("data-dragging");
});
document.addEventListener("drop", (event) => {
  const parts = composerParts(event.target);
  if (!parts?.input) return;
  event.preventDefault();
  parts.form.removeAttribute("data-dragging");
  const dropped = [...(event.dataTransfer?.files ?? [])];
  if (dropped.length) addFiles(parts, dropped);
});

// Before the form is sent: nothing to send, or an answer still on its way, stops it.
// Otherwise the page hears of it first; a page that cancels bitop:prompt-submit sends
// the message itself, and the composer clears what was sent.
document.addEventListener("submit", (event) => {
  const parts = composerParts(event.target);
  if (!parts || event.target !== parts.form) return;
  const { form, textarea } = parts;
  const stopping = event.submitter?.dataset.mode === "stop";
  if (stopping) return;
  const text = textarea?.value ?? "";
  const files = filesOf(form);
  if (disabledComposer(form) || busy(form) || (!text.trim() && files.length === 0)) {
    event.preventDefault();
    return;
  }
  const proceed = form.dispatchEvent(
    new CustomEvent("bitop:prompt-submit", { bubbles: true, cancelable: true, detail: { text, files: files.map((f) => f.file) } }),
  );
  if (proceed) return;
  event.preventDefault();
  if (textarea && textarea.value === text) textarea.value = "";
  const sent = new Set(files.map((f) => f.id));
  commitFiles(parts, filesOf(form).filter((f) => !sent.has(f.id)));
  sync(parts);
  textarea?.focus();
});

// The page goes away: release every preview.
addEventListener("pagehide", () => {
  for (const form of document.querySelectorAll("[data-prompt-input]")) {
    for (const f of filesOf(form)) if (f.url) URL.revokeObjectURL(f.url);
  }
});

// An app changes data-status (or data-disabled) on the form: the button follows.
new MutationObserver((records) => {
  for (const record of records) {
    const parts = composerParts(record.target);
    if (parts && record.target === parts.form) {
      if (record.attributeName === "data-disabled") {
        const off = disabledComposer(parts.form);
        for (const control of parts.form.querySelectorAll("[data-prompt-textarea], [data-prompt-tool], [data-prompt-files], [data-attachment-remove]")) {
          control.disabled = off;
        }
      }
      updateSubmit(parts);
    }
  }
}).observe(document.documentElement, { subtree: true, attributes: true, attributeFilter: ["data-status", "data-disabled"] });

for (const form of document.querySelectorAll("[data-prompt-input]")) sync(composerParts(form));
