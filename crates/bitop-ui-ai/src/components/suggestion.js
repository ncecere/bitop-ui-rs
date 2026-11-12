// Suggestion: a press puts the text in the target text box (and sends its form with
// data-suggestion-send), and tells the page through a bitop:suggestion event. A
// suggestion with a name is a plain submit button and needs no script.

document.addEventListener("click", (event) => {
  const button = event.target instanceof Element && event.target.closest("[data-suggestion]");
  if (!button || button.disabled) return;
  const text = button.dataset.suggestion;
  const proceed = button.dispatchEvent(
    new CustomEvent("bitop:suggestion", { bubbles: true, cancelable: true, detail: { suggestion: text } }),
  );
  const box = button.dataset.suggestionTarget && document.getElementById(button.dataset.suggestionTarget);
  if (!proceed || !box || box.disabled) return;
  box.value = text;
  // What typing would send, so that a growing text box and a send button follow.
  box.dispatchEvent(new Event("input", { bubbles: true }));
  box.focus();
  box.setSelectionRange?.(text.length, text.length);
  if (button.hasAttribute("data-suggestion-send")) box.form?.requestSubmit();
});
