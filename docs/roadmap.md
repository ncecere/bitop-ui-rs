# Roadmap

The target is every component in [bitop-ui](https://github.com/ncecere/bitop-ui): 120 in all.
51 are built. The base set comes first, then the AI set as its own crate.

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

## Base: navigation and layout (6 of 13)

Crate: `bitop-ui`.

- [x] `app-shell`
- [x] `navigation-menu`
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

## Base: display (9 of 18)

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
- [x] `copy-button`
- [x] `copy-field`

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
- `menu` has no checkbox items, radio items, groups or submenus yet, so `context-menu` and
  `menubar`, which share its items, have none either.
- `drawer` has no snap points, and `navigation-menu` has one popup for each panel, not one
  shared popup that changes size.
- `date-picker` has no `DateRangePresets` (the toggle group that keeps a preset's ID for a
  URL); the presets beside the calendar are there.
- Toasts sit below modal dialogs.
- The sidebar drawer on narrow screens does not trap focus.
