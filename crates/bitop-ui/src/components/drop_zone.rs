//! A drag-and-drop area around a real file input, with a button that opens the file
//! picker.
//!
//! The button makes it work with the keyboard, a switch and a screen reader; dropping
//! is a pointer-only extra. The area is a group named by its label, and the button is
//! described by the hint text. The files are held by an `<input type="file">`, so they are
//! submitted with the surrounding form under `name`; the form needs
//! `enctype="multipart/form-data"`.
//!
//! The library script checks chosen and dropped files alike against `accept`,
//! `max_size` and `max_files` (the picker's own filter is only a hint, and a drop
//! ignores it). Files that fail are taken out of the input and named in a polite
//! status line, such as "setup.exe isn't an accepted file type."; the rest are listed
//! under the button. Always check again on the server.
//!
//! The script fires `bitop:files` on the group when the files change, with
//! `detail.files` (the accepted `File`s) and `detail.rejections` (`{ file, reason }`,
//! where the reason is "type", "size" or "count").
//!
//! Differences from upstream: the group is named with `aria-label`, not with
//! `aria-labelledby` pointing at its own label. `onFiles` and `onReject` are that event. The chosen
//! files are listed by the component, since there is no caller code to do it.
//! `max_files` is new here. `busy` disables the button and shows a spinner in it.
//!
//! ```ignore
//! view! {
//!     <form method="post" enctype="multipart/form-data">
//!         drop_zone(id: &id, name: "documents", accept: ".pdf,image/*",
//!             max_size: 5 * 1024 * 1024, description: "PDF or images, up to 5 MB each.")
//!     </form>
//! }
//! ```

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

use super::{
    icon::{Icon, icon},
    spinner::{SpinnerSize, spinner},
};
use crate::css::{button as button_css, drop_zone as css};

/// A size in bytes as text, such as `5 MB`; one decimal at most.
#[must_use]
pub fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        return format!("{bytes} B");
    }
    let units = ["KB", "MB", "GB"];
    #[allow(clippy::cast_precision_loss)]
    let mut value = bytes as f64 / 1024.0;
    let mut unit = 0;
    while value >= 1024.0 && unit < units.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    let rounded = (value * 10.0).round() / 10.0;
    format!("{rounded} {}", units[unit])
}

/// A drop zone.
///
/// - `id` goes on the button; `name` on the file input.
/// - `label` is the group's heading and `button_label` the button's text.
/// - `description` is hint text, such as the accepted types and limits.
/// - `accept` is a list of extensions and MIME types, such as `".pdf,image/*"`.
/// - `max_size` is the largest accepted file in bytes, and `max_files` the most files;
///   0 means no limit.
/// - `single` accepts one file at a time.
/// - `busy` says an upload is running.
/// - `files_label` names the list of chosen files.
/// - The messages are templates: `{name}` is replaced with the file's name, `{size}`
///   with `max_size` and `{max}` with `max_files`. `single_message` is used in place of
///   `count_message` when `single` is set.
/// - `attrs` are forwarded to the group; `child` is rendered after the list of files.
#[component]
pub async fn drop_zone(
    cx: &Cx,
    id: &str,
    name: &str,
    #[default("Drag and drop files here, or")] label: &str,
    #[default("Choose files")] button_label: &str,
    #[default] description: &str,
    #[default] accept: &str,
    #[default] max_size: u64,
    #[default] max_files: usize,
    #[default] single: bool,
    #[default] disabled: bool,
    #[default] busy: bool,
    #[default(Icon::Upload)] icon_name: Icon,
    #[default("Chosen files")] files_label: &str,
    #[default("{name} isn't an accepted file type.")] type_message: &str,
    #[default("{name} is larger than {size}.")] size_message: &str,
    #[default("{name} was left out: no more than {max} files.")] count_message: &str,
    #[default("{name} was left out: one file only.")] single_message: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let description_id = format!("{id}-description");
    let inactive = disabled || busy;
    let size_message = size_message.replace("{size}", &format_size(max_size));
    let count_message = if single {
        single_message.to_owned()
    } else {
        count_message.replace("{max}", &max_files.to_string())
    };
    Ok(view! {
        <div
            role="group"
            aria-label=(label)
            data-drop-zone=""
            data-disabled=(disabled)
            data-max-size=((max_size > 0).then(|| max_size.to_string()))
            data-max-files=((max_files > 0).then(|| max_files.to_string()))
            data-type-message=(type_message)
            data-size-message=(&size_message)
            data-count-message=(&count_message)
            class=(class!(css::ZONE, attrs.remove("class")))
            (attrs)
        >
            <span aria-hidden="true" class=(css::ICON)>icon(name: icon_name)</span>
            <p class=(css::LABEL)>(label)</p>
            <button
                type="button"
                id=(id)
                class=(button_css::ROOT)
                data-variant="secondary"
                data-size="md"
                data-drop-zone-button=""
                aria-describedby=((!description.is_empty()).then_some(&description_id))
                aria-busy=(busy.then_some("true"))
                disabled=(inactive)
            >
                if busy {
                    spinner(size: SpinnerSize::Sm)
                }
                (button_label)
            </button>
            <input
                type="file"
                name=(name)
                hidden=(true)
                tabindex="-1"
                multiple=(!single)
                accept=((!accept.is_empty()).then_some(accept))
                disabled=(inactive)
                aria-label=(button_label)
            >
            if !description.is_empty() {
                <p id=(&description_id) class=(css::DESCRIPTION)>(description)</p>
            }
            // Always rendered: a live region must exist before its text changes.
            <p role="status" class=(css::REJECTED) data-drop-zone-rejected=""></p>
            <ul class=(css::FILES) aria-label=(files_label) data-drop-zone-files=""></ul>
            (child)
        </div>
    })
}

#[cfg(test)]
mod tests {
    use super::format_size;

    #[test]
    fn formats_sizes() {
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1024), "1 KB");
        assert_eq!(format_size(1536), "1.5 KB");
        assert_eq!(format_size(5 * 1024 * 1024), "5 MB");
        assert_eq!(format_size(3 * 1024 * 1024 * 1024 * 1024), "3072 GB");
    }
}
