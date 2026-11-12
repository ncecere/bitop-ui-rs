mod c;
mod healthz;
mod kit;

use bitop_ui::components::{
    select::select,
    sidebar::{
        SIDEBAR_COOKIE, sidebar, sidebar_content, sidebar_group, sidebar_header, sidebar_item,
        sidebar_layout, sidebar_main, sidebar_toggle,
    },
    toast::toast_region,
};
use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt, asset},
    context::Cx,
    cookie::{CookieLayer, Cookies, cookies},
    router::{RouterBuilder, RouterBuilderDiscoverExt, Slot, layout, module_router, page},
    runtime::RuntimeLayer,
    view::{View, attributes, element_id, view},
};

/// One gallery page: the route under `/c/`, the title, and what the component is for.
pub struct Entry {
    pub slug: &'static str,
    pub href: &'static str,
    pub title: &'static str,
    pub summary: &'static str,
}

/// A named set of components, shown as one group in the sidebar.
pub struct Group {
    pub title: &'static str,
    pub entries: &'static [Entry],
}

macro_rules! entry {
    ($slug:literal, $title:literal, $summary:literal) => {
        Entry {
            slug: $slug,
            href: concat!("/c/", $slug),
            title: $title,
            summary: $summary,
        }
    };
}

/// Every page in the gallery. Add a component here and in `app/c.rs`.
pub const GROUPS: &[Group] = &[
    Group {
        title: "Forms",
        entries: &[
            entry!("button", "Button", "Starts an action."),
            entry!(
                "checkbox",
                "Checkbox and switch",
                "Turn one option on or off."
            ),
            entry!(
                "input",
                "Input and field",
                "Text inputs with a label, help text and errors."
            ),
            entry!("select", "Select", "Choose one value from a short list."),
            entry!(
                "combobox",
                "Combobox",
                "Choose one value from a list you can filter."
            ),
            entry!(
                "button-group",
                "Button group",
                "Related buttons joined into one unit."
            ),
            entry!(
                "radio-group",
                "Radio group",
                "Choose one option from a few."
            ),
            entry!("toggle", "Toggle", "A button that stays pressed."),
            entry!(
                "toggle-group",
                "Toggle group",
                "Toggles that share their pressed state."
            ),
            entry!(
                "input-group",
                "Input group",
                "An input with text, icons or buttons inside it."
            ),
            entry!(
                "number-input",
                "Number input",
                "An amount, with grouping and step buttons."
            ),
            entry!(
                "input-otp",
                "Input OTP",
                "A one-time code, one character per box."
            ),
            entry!(
                "slider",
                "Slider",
                "Choose a value from a range by dragging."
            ),
            entry!(
                "color-field",
                "Color field",
                "A hex colour with a picker and a contrast check."
            ),
            entry!(
                "copy-button",
                "Copy button",
                "Copies text to the clipboard."
            ),
            entry!(
                "copy-field",
                "Copy field",
                "A read-only value with a copy button."
            ),
            entry!(
                "tag-input",
                "Tag input",
                "A list of short text tags with an inline input."
            ),
            entry!(
                "calendar",
                "Calendar",
                "A month grid for a day, several days or a range."
            ),
            entry!(
                "date-picker",
                "Date picker",
                "A button that opens a calendar in a popover."
            ),
            entry!(
                "drop-zone",
                "Drop zone",
                "Drop files, or choose them with a button."
            ),
        ],
    },
    Group {
        title: "Overlays",
        entries: &[
            entry!(
                "dialog",
                "Dialog",
                "A window over the page, and its alert variant."
            ),
            entry!("popover", "Popover", "A small panel anchored to a button."),
            entry!("tooltip", "Tooltip", "A hint shown on hover or focus."),
            entry!(
                "menu",
                "Menu",
                "A list of actions that opens from a button."
            ),
            entry!(
                "command",
                "Command palette",
                "Search actions and pages from the keyboard."
            ),
            entry!("toast", "Toast", "A brief message that dismisses itself."),
            entry!(
                "sheet",
                "Sheet",
                "A dialog attached to an edge of the screen."
            ),
            entry!("drawer", "Drawer", "An edge panel that can be swiped away."),
            entry!(
                "hover-card",
                "Hover card",
                "A preview of a link, shown on hover or focus."
            ),
            entry!(
                "context-menu",
                "Context menu",
                "A menu that opens at the pointer on right-click."
            ),
            entry!(
                "menubar",
                "Menubar",
                "A row of menus, as in a desktop application."
            ),
        ],
    },
    Group {
        title: "Navigation and layout",
        entries: &[
            entry!(
                "sidebar",
                "Sidebar",
                "The application shell with a navigation sidebar."
            ),
            entry!("tabs", "Tabs", "Switch between panels on one page."),
            entry!(
                "navigation-menu",
                "Navigation menu",
                "Site navigation with panels of links."
            ),
            entry!(
                "separator",
                "Separator",
                "A line between sections or items."
            ),
            entry!(
                "scroll-area",
                "Scroll area",
                "A region that scrolls inside the page."
            ),
            entry!(
                "aspect-ratio",
                "Aspect ratio",
                "A box that keeps its proportions."
            ),
            entry!(
                "breadcrumbs",
                "Breadcrumbs",
                "The pages above the current one."
            ),
            entry!(
                "pagination",
                "Pagination",
                "Links to the pages of a long list."
            ),
            entry!(
                "page-header",
                "Page header",
                "The title, description and actions of a page."
            ),
            entry!(
                "layout",
                "Layout",
                "Stack, inline and container: spacing from the scale."
            ),
            entry!(
                "resizable",
                "Resizable",
                "Panels with handles that resize them."
            ),
            entry!(
                "direction",
                "Direction",
                "A section with its own reading direction."
            ),
            entry!(
                "save-bar",
                "Save bar",
                "A form's actions, in reach while it has unsaved changes."
            ),
            entry!(
                "color-mode",
                "Color mode",
                "Switch between light, dark and the system setting."
            ),
        ],
    },
    Group {
        title: "Display",
        entries: &[
            entry!("avatar", "Avatar", "A person's picture, or their initials."),
            entry!("badge", "Badge", "A short label for a status or a count."),
            entry!("card", "Card", "A surface that groups related content."),
            entry!("alert", "Alert", "An inline message about the page."),
            entry!("kbd", "Kbd", "A key cap, and a key combination."),
            entry!("text-link", "Text link", "A link in running text."),
            entry!(
                "empty-state",
                "Empty state",
                "What to show when there is nothing yet."
            ),
            entry!(
                "accordion",
                "Accordion",
                "Headings that each show and hide a panel."
            ),
            entry!(
                "disclosure",
                "Disclosure",
                "A button that shows and hides a panel."
            ),
            entry!(
                "carousel",
                "Carousel",
                "Slides on a track, with buttons and dots."
            ),
            entry!("icon", "Icon", "The small icon set the components use."),
            entry!(
                "item",
                "Item",
                "A row with media, a title, a description and actions."
            ),
            entry!(
                "marker",
                "Marker",
                "An inline note in a conversation or an activity feed."
            ),
            entry!(
                "time",
                "Time",
                "A date or a relative time, in the visitor's locale."
            ),
            entry!(
                "description-list",
                "Description list",
                "Labels and values, and a line of short facts."
            ),
            entry!(
                "stat-card",
                "Stat card",
                "A metric tile with a change indicator."
            ),
            entry!(
                "checklist",
                "Checklist",
                "Onboarding steps with done states."
            ),
        ],
    },
    Group {
        title: "Feedback",
        entries: &[
            entry!("spinner", "Spinner", "A sign that work is in progress."),
            entry!(
                "skeleton",
                "Skeleton",
                "A placeholder for content that is loading."
            ),
            entry!("progress", "Progress", "How far a task has got."),
            entry!(
                "loader",
                "Loader",
                "Three pulsing dots for a pending reply."
            ),
            entry!("meter", "Meter", "How much of a limit is used."),
            entry!(
                "query-state",
                "Query state",
                "Loading, error, empty or the content."
            ),
            entry!(
                "shimmer",
                "Shimmer",
                "Text with a light sweeping across it."
            ),
        ],
    },
    Group {
        title: "Data and charts",
        entries: &[
            entry!(
                "table",
                "Table",
                "Rows and columns, with a caption and real headers."
            ),
            entry!(
                "data-table",
                "Data table",
                "A table with sorting, selection, a columns menu and paging."
            ),
            entry!(
                "filter-bar",
                "Filter bar",
                "Faceted filters with a chip for each active filter."
            ),
            entry!(
                "diff-viewer",
                "Diff viewer",
                "Before and after, for text and JSON."
            ),
            entry!(
                "chart",
                "Chart",
                "Series colours and patterns, the legend and the data table that charts share."
            ),
            entry!(
                "bar-chart",
                "Bar chart",
                "Bars per point, overlapping or stacked, with a readout and a data table."
            ),
            entry!(
                "line-chart",
                "Line chart",
                "Lines or areas over ordered points, with gaps for missing data."
            ),
            entry!(
                "sparkline",
                "Sparkline",
                "A tiny inline trend for stat cards and table cells."
            ),
        ],
    },
];

