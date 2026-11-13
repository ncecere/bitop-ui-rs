// The chat example's stand-in for a server: it plays a canned reply into the page,
// as an app streaming HTML would. Not part of the library.

const root = document.querySelector("[data-chat-example]");
if (root) {
  const form = document.getElementById("chat-composer");
  const log = document.getElementById("chat-log");
  const turns = document.getElementById("chat-turns");
  const empty = document.getElementById("chat-empty");
  const announcer = document.getElementById("chat-announcer");
  const template = (name) => document.querySelector(`template[data-chat-${name}]`);
  const stages = [...document.querySelectorAll("template[data-chat-stage]")];
  const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
  let run = 0;
  let replies = 0;
  let current;

  const setStatus = (status) => {
    form.dataset.status = status;
    announcer.dataset.status = status;
  };

  // Each reply gets its own IDs, and so do the references to them.
  const REFERENCES = ["id", "popovertarget", "data-citation-trigger", "aria-labelledby", "aria-describedby", "aria-controls", "data-tooltip", "data-citation-activate", "data-citation-go"];
  const uniquify = (node, n) => {
    for (const element of node.querySelectorAll("*")) {
      for (const name of REFERENCES) {
        const value = element.getAttribute(name);
        if (value) element.setAttribute(name, value.split(" ").map((id) => `${id}-${n}`).join(" "));
      }
    }
  };

  const open = () => {
    empty.hidden = true;
    turns.hidden = false;
    log.dataset.stick = "true";
  };

  function addUserTurn(text, files) {
    const turn = template("user").content.cloneNode(true);
    turn.querySelector("[data-user-text]").textContent = text;
    const list = turn.querySelector("[data-user-files]");
    for (const file of files) {
      const chip = template("file").content.cloneNode(true);
      chip.querySelector("[data-file-name]").textContent = file.name;
      list.append(chip);
    }
    if (!files.length) list.remove();
    turns.append(turn);
  }

  // The context window, counted as upstream's example counts it: 1,800 tokens, plus 12
  // for every four characters of the conversation. A server would render the context
  // again; this stand-in writes the numbers into the one it rendered.
  const contextCard = document.getElementById("chat-context");
  const contextTrigger = document.querySelector('[popovertarget="chat-context"]');
  const MAX_TOKENS = 200_000;
  const grouped = new Intl.NumberFormat("en-US");
  const compact = new Intl.NumberFormat("en-US", { notation: "compact", maximumFractionDigits: 1 });
  const money = new Intl.NumberFormat("en-US", { minimumFractionDigits: 2, maximumFractionDigits: 4 });

  function updateContext() {
    const texts = [...turns.querySelectorAll("[data-user-text], [data-response]")];
    const chars = texts.reduce((n, el) => n + el.textContent.trim().length, 0);
    const used = 1_800 + Math.round(chars / 4) * 12;
    const ratio = Math.min(used / MAX_TOKENS, 1);
    const percent = Math.round(ratio * 100);
    contextTrigger.querySelector("svg").style.setProperty("--ratio", ratio.toFixed(4));
    contextTrigger.querySelector(":scope > span").textContent = `${percent}%`;
    contextTrigger.setAttribute("aria-label", `Context window: ${percent}% used, ${grouped.format(used)} of ${grouped.format(MAX_TOKENS)} tokens`);
    contextTrigger.dataset.level = ratio >= 0.9 ? "critical" : ratio >= 0.75 ? "high" : "normal";
    const meter = contextCard.querySelector("[data-context-meter]");
    const text = `${percent}% · ${compact.format(used)} / ${compact.format(MAX_TOKENS)}`;
    meter.setAttribute("aria-valuenow", String(used));
    meter.setAttribute("aria-valuetext", text);
    meter.querySelector("[aria-hidden]").lastChild.textContent = text;
    meter.querySelector("[style]").style.width = `${(ratio * 100).toFixed(4)}%`;
    const shares = [0.7, 0.22, 0.08];
    contextCard.querySelectorAll("dd > span:first-child").forEach((span, i) => {
      span.textContent = `${grouped.format(Math.round(used * shares[i]))} tokens`;
    });
    contextCard.querySelector("dl + div > span:last-child").textContent = `$${money.format(used * 0.000006)}`;
  }

  function finish(reply, stopped) {
    reply.querySelector("[data-reply-reasoning]")?.removeAttribute("data-streaming");
    const answer = reply.querySelectorAll("[data-response]")[1];
    answer?.removeAttribute("data-streaming");
    if (stopped) reply.querySelector("[data-reply-stopped]").hidden = false;
    else reply.querySelector("[data-reply-sources]").hidden = false;
    reply.querySelector("[data-reply-actions]").hidden = false;
    setStatus("ready");
    updateContext();
  }

  async function respond(replace) {
    const token = ++run;
    const n = ++replies;
    const alive = () => run === token;
    const fragment = template("reply").content.cloneNode(true);
    uniquify(fragment, n);
    const reply = fragment.firstElementChild;
    if (replace) replace.replaceWith(fragment);
    else turns.append(fragment);
    current = reply;
    setStatus("submitted");
    await sleep(500);
    if (!alive()) return;
    setStatus("streaming");
    await sleep(700);
    if (!alive()) return;
    reply.querySelector("[data-reply-reasoning]").removeAttribute("data-streaming");
    const answer = reply.querySelectorAll("[data-response]")[1];
    for (const stage of stages) {
      const patches = [...stage.content.children].map((t) => {
        const holder = document.createElement("template");
        holder.innerHTML = t.innerHTML;
        uniquify(holder.content, n);
        return { index: Number(t.dataset.index), html: holder.innerHTML };
      });
      window.bitopUi.patchResponse(answer, patches, Number(stage.dataset.len));
      await sleep(25);
      if (!alive()) return;
    }
    finish(reply, false);
  }

  form.addEventListener("bitop:prompt-submit", (event) => {
    event.preventDefault();
    open();
    addUserTurn(event.detail.text, event.detail.files);
    respond();
  });

  form.addEventListener("bitop:prompt-stop", (event) => {
    event.preventDefault();
    run += 1;
    if (current) finish(current, true);
  });

  document.addEventListener("click", (event) => {
    const regenerate = event.target instanceof Element && event.target.closest("[data-reply-regenerate]");
    if (!regenerate || form.dataset.status !== "ready") return;
    respond(regenerate.closest("[data-message]"));
  });
}
