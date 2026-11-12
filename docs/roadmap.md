# Roadmap

The target is every component in [bitop-ui](https://github.com/ncecere/bitop-ui): 120 in all.
30 are built. The base set comes first, then the AI set as its own crate.

Names are bitop-ui's. A few live together here, as they do there: `switch` is in the `checkbox`
module, `textarea` in `input`, `alert dialog` in `dialog`, and `app-shell` is `sidebar`.

Order of work: forms, overlays, navigation and layout, display and feedback, data and charts,
then AI chat, agents and tools, code, and voice.

## Base: forms (7 of 20)

Crate: `bitop-ui`.

- [x] `button`
- [ ] `button-group`
- [x] `checkbox`
- [x] `switch`
- [ ] `radio-group`
- [ ] `toggle`
- [ ] `toggle-group`
- [x] `input`
- [ ] `input-group`
- [ ] `input-otp`
- [ ] `number-input`
- [ ] `tag-input`
- [ ] `color-field`
- [x] `field`
- [x] `select`
- [x] `combobox`
- [ ] `slider`
- [ ] `calendar`
- [ ] `date-picker`
- [ ] `drop-zone`

## Base: overlays (6 of 11)

Crate: `bitop-ui`.

- [x] `dialog`
- [ ] `sheet`
- [ ] `drawer`
- [x] `popover`
- [ ] `hover-card`
- [x] `tooltip`
- [x] `menu`
- [ ] `context-menu`
- [ ] `menubar`
- [x] `command-palette`
- [x] `toast`

## Base: navigation and layout (5 of 13)

Crate: `bitop-ui`.

- [x] `app-shell`
- [ ] `navigation-menu`
- [ ] `breadcrumbs`
- [ ] `pagination`
- [x] `tabs`
- [ ] `page-header`
- [ ] `layout`
- [x] `separator`
- [x] `scroll-area`
- [ ] `resizable`
- [x] `aspect-ratio`
- [ ] `direction`
- [ ] `save-bar`

## Base: display (7 of 18)

Crate: `bitop-ui`.

- [x] `avatar`
- [x] `badge`
- [x] `card`
- [x] `alert`
- [ ] `item`
- [x] `kbd`
- [ ] `marker`
- [x] `text-link`
- [ ] `time`
- [ ] `description-list`
- [ ] `stat-card`
- [x] `empty-state`
- [ ] `checklist`
- [ ] `accordion`
- [ ] `disclosure`
- [ ] `carousel`
- [ ] `copy-button`
- [ ] `copy-field`

## Base: feedback (3 of 7)

Crate: `bitop-ui`.

- [x] `spinner`
- [ ] `loader`
- [x] `skeleton`
- [x] `progress`
- [ ] `meter`
- [ ] `query-state`
- [ ] `shimmer`

## Base: utilities (2 of 2)

Crate: `bitop-ui`.

- [x] `color-mode`
- [x] `visually-hidden`

## Data and charts (0 of 8)

Crate: `bitop-ui`.

- [ ] `table`
- [ ] `data-table`
- [ ] `filter-bar`
- [ ] `diff-viewer`
- [ ] `chart`
- [ ] `bar-chart`
- [ ] `line-chart`
- [ ] `sparkline`

## AI: chat (0 of 13)

Crate: `bitop-ui-ai`.

- [ ] `conversation`
- [ ] `message`
- [ ] `response`
- [ ] `prompt-input`
- [ ] `suggestion`
- [ ] `model-selector`
- [ ] `attachments`
- [ ] `reasoning`
- [ ] `sources`
- [ ] `inline-citation`
- [ ] `code-block`
- [ ] `image`
- [ ] `open-in-chat`

## AI: agents and tools (0 of 12)

Crate: `bitop-ui-ai`.

- [ ] `tool`
- [ ] `confirmation`
- [ ] `task`
- [ ] `plan`
- [ ] `chain-of-thought`
- [ ] `queue`
- [ ] `checkpoint`
- [ ] `context`
- [ ] `agent`
- [ ] `artifact`
- [ ] `question`
- [ ] `questionnaire`

## AI: code and developer (0 of 11)

Crate: `bitop-ui-ai`.

- [ ] `snippet`
- [ ] `terminal`
- [ ] `file-tree`
- [ ] `commit`
- [ ] `stack-trace`
- [ ] `test-results`
- [ ] `schema-display`
- [ ] `environment-variables`
- [ ] `package-info`
- [ ] `sandbox`
- [ ] `web-preview`

## AI: voice (0 of 5)

Crate: `bitop-ui-ai`.

- [ ] `speech-input`
- [ ] `mic-selector`
- [ ] `voice-selector`
- [ ] `audio-player`
- [ ] `transcription`

## Known gaps in what is built

- The first 20 components came from an earlier project and were mapped onto bitop-ui's tokens.
  Their look has not yet been compared part by part with bitop-ui's.
- There is also an `icon` component with a small set of Lucide icons, which bitop-ui gets from
  a React package.
- `alert` has no `ErrorAlert` yet (the mapping from an HTTP status to a title and message).
- `color-mode` has the script and `bitopUi.setColorMode()`, but no toggle component.
- Toasts sit below modal dialogs.
- The sidebar drawer on narrow screens does not trap focus.