pub fn router() -> RouterBuilder {
    module_router!()
        .discover()
        .assets(AssetBundle::load().unwrap())
        .layer(CookieLayer::new())
        .layer(RuntimeLayer::new())
}

#[layout]
async fn root(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    let collapsed = cookies(cx)
        .get(SIDEBAR_COOKIE)
        .is_some_and(|cookie| cookie.value() == "collapsed");
    let sidebar_id = element_id(cx);

    Ok(view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"bitop-ui gallery"</title>
                bitop_ui::head()
                bitop_ui_ai::head()
                <script type="module" src=(asset!("./client/gallery.js"))></script>
                topcoat::runtime::script()
                topcoat::dev::script()
            </head>
            <body>
                sidebar_layout(
                    collapsed: collapsed,
                    sidebar(
                        id: &sidebar_id,
                        label: "Components",
                        sidebar_header(
                            <a href="/" style="color: inherit; text-decoration: none">
                                "bitop-ui"
                            </a>
                        )
                        sidebar_content(
                            for group in GROUPS {
                                sidebar_group(
                                    label: group.title,
                                    for entry in group.entries {
                                        sidebar_item(href: entry.href, (entry.title))
                                    }
                                )
                            }
                        )
                    )
                    sidebar_main(
                        <header
                            style="display: flex; flex-wrap: wrap; align-items: center; gap: var(--space-2); padding: var(--space-2) var(--space-4); border-block-end: 1px solid var(--color-border)"
                        >
                            sidebar_toggle(target: &sidebar_id)
                            <span style="flex: 1"></span>
                            select(
                                attrs: attributes! { aria-label="Theme" data-gallery-set="theme" },
                                <option value="system">"System theme"</option>
                                <option value="light">"Light"</option>
                                <option value="dark">"Dark"</option>
                            )
                            select(
                                attrs: attributes! { aria-label="Text direction" data-gallery-set="dir" },
                                <option value="ltr">"Left to right"</option>
                                <option value="rtl">"Right to left"</option>
                            )
                        </header>
                        <main
                            style="inline-size: 100%; max-inline-size: 52rem; margin-inline: auto; padding: var(--space-8)"
                        >
                            (slot)
                        </main>
                    )
                )
                toast_region()
            </body>
        </html>
    })
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <h1>"bitop-ui gallery"</h1>
        <p
            style="margin-block: var(--space-2) var(--space-6); color: var(--color-text-muted)"
        >
            "Every component on its own page, in every state."
        </p>
        for group in GROUPS {
            <h2
                style="margin-block: var(--space-6) var(--space-3); font-size: var(--font-size-lg)"
            >
                (group.title)
            </h2>
            <ul
                style="display: grid; grid-template-columns: repeat(auto-fill, minmax(14rem, 1fr)); gap: var(--space-3); padding: 0; list-style: none"
            >
                for entry in group.entries {
                    <li>
                        <a
                            href=(entry.href)
                            style="display: block; block-size: 100%; padding: var(--space-4); border: 1px solid var(--color-border); border-radius: var(--radius-card); background: var(--color-surface); color: inherit; text-decoration: none"
                        >
                            <strong>(entry.title)</strong>
                            <span
                                style="display: block; margin-block-start: var(--space-1); color: var(--color-text-muted); font-size: var(--font-size-sm)"
                            >
                                (entry.summary)
                            </span>
                        </a>
                    </li>
                }
            </ul>
        }
    })
}
