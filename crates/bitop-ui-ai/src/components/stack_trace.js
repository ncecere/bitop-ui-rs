// Stack trace: a frame's location button sends bitop:stack-frame from the root, with
// the frame's file, line, column and text, for an app that opens it in an editor.

document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-stack-frame]");
  if (!button) return;
  const root = button.closest("[data-stack-trace]") ?? button;
  const number = (value) => (value ? Number(value) : null);
  root.dispatchEvent(
    new CustomEvent("bitop:stack-frame", {
      bubbles: true,
      detail: {
        file: button.dataset.file || null,
        line: number(button.dataset.line),
        column: number(button.dataset.column),
        raw: button.closest("li")?.textContent.replace(/\s+/g, " ").trim() ?? "",
      },
    }),
  );
});
