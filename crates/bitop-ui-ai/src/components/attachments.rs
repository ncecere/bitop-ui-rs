//! File chips for a composer or a sent message: an image thumbnail when there is a
//! preview URL, the type and size, the upload state and an optional remove button.
//!
//! The upload state is spelled out ("Uploading…", "Upload failed") for assistive
//! technology, not only shown as a spinner or a colour.
//!
//! The remove button has two forms. With `remove_name`, it is a submit button that
//! sends `remove_name=remove_value` with the form around it, so that the server can drop
//! the file. Without it, the library script removes the chip, moves focus to the next
//! chip's remove button (or the previous one, or the element whose ID is in
//! `data-attachments-focus` on the list), and sends a `bitop:attachment-remove` event
//! from the list with the chip's `data-attachment-id` in `detail.id`. Cancel the event
//! to keep the chip.
//!
//! [`prompt_input`](super::prompt_input::prompt_input) renders these for the files
//! chosen in the browser.
//!
//! ```ignore
//! view! {
//!     attachments(
//!         attachment(name: "report.pdf", size: 482_133, mime_type: "application/pdf",
//!             removable: true)
//!         attachment(name: "chart.png", mime_type: "image/png", url: "/files/chart.png")
//!     )
//! }
//! ```
//!
//! Differences from upstream: `onRemove` is the event or the submit button above.
//! Each part takes the file's fields as arguments, not an object. A remove button is
//! shown disabled with `disabled`, where upstream hides it (its roadmap item L2).

use bitop_ui::{
    components::{
        icon::{Icon, icon},
        spinner::{SpinnerSize, spinner},
    },
    css::button as button_css,
};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::css::attachments as css;

/// A file size in bytes as text: `1.5 KB`, `12 MB`. One decimal under ten units,
/// whole numbers from ten.
///
/// ```
/// use bitop_ui_ai::components::attachments::format_bytes;
///
/// assert_eq!(format_bytes(1536), "1.5 KB");
/// assert_eq!(format_bytes(12 * 1024 * 1024), "12 MB");
/// ```
#[must_use]
pub fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        return format!("{bytes} B");
    }
    let units = ["KB", "MB", "GB", "TB"];
    #[allow(clippy::cast_precision_loss)]
    let mut value = bytes as f64 / 1024.0;
    let mut unit = 0;
    while value >= 1024.0 && unit < units.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    let shown = if value >= 10.0 {
        value.round()
    } else {
        (value * 10.0).round() / 10.0
    };
    format!("{shown} {}", units[unit])
}

/// The extension of a file name in capitals, such as `PDF`; none for a name without
/// one or a name that starts with its only dot.
#[must_use]
pub fn file_extension(name: &str) -> Option<String> {
    match name.rfind('.') {
        Some(dot) if dot > 0 && dot + 1 < name.len() => Some(name[dot + 1..].to_uppercase()),
        _ => None,
    }
}

/// The icon for a MIME type: a picture for images, a page of text for text and
/// documents, and a plain file for anything else.
#[must_use]
pub fn file_icon(mime_type: &str) -> Icon {
    if mime_type.starts_with("image/") {
        Icon::Image
    } else if mime_type.starts_with("text/")
        || ["pdf", "word", "document"]
            .iter()
            .any(|kind| mime_type.contains(kind))
    {
        Icon::FileText
    } else {
        Icon::File
    }
}

/// The upload state of an [`attachment`]. Rendered as `data-status`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AttachmentStatus {
    #[default]
    Ready,
    Uploading,
    Error,
}

impl AttachmentStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Uploading => "uploading",
            Self::Error => "error",
        }
    }
}

/// A list of [`attachment`]s, named by `label`. `attrs` are forwarded to the `<ul>`.
#[component]
pub async fn attachments(
    cx: &Cx,
    #[default("Attachments")] label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <ul
            aria-label=(label)
            data-attachments=""
            class=(class!(css::LIST, attrs.remove("class")))
            (attrs)
        >
            (child)
        </ul>
    })
}

