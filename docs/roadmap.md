# Roadmap

The target is every component in [bitop-ui](https://github.com/ncecere/bitop-ui): 120 in all.
120 are built. The base set comes first, then the AI set as its own crate.

Names are bitop-ui's. A few live together here, as they do there: `switch` is in the `checkbox`
module, `textarea` in `input`, `alert dialog` in `dialog`, and `app-shell` is `sidebar`.

Order of work: forms, overlays, navigation and layout, display and feedback, data and charts,
then AI chat, agents and tools, code, and voice.

## Base: forms (20 of 20)

Crate: `bitop-ui`.

- [x] `button`
- [x] `button-group`
- [x] `checkbox`
- [x] `switch`
- [x] `radio-group`
- [x] `toggle`
- [x] `toggle-group`
- [x] `input`
- [x] `input-group`
- [x] `input-otp`
- [x] `number-input`
- [x] `tag-input`
- [x] `color-field`
- [x] `field`
- [x] `select`
- [x] `combobox`
- [x] `slider`
- [x] `calendar`
- [x] `date-picker`
- [x] `drop-zone`

## Base: overlays (11 of 11)

Crate: `bitop-ui`.

- [x] `dialog`
- [x] `sheet`
- [x] `drawer`
- [x] `popover`
- [x] `hover-card`
- [x] `tooltip`
- [x] `menu`
- [x] `context-menu`
- [x] `menubar`
- [x] `command-palette`
- [x] `toast`

## Base: navigation and layout (13 of 13)

Crate: `bitop-ui`.

- [x] `app-shell`
- [x] `navigation-menu`
- [x] `breadcrumbs`
- [x] `pagination`
- [x] `tabs`
- [x] `page-header`
- [x] `layout`
- [x] `separator`
- [x] `scroll-area`
- [x] `resizable`
- [x] `aspect-ratio`
- [x] `direction`
- [x] `save-bar`

## Base: display (18 of 18)

Crate: `bitop-ui`.

- [x] `avatar`
- [x] `badge`
- [x] `card`
- [x] `alert`
- [x] `item`
- [x] `kbd`
- [x] `marker`
- [x] `text-link`
- [x] `time`
- [x] `description-list`
- [x] `stat-card`
- [x] `empty-state`
- [x] `checklist`
- [x] `accordion`
- [x] `disclosure`
- [x] `carousel`
- [x] `copy-button`
- [x] `copy-field`

## Base: feedback (7 of 7)

Crate: `bitop-ui`.

- [x] `spinner`
- [x] `loader`
- [x] `skeleton`
- [x] `progress`
- [x] `meter`
- [x] `query-state`
- [x] `shimmer`

## Base: utilities (2 of 2)

Crate: `bitop-ui`.

- [x] `color-mode`
- [x] `visually-hidden`

## Data and charts (8 of 8)

Crate: `bitop-ui`.

- [x] `table`
- [x] `data-table`
- [x] `filter-bar`
- [x] `diff-viewer`
- [x] `chart`
- [x] `bar-chart`
- [x] `line-chart`
- [x] `sparkline`

## AI: chat (13 of 13)

Crate: `bitop-ui-ai`.

- [x] `conversation`
- [x] `message`
- [x] `response`
- [x] `prompt-input`
- [x] `suggestion`
- [x] `model-selector`
- [x] `attachments`
- [x] `reasoning`
- [x] `sources`
- [x] `inline-citation`
- [x] `code-block`
- [x] `image`
- [x] `open-in-chat`

## AI: agents and tools (12 of 12)

Crate: `bitop-ui-ai`.

- [x] `tool`
- [x] `confirmation`
- [x] `task`
- [x] `plan`
- [x] `chain-of-thought`
- [x] `queue`
- [x] `checkpoint`
- [x] `context`
- [x] `agent`
- [x] `artifact`
- [x] `question`
- [x] `questionnaire`

## AI: code and developer (11 of 11)

Crate: `bitop-ui-ai`.

- [x] `snippet`
- [x] `terminal`
- [x] `file-tree`
- [x] `commit`
- [x] `stack-trace`
- [x] `test-results`
- [x] `schema-display`
- [x] `environment-variables`
- [x] `package-info`
- [x] `sandbox`
- [x] `web-preview`

## AI: voice (5 of 5)

Crate: `bitop-ui-ai`.

- [x] `speech-input`
- [x] `mic-selector`
- [x] `voice-selector`
- [x] `audio-player`
- [x] `transcription`

## Parity with bitop-ui

Every component was compared with bitop-ui's docs site, side by side in a browser
(`e2e/compare.mjs`) and by using both. What differed, and is being fixed:

- [x] The Inter font is served with the library (`bitop_ui::font`), as bitop-ui bundles it.
- [x] `button`: ported again: primary, secondary, ghost, danger and link; `loading`,
  `block`, `icon_only`, and `icon_button`.
- [x] `avatar`'s initials follow bitop-ui (first two words, punctuation and particles
  skipped, "?" for none).
