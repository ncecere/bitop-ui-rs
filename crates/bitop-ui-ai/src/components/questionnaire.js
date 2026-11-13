// Questionnaire: shows one server-rendered step at a time. Next checks the current
// question, Back and Edit go back, Skip empties an optional question, and focus moves
// to each new step's heading. Uses window.bitopUi.questionFields from question.js.

function questionnaireParts(form) {
  const steps = [...form.querySelectorAll(":scope > [data-questionnaire-step]")];
  return {
    steps,
    total: Number(form.dataset.total),
    progress: form.querySelector(":scope > [data-questionnaire-progress]"),
    back: form.querySelector("[data-questionnaire-back]"),
    skip: form.querySelector("[data-questionnaire-skip]"),
    next: form.querySelector("[data-questionnaire-next]"),
  };
}

const currentStep = (form) => Number(form.dataset.step ?? 0);
const fieldsAt = (form, index) => form.querySelector(`:scope > [data-questionnaire-step="${index}"] [data-question-fields]`);

// The answer of one question as text, for the review: the chosen labels and the text.
function answerText(fields) {
  const helpers = window.bitopUi.questionFields;
  if (!helpers.answered(fields)) return undefined;
  const { selected, text } = helpers.value(fields);
  const labels = selected.map((value) => {
    const input = [...fields.querySelectorAll("[data-question-group] input")].find((i) => i.value === value);
    return input?.nextElementSibling?.textContent.trim() ?? value;
  });
  if (text.trim()) labels.push(text.trim());
  return labels.join(", ");
}

function fillReview(form) {
  for (const answer of form.querySelectorAll("[data-questionnaire-answer]")) {
    const fields = fieldsAt(form, answer.dataset.questionnaireAnswer);
    const text = fields ? answerText(fields) : undefined;
    answer.textContent = text ?? form.dataset.skippedLabel;
    answer.toggleAttribute("data-skipped", text === undefined);
  }
}

function showStep(form, index, focus = true) {
  const { steps, total, progress, back, skip, next } = questionnaireParts(form);
  const last = steps.length - 1;
  index = Math.max(0, Math.min(index, last));
  form.dataset.step = String(index);
  const step = steps[index];
  const onReview = step?.hasAttribute("data-review");
  for (const other of steps) other.hidden = other !== step;
  if (onReview) fillReview(form);
  // The previous step's error does not follow the reader back to it.
  const fields = step?.querySelector("[data-question-fields]");
  if (fields) window.bitopUi.questionFields.showError(fields, false);
  if (progress) {
    const label = onReview
      ? form.dataset.reviewLabel
      : form.dataset.progressLabel.replace("{current}", String(index + 1)).replace("{total}", String(total));
    const max = Math.max(total, 1);
    const value = onReview ? total : index + 1;
    progress.setAttribute("aria-label", label);
    progress.setAttribute("aria-valuenow", String(value));
    progress.setAttribute("aria-valuetext", `${Math.round((value / max) * 100)}%`);
    const text = progress.firstElementChild?.firstElementChild;
    if (text) text.textContent = label;
    const indicator = progress.lastElementChild?.firstElementChild;
    if (indicator) indicator.style.width = `${Math.round((value / max) * 100)}%`;
  }
  if (back) back.hidden = index === 0;
  if (skip) skip.hidden = !step?.hasAttribute("data-optional");
  if (next) next.textContent = index === last ? form.dataset.submitLabel : form.dataset.nextLabel;
  if (focus) step?.querySelector("h2, h3, h4, h5, h6")?.focus();
}

// Whether the current question may be left: answered, or optional.
function stepDone(form) {
  const index = currentStep(form);
  const step = form.querySelector(`:scope > [data-questionnaire-step="${index}"]`);
  const fields = fieldsAt(form, index);
  if (!fields || step.hasAttribute("data-optional")) return true;
  const helpers = window.bitopUi.questionFields;
  if (helpers.answered(fields)) return true;
  helpers.showError(fields, true);
  helpers.focusFirst(fields);
  return false;
}

function questionnaireAnswers(form) {
  const answers = {};
  const helpers = window.bitopUi.questionFields;
  for (const fields of form.querySelectorAll("[data-question-fields]")) {
    const { selected, other, text } = helpers.value(fields);
    const id = fields.dataset.name;
    answers[id] = helpers.answered(fields)
      ? { selected, text: fields.dataset.kind === "freeform" || other ? text.trim() : undefined }
      : null;
  }
  return answers;
}

document.addEventListener("submit", (event) => {
  const form = event.target;
  if (!(form instanceof HTMLFormElement) || !form.hasAttribute("data-questionnaire")) return;
  const { steps } = questionnaireParts(form);
  const index = currentStep(form);
  if (!stepDone(form)) {
    event.preventDefault();
    return;
  }
  if (index < steps.length - 1) {
    event.preventDefault();
    showStep(form, index + 1);
    return;
  }
  const proceed = form.dispatchEvent(
    new CustomEvent("bitop:questionnaire-submit", { bubbles: true, cancelable: true, detail: { answers: questionnaireAnswers(form) } }),
  );
  if (!proceed) event.preventDefault();
});

document.addEventListener("click", (event) => {
  if (!(event.target instanceof Element)) return;
  const form = event.target.closest("[data-questionnaire]");
  if (!form) return;
  const button = event.target.closest("button");
  if (!button || button.disabled) return;
  const index = currentStep(form);
  if (button.hasAttribute("data-questionnaire-back")) {
    showStep(form, index - 1);
  } else if (button.hasAttribute("data-questionnaire-edit")) {
    showStep(form, Number(button.dataset.questionnaireEdit));
  } else if (button.hasAttribute("data-questionnaire-skip")) {
    const fields = fieldsAt(form, index);
    if (fields) window.bitopUi.questionFields.clear(fields);
    const { steps, next } = questionnaireParts(form);
    if (index < steps.length - 1) showStep(form, index + 1);
    else form.requestSubmit(next);
  }
});