/// One attached file.
///
/// - `name` is the file name; `size` its size in bytes; `mime_type` its type, such
///   as `image/png`, which picks the icon.
/// - `url` is a preview for an image, shown as a thumbnail.
/// - `status` is the upload state.
/// - `id` identifies the file in the `bitop:attachment-remove` event, as
///   `data-attachment-id`.
/// - `removable` adds the remove button; `remove_name` and `remove_value` make it a
///   submit button. `disabled` disables it.
/// - `icon_name` replaces the icon for the type.
/// - `uploading_label`, `error_label`, `remove_label` (with `{name}` for the file
///   name) and `remove_tooltip` are the strings.
/// - `attrs` are forwarded to the `<li>`.
#[component]
pub async fn attachment(
    cx: &Cx,
    name: &str,
    #[default]
    #[into]
    size: Option<u64>,
    #[default] mime_type: &str,
    #[default] url: &str,
    #[default] status: AttachmentStatus,
    #[default] id: &str,
    #[default] removable: bool,
    #[default] remove_name: &str,
    #[default] remove_value: &str,
    #[default] disabled: bool,
    #[default]
    #[into]
    icon_name: Option<Icon>,
    #[default("Uploading…")] uploading_label: &str,
    #[default("Upload failed")] error_label: &str,
    #[default("Remove {name}")] remove_label: &str,
    #[default("Remove")] remove_tooltip: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let image = mime_type.starts_with("image/") && !url.is_empty();
    let meta = match status {
        AttachmentStatus::Uploading => uploading_label.to_owned(),
        AttachmentStatus::Error => error_label.to_owned(),
        AttachmentStatus::Ready => [file_extension(name), size.map(format_bytes)]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join(" · "),
    };
    let type_icon = icon_name.unwrap_or_else(|| file_icon(mime_type));
    let submits = !remove_name.is_empty();
    Ok(view! {
        <li
            data-status=(status.as_str())
            data-attachment=""
            data-attachment-id=((!id.is_empty()).then_some(id))
            class=(class!(css::ATTACHMENT, attrs.remove("class")))
            (attrs)
        >
            <span aria-hidden="true" class=(css::PREVIEW) data-attachment-preview="">
                if status == AttachmentStatus::Uploading {
                    spinner(size: SpinnerSize::Sm)
                } else if image {
                    <img src=(url) alt="" class=(css::IMAGE)>
                } else {
                    icon(name: type_icon)
                }
            </span>
            <span class=(css::TEXT)>
                <span class=(css::NAME) title=(name) data-attachment-name="">
                    (name)
                </span>
                <span class=(css::META) data-attachment-meta="">(meta)</span>
            </span>
            if removable {
                <button
                    type=(if submits { "submit" } else { "button" })
                    name=(submits.then_some(remove_name))
                    value=(submits.then_some(remove_value))
                    aria-label=(remove_label.replace("{name}", name))
                    title=(remove_tooltip)
                    disabled=(disabled)
                    data-attachment-remove=(if submits { "submit" } else { "" })
                    data-variant="ghost"
                    data-size="icon"
                    class=(class!(button_css::ROOT, css::REMOVE))
                >
                    icon(name: Icon::X, attrs: attributes! { class=(css::REMOVE_ICON) })
                </button>
            }
        </li>
    })
}

#[cfg(test)]
mod tests {
    use super::{file_extension, file_icon, format_bytes};
    use bitop_ui::components::icon::Icon;

    #[test]
    fn sizes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1023), "1023 B");
        assert_eq!(format_bytes(1024), "1 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(482_133), "471 KB");
        assert_eq!(format_bytes(5 * 1024 * 1024 + 300 * 1024), "5.3 MB");
        assert_eq!(format_bytes(2048 * 1024 * 1024 * 1024 * 1024), "2048 TB");
    }

    #[test]
    fn extensions_and_icons() {
        assert_eq!(file_extension("report.final.pdf").as_deref(), Some("PDF"));
        assert_eq!(file_extension(".env"), None);
        assert_eq!(file_extension("README"), None);
        assert_eq!(file_extension("trailing."), None);
        assert_eq!(file_icon("image/png"), Icon::Image);
        assert_eq!(file_icon("application/pdf"), Icon::FileText);
        assert_eq!(file_icon("text/markdown"), Icon::FileText);
        assert_eq!(file_icon("application/zip"), Icon::File);
    }
}