- [x] `voice-selector`: Enter in the search no longer submits the form around it.
- [ ] `avatar`: sizes xs to xl, square shape, `decorative`, a tint chosen from the name.
- [ ] `badge`: outline, `sm`, `dot`, `pulse`, `StatusBadge`, `StatusDot`, the ring.
- [x] `dialog`: sizes, the close button, the footer band, the place near the top, the
  blurred backdrop; `alert_dialog`'s `busy`, `error` and `tone`.
- [x] `toast`: info and warning, an action, `promise`, `close(id)`, a collapsed stack with
  `limit` and `position`, F6 to the region.
- [x] `tooltip`: `side`, `align`, `shortcut`, `delay`, and `tooltip_text`.
- [x] `popover`: title, description, `side`, opening on hover, focus moving in.
- [x] `drawer`: the footer's buttons as wide as the content, as upstream's example.
- [ ] `tabs`: pills, counts, icons, scrolling overflow, nav tabs, manual activation.
- [ ] `menu`, `context-menu`, `menubar`: item icons, 32px rows; ArrowDown on a trigger
  opens; hovering another menubar menu while one is open.
- [ ] `select`: a styled list with icons, descriptions and a check, and the field props.
- [ ] `combobox`: several values as chips, a clear button, free text, option hints and
  icons, `size`, `auto_highlight`, `limit`.
- [ ] `command`: a trigger with its shortcut, item icons, hints and shortcuts, the footer,
  matching each word with ranking.
- [ ] `checkbox` and `switch`: indeterminate, groups, descriptions, the switch's label,
  description and position, its size and off colour.
- [ ] `field` and `input`: `Fieldset`, `Form`, `FormActions`; label and error sizes, the
  error's icon; `input`'s `size` and start icon.
- [ ] `sidebar`: the icon rail when collapsed; item icons, badges, dots and descriptions;
  collapsible sections; brand, workspace switcher, user, top bar and skip link.
- [ ] `slider`: a range with two thumbs. `input-otp`: `mask` and `normalize`.
- [ ] Smaller ones: `scroll-area` orientation, `pagination` colours, `checkpoint`,
  `empty-state` icon size, `bar-chart` solid fills and spacing, `save-bar` hiding when
  the form is no longer dirty, `time`'s "—", `breadcrumbs`' "…", `model-selector`'s
  End key and badges, `filter-bar`'s custom date range, `data-table`'s search icon,
  `context` moving focus into its card, `chat-example`'s context meter.

## Known gaps in what is built

- The first 20 components came from an earlier project and were mapped onto bitop-ui's tokens.
  They are being ported again from bitop-ui's source; see "Parity with bitop-ui" below.
- There is also an `icon` component with a small set of Lucide icons, which bitop-ui gets from
  a React package.
- `menu` items have no `shortcut` hint, and there is no `MenuHeader`.
- A popup lines up with the start edge of the page, not of its own section, when a section's
  direction differs from the page's (see `direction`). Submenus follow their section.
- `accordion` and `disclosure` are `<details>`: the heading of an accordion item is inside its
  `<summary>`, and the panel's height is animated only where the browser can do it. Firefox
  opens and closes at once; Safari animates opening and closes at once.
- `pagination`'s `paginator` renders links only, not buttons for a table paged in the browser.
- `layout`'s stack and inline render a `<div>` or a `<ul>`, not any element.
- `resizable` does not size panels that are added to a group after it was laid out.
- `carousel` with several slides in view shows a dot for every slide, though the last few
  cannot come to the start edge.
- `drawer` has no snap points, and `navigation-menu` has one popup for each panel, not one
  shared popup that changes size.
- `date-picker` has no `DateRangePresets` (the toggle group that keeps a preset's ID for a
  URL); the presets beside the calendar are there.
- Toasts sit below modal dialogs, which are in the top layer. They cannot be swiped away,
  and are not named by their title through `aria-labelledby` (the Playwright Firefox
  crash); their text is their content.
