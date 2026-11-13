// Mic selector: lists the audio inputs into the server-rendered <select>, asks for
// permission (which browsers need before they name devices) and follows devicechange.
// Without navigator.mediaDevices it shows the unsupported text instead.

// Drops the USB "(vendor:product)" suffix Chrome adds to device names.
function micLabel(device, index, template) {
  return device.label.replace(/\s*\([\da-f]{4}:[\da-f]{4}\)$/i, "") || template.replace("{n}", String(index + 1));
}

function micError(root, error) {
  const d = root.dataset;
  if (error.name === "NotAllowedError" || error.name === "SecurityError") return d.blockedText;
  if (error.name === "NotFoundError") return d.notFoundText;
  return d.errorText;
}

function micParts(root) {
  return {
    field: root.querySelector("[data-mic-field]"),
    select: root.querySelector("[data-mic-field] select"),
    label: root.querySelector("[data-mic-label]"),
    permission: root.querySelector("[data-mic-permission]"),
    unsupported: root.querySelector("[data-mic-unsupported]"),
    status: root.querySelector("[data-mic-status]"),
  };
}

function setMicStatus(root, text, error) {
  const { status } = micParts(root);
  if (!status) return;
  status.replaceChildren();
  if (!text) return;
  const template = root.querySelector(error ? "template[data-mic-error-template]" : "template[data-mic-message-template]");
  const message = template.content.firstElementChild.cloneNode(true);
  message.querySelector("[data-mic-text]").textContent = text;
  status.append(message);
}

async function refreshMics(root) {
  const parts = micParts(root);
  const media = navigator.mediaDevices;
  if (!media?.enumerateDevices) {
    root.dataset.status = "unsupported";
    parts.field.hidden = true;
    parts.label.hidden = false;
    parts.permission.hidden = true;
    parts.unsupported.hidden = false;
    return;
  }
  let devices;
  try {
    devices = (await media.enumerateDevices()).filter((device) => device.kind === "audioinput");
  } catch (error) {
    root.dataset.status = "error";
    setMicStatus(root, micError(root, error), true);
    return;
  }
  const hasPermission = devices.some((device) => device.label !== "");
  // Before permission some browsers list devices with an empty ID: they cannot be chosen.
  const usable = devices.filter((device) => device.deviceId !== "");
  const { select } = parts;
  const chosen = select.value || root.dataset.value || "";
  const placeholder = select.querySelector("option[value='']");
  const options = usable.map((device, index) => new Option(micLabel(device, index, root.dataset.unnamed), device.deviceId));
  select.replaceChildren(...(placeholder ? [placeholder] : []), ...options);
  select.value = usable.some((device) => device.deviceId === chosen) ? chosen : "";
  select.disabled = root.hasAttribute("data-disabled");
  root.dataset.status = "ready";
  root.toggleAttribute("data-permission", hasPermission);
  parts.field.hidden = usable.length === 0;
  parts.label.hidden = usable.length > 0;
  parts.permission.hidden = hasPermission;
  if (!root.dataset.failed) setMicStatus(root, hasPermission && usable.length === 0 ? root.dataset.emptyText : "", false);
}

document.addEventListener("click", async (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-mic-permission]");
  if (!button || button.disabled || button.getAttribute("aria-busy") === "true") return;
  const root = button.closest("[data-mic-selector]");
  button.setAttribute("aria-busy", "true");
  try {
    const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
    for (const track of stream.getTracks()) track.stop();
    delete root.dataset.failed;
    setMicStatus(root, "", false);
    await refreshMics(root);
    root.dispatchEvent(new CustomEvent("bitop:mic-permission", { bubbles: true, detail: { granted: true } }));
  } catch (error) {
    root.dataset.failed = error.name || "Error";
    setMicStatus(root, micError(root, error), true);
    root.dispatchEvent(new CustomEvent("bitop:mic-permission", { bubbles: true, detail: { granted: false, error: error.name } }));
  } finally {
    button.removeAttribute("aria-busy");
  }
});

navigator.mediaDevices?.addEventListener?.("devicechange", () => {
  for (const root of document.querySelectorAll("[data-mic-selector]")) refreshMics(root);
});

for (const root of document.querySelectorAll("[data-mic-selector]")) refreshMics(root);
window.bitopUi.refreshMicSelectors = () => Promise.all([...document.querySelectorAll("[data-mic-selector]")].map(refreshMics));
