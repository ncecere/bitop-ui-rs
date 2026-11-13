// Question: checks that there is an answer before the form is sent, shows the "Other"
// text box only while "Other" is chosen, and follows data-state on the form. The
// helpers for one question's fields are shared on window.bitopUi.questionFields, for
// the questionnaire.

const questionFields = {
  // What the fields hold: the chosen values (without "Other"), whether "Other" is
  // chosen, and the text.
  value(fields) {
    const kind = fields.dataset.kind;
    if (kind === "freeform") return { selected: [], other: false, text: fields.querySelector("[data-question-text]")?.value ?? "" };
    const checked = [...fields.querySelectorAll("[data-question-group] input:checked")];
    const other = checked.some((input) => input.hasAttribute("data-question-other"));
    return {
      selected: checked.filter((input) => !input.hasAttribute("data-question-other")).map((input) => input.value),
      other,
      text: other ? (fields.querySelector("[data-question-other-text]")?.value ?? "") : "",
    };
  },

  answered(fields) {
    const { selected, other, text } = questionFields.value(fields);
    if (fields.dataset.kind === "freeform") return text.trim() !== "";
    return selected.length > 0 || (other && text.trim() !== "");
  },

  // Shows or hides the error, and ties it to the group or the text box.
  showError(fields, shown) {
    const error = fields.querySelector("[data-question-error]");
    const control = fields.querySelector("[data-question-group], [data-question-text]");
    if (!error || !control) return;
    error.hidden = !shown;
    const ids = (control.getAttribute("aria-describedby") ?? "").split(" ").filter((id) => id && id !== error.id);
    if (shown) ids.push(error.id);
    if (ids.length) control.setAttribute("aria-describedby", ids.join(" "));
    else control.removeAttribute("aria-describedby");
    if (shown) control.setAttribute("aria-invalid", "true");
    else control.removeAttribute("aria-invalid");
  },

  // Focus on the first control that can take it: the chosen radio, or the first.
  focusFirst(fields) {
    const controls = [...fields.querySelectorAll("input:not([type=hidden]), textarea")].filter(
      (control) => !control.disabled && control.checkVisibility(),
    );
    const radios = controls.filter((control) => control.type === "radio");
    const target = radios.find((radio) => radio.checked) ?? controls[0];
    target?.focus();
  },

  // The "Other" text box is sent only while "Other" is chosen.
  syncOther(fields) {
    const text = fields.querySelector("[data-question-other-text]");
    const other = fields.querySelector("[data-question-other]");
    if (!text || !other) return;
    const group = fields.querySelector("[data-question-group]");
    text.disabled = !other.checked || other.disabled || group?.disabled;
  },

  // Empties the fields, for a skipped question.
  clear(fields) {
    for (const input of fields.querySelectorAll("[data-question-group] input")) input.checked = false;
    for (const box of fields.querySelectorAll("[data-question-text], [data-question-other-text]")) box.value = "";
    questionFields.syncOther(fields);
    questionFields.showError(fields, false);
  },
};
window.bitopUi.questionFields = questionFields;

const fieldsOf = (target) => (target instanceof Element ? target.closest("[data-question-fields]") : null);

function questionEdited(event) {
  const fields = fieldsOf(event.target);
  if (!fields) return;
  questionFields.syncOther(fields);
  const error = fields.querySelector("[data-question-error]");
  if (error && !error.hidden && questionFields.answered(fields)) questionFields.showError(fields, false);
}
document.addEventListener("change", questionEdited);
document.addEventListener("input", questionEdited);

const questionFocus = new WeakSet();
document.addEventListener("focusin", (event) => {
  const form = event.target instanceof Element && event.target.closest("[data-question]");
  if (form) questionFocus.add(form);
});
document.addEventListener("focusout", (event) => {
  const form = event.target instanceof Element && event.target.closest("[data-question]");
  if (form && !form.contains(event.relatedTarget)) questionFocus.delete(form);
});

document.addEventListener("submit", (event) => {
  const form = event.target;
  if (!(form instanceof HTMLFormElement) || !form.hasAttribute("data-question")) return;
  const fields = form.querySelector("[data-question-fields]");
  if (form.dataset.state !== "pending" || !fields) {
    event.preventDefault();
    return;
  }
  const skipped = event.submitter?.hasAttribute("data-question-skip") ?? false;
  if (!skipped && !questionFields.answered(fields)) {
    event.preventDefault();
    questionFields.showError(fields, true);
    questionFields.focusFirst(fields);
    return;
  }
  const { selected, other, text } = questionFields.value(fields);
  const proceed = form.dispatchEvent(
    new CustomEvent("bitop:question-submit", {
      bubbles: true,
      cancelable: true,
      detail: skipped ? { skipped, selected: [], text: "" } : { skipped, selected, text: fields.dataset.kind === "freeform" || other ? text.trim() : "" },
    }),
  );
  if (!proceed) event.preventDefault();
});

// The page answered in script: lock the controls, say so, and keep focus in the card.
function questionStateChanged(form) {
  const state = form.dataset.state;
  const locked = state !== "pending";
  for (const control of form.querySelectorAll("[data-question-group], [data-question-text]")) control.disabled = locked;
  const fields = form.querySelector("[data-question-fields]");
  if (fields) questionFields.syncOther(fields);
  if (locked && fields) questionFields.showError(fields, false);
  const status = form.querySelector("[data-question-status]");
  if (status) status.textContent = state === "answered" ? form.dataset.answeredLabel : state === "skipped" ? form.dataset.skippedLabel : "";
  const active = document.activeElement;
  const lost = !form.contains(active) || active?.matches("button, [data-question-group] *, [data-question-text]");
  if (locked && questionFocus.has(form) && lost) form.focus({ preventScroll: true });
}

new MutationObserver((records) => {
  for (const record of records) {
    const form = record.target;
    if (form instanceof Element && form.matches("[data-question]") && form.dataset.state !== record.oldValue) questionStateChanged(form);
  }
}).observe(document.documentElement, { subtree: true, attributes: true, attributeFilter: ["data-state"], attributeOldValue: true });

for (const fields of document.querySelectorAll("[data-question-fields]")) questionFields.syncOther(fields);