- `dialog`'s `initialFocus` is an `autofocus` attribute on a control in the body;
  `finalFocus` is `final_focus`, an element's ID, which the library script focuses after
  the browser has returned focus to the trigger. `alert_dialog`'s `onConfirm` is
  `confirm_attrs` (a form submission, or a click a script hears) and its `error` a
  message; `bitopUi.dialog.setBusy` and `setError` change both in the browser. The
  alert dialog and the popover are named with `aria-label` (their title's text), for
  the Playwright Firefox crash.
- `popover` has no `pointerFocus`: a click or a key moves focus to the first control,
  hover leaves it where it is. Opened by hover, it closes 150ms after the pointer leaves
  the trigger and the popup, rather than following Base UI's safe triangle.
- `tooltip` has no `TooltipProvider`: each tooltip waits its own `delay`, also when the
  pointer moves from one trigger to the next. The tooltip cannot be hovered: it closes
  when the pointer leaves its trigger.
- The sidebar drawer on narrow screens does not trap focus.
- The charts' data table has no `onOpenChange` or controlled `open`; listen for the native
  `toggle` event. A lone point between gaps in a `sparkline` is not drawn, as upstream.
- The AI chat set (`bitop-ui-ai`) renders on the server, so a few React-only parts of
  upstream are other things here: callbacks (`onSelect`, `onSubmit`, `onStop`,
  `onBranchChange`, `onOpen`, `onFileError`) are DOM events, form submissions or a status
  line; `useConversation` is `bitopUi.conversation`; controlled `value`s are initial
  values. Each module's documentation lists its differences.
- `response` renders Markdown on the server with `pulldown-cmark`: there are no
  `components`, `remarkPlugins`, `highlight` or `LazyResponse`. Relative and same-site
  link and image URLs are refused, where upstream allows them.
- `code-block` has no syntax highlighter, as upstream bundles none; the `<code>` has a
  `language-…` class for one in the browser.
- `model-selector` and `open-in-chat` have no provider icons, and `open-in-chat` no
  custom trigger. `inline-citation`'s and `sources`' descriptions and quotes are text.
- `message` branches are switched by the script among server-rendered versions; before
  the script runs, CSS shows the chosen one only among the first ten.
- `image` turns bytes into a `data:` URL on the server rather than an object URL. Its
  loading placeholder is not shown on its gallery page, as nothing there loads slowly;
  the check reaches it.
- The agent and voice components render on the server too. `plan`'s toggle, the steps of
  a `questionnaire` and every voice part need the library script; without it a plan stays
  as rendered, a questionnaire shows its first step only, and the voice controls stay
  disabled. Callbacks (`onRestore`, `onSubmit`, `onSkip`, `onSeek`, `onPreview`,
  `onTranscriptionChange`, `onAudioRecorded`) are form submissions, DOM events, or a
  target text box; controlled values are initial values.
- `tool` and `agent` take their input, output and schemas as text the caller has
  formatted; upstream's `formatToolValue` (any value to JSON) is not ported.
- `question`'s multiple-choice group is a `<fieldset>` of the base checkboxes, as the base
  set has no checkbox group. `questionnaire`'s Edit is a ghost button (no link variant).
- `context` writes numbers the English way and money through a format string, not `Intl`;
  1,250,000 tokens round to "1.2M" where `Intl` says "1.3M".
- `mic-selector` has no `useAudioInputs` or `formatLabel`; `voice-selector` has no voice
  icons or custom trigger; `transcription` has no render function for segments.
- The context card, the plan, the confirmation, question, artifact and agent cards, and
  their sections are named with `aria-label` rather than by an element inside them,
  because of a crash in Playwright's Firefox (see the contributing guide).
- The code and developer components render on the server too. `stack-trace`,
  `terminal` and `schema-display` parse on the server (a stack, ANSI codes, a JSON
  Schema through the base set's JSON parser); callbacks (`onFrameClick`, `onClear`,
  `onSelect`, `onExpandedChange`, `onUrlChange`, `onShowValuesChange`) are DOM events or
  the native `change` and `toggle` events, and controlled values are initial values.
  Each module's documentation lists its differences.
- `terminal` streams by having the app append server-rendered spans
  (`terminal_chunk`): ANSI styles start fresh at each chunk, and a `\r` rewrites only
  lines within the chunk it arrives in.
- `web-preview`'s spinner shows only for loads the script starts, not the first one.
  There is no `useWebPreview`.
- `commit`'s and `stack-trace`'s triggers are buttons with `aria-expanded` (upstream's
  collapsible) beside other buttons; `test-results`, `sandbox` and the sections of
  `schema-display` are `<details>`. `file-tree` items are named by their row's text, not
  through `aria-labelledby`, because of the Playwright Firefox crash.
- `package-info`, `environment-variables`, `test-results` and `schema-display` show the
  base badge in upstream's small size and info colour with styles of their own, as the
  base badge has neither.
