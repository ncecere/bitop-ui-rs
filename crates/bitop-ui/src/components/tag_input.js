// Tag input: adds and removes tags. Each tag carries a hidden input, so the tags are
// submitted with the form as they stand.

function tagParts(target) {
  const root = target instanceof Element && target.closest("[data-tag-input]");
  if (!root) return undefined;
  return {
    root,
    list: root.querySelector(":scope > ul"),
    input: root.querySelector(':scope > input[type="text"]'),
    status: root.querySelector(':scope > [role="status"]'),
    template: root.querySelector(":scope > template"),
    max: Number(root.dataset.maxTags ?? Infinity),
    maxLength: Number(root.dataset.maxTagLength ?? 64),
  };
}

function tagValues(list) {
  return [...list.children].map((tag) => tag.dataset.tag);
}

// The input is disabled while the limit is reached, and says so.
function settleTags(parts) {
  const { root, list, input } = parts;
  if (root.hasAttribute("data-disabled")) return;
  const full = list.children.length >= parts.max;
  input.disabled = full;
  input.placeholder = full ? root.dataset.limitPlaceholder : root.dataset.placeholder;
  root.dispatchEvent(new Event("change", { bubbles: true }));
  root.dispatchEvent(new CustomEvent("bitop:change", { bubbles: true, detail: { value: tagValues(list) } }));
}

function addTags(parts, raw) {
  const { root, list, input, status, template } = parts;
  const values = tagValues(list);
  const added = [];
  for (const part of raw.split(",")) {
    let tag = part.trim();
    if (!root.hasAttribute("data-keep-case")) tag = tag.toLowerCase();
    tag = [...tag].slice(0, parts.maxLength).join("");
    if (!tag || values.includes(tag)) continue;
    if (values.length >= parts.max) break;
    const item = template.content.firstElementChild.cloneNode(true);
    item.dataset.tag = tag;
    item.querySelector("span").textContent = tag;
    item.querySelector("button").setAttribute("aria-label", root.dataset.removeLabel.replace("{tag}", tag));
    item.querySelector("input").value = tag;
    list.append(item);
    values.push(tag);
    added.push(tag);
  }
  input.value = "";
  if (!added.length) return;
  status.textContent =
    added.length === 1
      ? root.dataset.addedMessage.replace("{tag}", added[0])
      : root.dataset.addedManyMessage.replace("{count}", added.length);
  settleTags(parts);
}

function removeTag(parts, item) {
  const { root, input, status } = parts;
  item.remove();
  status.textContent = root.dataset.removedMessage.replace("{tag}", item.dataset.tag);
  settleTags(parts);
  input.focus();
}

document.addEventListener("keydown", (event) => {
  const parts = tagParts(event.target);
  if (!parts || event.target !== parts.input || event.isComposing) return;
  const { input, list } = parts;
  const pending = input.value.trim();
  if ((event.key === "Enter" || event.key === ",") && pending) {
    event.preventDefault();
    addTags(parts, input.value);
  } else if (event.key === "Enter") {
    // An empty Enter would submit the form by accident.
    event.preventDefault();
  } else if (event.key === "Tab" && !event.shiftKey && pending) {
    addTags(parts, input.value);
  } else if (event.key === "Backspace" && input.value === "" && list.lastElementChild) {
    event.preventDefault();
    removeTag(parts, list.lastElementChild);
  }
});

document.addEventListener("paste", (event) => {
  const parts = tagParts(event.target);
  if (!parts || event.target !== parts.input) return;
  const pasted = event.clipboardData?.getData("text") ?? "";
  if (!pasted.includes(",") && !pasted.includes("\n")) return;
  event.preventDefault();
  addTags(parts, parts.input.value + pasted.replace(/\r?\n/g, ","));
});

// Text left in the input becomes a tag, so that it is not lost on submit.
document.addEventListener("focusout", (event) => {
  const parts = tagParts(event.target);
  if (parts && event.target === parts.input && parts.input.value.trim()) addTags(parts, parts.input.value);
});

document.addEventListener("click", (event) => {
  const parts = tagParts(event.target);
  if (!parts || parts.root.hasAttribute("data-disabled")) return;
  const remove = event.target.closest("li > button");
  if (remove) removeTag(parts, remove.parentElement);
  // A click on the padding or between the tags goes to the input.
  else if (!event.target.closest("li")) parts.input.focus();
});
