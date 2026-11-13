// Drop zone: the button opens the file picker; dropped files go into the same file
// input. Files that fail accept, the size limit or the count limit are taken out of the
// input and named in the status line.

function zoneParts(target) {
  const zone = target instanceof Element && target.closest("[data-drop-zone]");
  if (!zone) return undefined;
  return {
    zone,
    input: zone.querySelector(':scope > input[type="file"]'),
    rejected: zone.querySelector(":scope > [data-drop-zone-rejected]"),
    list: zone.querySelector(":scope > [data-drop-zone-files]"),
  };
}

// Whether the file matches an accept list such as ".pdf,image/*,text/plain".
function matchesAccept(file, accept) {
  const rules = (accept ?? "")
    .split(",")
    .map((rule) => rule.trim().toLowerCase())
    .filter(Boolean);
  if (!rules.length) return true;
  const name = file.name.toLowerCase();
  const type = file.type.toLowerCase();
  return rules.some((rule) => {
    if (rule.startsWith(".")) return name.endsWith(rule);
    if (rule.endsWith("/*")) return type.startsWith(rule.slice(0, -1));
    return type === rule;
  });
}

function formatSize(bytes) {
  if (bytes < 1024) return `${bytes} B`;
  const units = ["KB", "MB", "GB"];
  let value = bytes / 1024;
  let unit = 0;
  while (value >= 1024 && unit < units.length - 1) {
    value /= 1024;
    unit += 1;
  }
  return `${Number(value.toFixed(1))} ${units[unit]}`;
}

// Sorts `files` into accepted and rejected, puts the accepted ones in the input, and
// shows both.
function deliverFiles(parts, files) {
  const { zone, input, rejected, list } = parts;
  if (input.disabled) return;
  const maxSize = Number(zone.dataset.maxSize ?? Infinity);
  const maxFiles = input.multiple ? Number(zone.dataset.maxFiles ?? Infinity) : 1;
  const accepted = [];
  const rejections = [];
  for (const file of files) {
    if (!matchesAccept(file, input.accept)) rejections.push({ file, reason: "type" });
    else if (file.size > maxSize) rejections.push({ file, reason: "size" });
    else if (accepted.length >= maxFiles) rejections.push({ file, reason: "count" });
    else accepted.push(file);
  }
  const kept = new DataTransfer();
  for (const file of accepted) kept.items.add(file);
  input.files = kept.files;

  rejected.textContent = rejections
    .map(({ file, reason }) => zone.dataset[`${reason}Message`].replace("{name}", file.name))
    .join(" ");
  list.replaceChildren(
    ...accepted.map((file) => {
      const item = document.createElement("li");
      const size = document.createElement("span");
      size.textContent = formatSize(file.size);
      item.append(file.name, size);
      return item;
    }),
  );
  zone.dispatchEvent(new CustomEvent("bitop:files", { bubbles: true, detail: { files: accepted, rejections } }));
}

document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-drop-zone-button]");
  if (button) zoneParts(button)?.input.click();
});

// True while the change event below is this script's own, sent after a drop.
let zoneDropped = false;

document.addEventListener("change", (event) => {
  const parts = zoneParts(event.target);
  if (parts && event.target === parts.input && !zoneDropped) deliverFiles(parts, [...parts.input.files]);
});

function dragOver(event) {
  const parts = zoneParts(event.target);
  if (!parts) return;
  // Without this the browser opens the dropped file instead.
  event.preventDefault();
  if (!parts.input.disabled) parts.zone.setAttribute("data-dragging", "");
}

document.addEventListener("dragenter", dragOver);
document.addEventListener("dragover", dragOver);

document.addEventListener("dragleave", (event) => {
  const parts = zoneParts(event.target);
  if (parts && !parts.zone.contains(event.relatedTarget)) parts.zone.removeAttribute("data-dragging");
});

document.addEventListener("drop", (event) => {
  const parts = zoneParts(event.target);
  if (!parts) return;
  event.preventDefault();
  parts.zone.removeAttribute("data-dragging");
  const files = [...(event.dataTransfer?.files ?? [])];
  if (!files.length) return;
  deliverFiles(parts, files);
  // What a page listening to the input expects after its files have changed.
  zoneDropped = true;
  parts.input.dispatchEvent(new Event("change", { bubbles: true }));
  zoneDropped = false;
});
