//! Before and after, for text and JSON: an audit entry, or two versions of an
//! agent's settings.
//!
//! ```ignore
//! view! {
//!     diff_viewer(label: "System prompt", before: &v1.prompt, after: &v2.prompt)
//!     diff_viewer(label: "Changes to the agent", format: DiffFormat::Json,
//!         before: &entry.before, after: &entry.after, mode: DiffMode::Split)
//! }
//! ```
//!
//! - Text: a line diff ([`diff_text`], Myers' algorithm). Runs of unchanged lines
//!   longer than twice `context_lines` are folded into a "Show N unchanged lines"
//!   button.
//! - JSON: a structural diff of the two values ([`diff_json`]): one row for each
//!   added, removed or changed key, with its path (`limits.maxPages`, `tags[2]`) and
//!   the old and new values.
//! - The layout is unified (one column, old then new) or split (before beside
//!   after), switched with the built-in toggle.
//!
//! Changes are never shown by colour alone: every changed line has a visible + or −
//! (or ~ for a changed key) and a visually hidden "Added", "Removed" or "Changed",
//! and the summary above says how much changed. The diff is a table named by
//! `label`, in a scroll area, so long values scroll and do not stretch the page.
//!
//! The diff is computed on the server. Both layouts are rendered and the library
//! script switches between them and unfolds unchanged lines, so neither needs a
//! request.
//!
//! Differences from upstream: the values are strings (JSON is parsed here, by a
//! small parser, since the library has no JSON dependency). The strings that
//! upstream builds with functions are templates in [`DiffViewerLabels`], with one
//! form for a count of one and one for the rest. Without a script the layout stays
//! as rendered and folded lines stay folded. The scroll area is always a focusable
//! region, also when the diff is short.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::{
    scroll_area::{ScrollOrientation, scroll_area},
    toggle::{ToggleSize, ToggleVariant},
    toggle_group::{toggle_group, toggle_group_item},
};
use crate::{css::diff_viewer as css, format::format_number};

/// What a line of a text diff is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiffKind {
    Equal,
    Add,
    Remove,
}

impl DiffKind {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Equal => "equal",
            Self::Add => "add",
            Self::Remove => "remove",
        }
    }
}

/// One line of a text diff.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiffLine {
    /// Whether the line is in both texts, only the new one, or only the old one.
    pub kind: DiffKind,
    /// The line, without its line ending.
    pub text: String,
    /// The line's number in the old text, from 1, for equal and removed lines.
    pub old_number: Option<usize>,
    /// The line's number in the new text, from 1, for equal and added lines.
    pub new_number: Option<usize>,
}

/// A line diff of two texts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextDiff {
    /// Every line of both texts, in order.
    pub lines: Vec<DiffLine>,
    /// How many lines were added.
    pub added: usize,
    /// How many lines were removed.
    pub removed: usize,
    /// The texts were too different to align within the budget: every old line is
    /// shown removed and every new line added.
    pub approximate: bool,
}

/// The lines of a text. `\r\n` and `\r` end a line as `\n` does, and a final line
/// ending does not add an empty last line.
#[must_use]
pub fn split_lines(text: &str) -> Vec<&str> {
    if text.is_empty() {
        return Vec::new();
    }
    let mut lines = Vec::new();
    let mut rest = text;
    while let Some(at) = rest.find(['\n', '\r']) {
        lines.push(&rest[..at]);
        let crlf = rest[at..].starts_with("\r\n");
        rest = &rest[at + if crlf { 2 } else { 1 }..];
    }
    if !rest.is_empty() {
        lines.push(rest);
    }
    lines
}

/// The edit script from `a` to `b` by Myers' O(ND) algorithm, or `None` when it
/// needs more than `max_edits` edits.
fn myers(a: &[&str], b: &[&str], max_edits: usize) -> Option<Vec<DiffKind>> {
    let n = a.len();
    let m = b.len();
    if n == 0 {
        return Some(vec![DiffKind::Add; m]);
    }
    if m == 0 {
        return Some(vec![DiffKind::Remove; n]);
    }
    let max = n + m;
    let offset = max + 1;
    // v[offset + k] is the furthest x reached on diagonal k.
    let mut v = vec![0usize; 2 * max + 3];
    // trace[d] holds v for k in -d..=d after step d, at index k + d.
    let mut trace: Vec<Vec<usize>> = Vec::new();
    let at = |index: usize, k: isize| index.wrapping_add_signed(k);
    let mut last = None;
    'outer: for d in 0..=max {
        if d > max_edits {
            return None;
        }
        let d_signed = isize::try_from(d).ok()?;
        let mut k = -d_signed;
        while k <= d_signed {
            let down =
                k == -d_signed || (k != d_signed && v[at(offset, k - 1)] < v[at(offset, k + 1)]);
            let mut x = if down {
                v[at(offset, k + 1)]
            } else {
                v[at(offset, k - 1)] + 1
            };
            let mut y = x.wrapping_add_signed(-k);
            while x < n && y < m && a[x] == b[y] {
                x += 1;
                y += 1;
            }
            v[at(offset, k)] = x;
            if x >= n && y >= m {
                trace.push(v[offset - d..=offset + d].to_vec());
                last = Some(d);
                break 'outer;
            }
            k += 2;
        }
        trace.push(v[offset - d..=offset + d].to_vec());
    }
    let last = last?;
    let mut ops = Vec::new();
    let mut x = n;
    let mut y = m;
    for d in (1..=last).rev() {
        let previous = &trace[d - 1];
        let d_signed = isize::try_from(d).ok()?;
        // previous holds diagonals -(d-1)..=(d-1) at index k + d - 1.
        let reached = |k: isize| previous[usize::try_from(k + d_signed - 1).unwrap_or(0)];
        let k = isize::try_from(x).ok()? - isize::try_from(y).ok()?;
        let down = k == -d_signed || (k != d_signed && reached(k - 1) < reached(k + 1));
        let previous_k = if down { k + 1 } else { k - 1 };
        let previous_x = reached(previous_k);
        let previous_y = previous_x.wrapping_add_signed(-previous_k);
        while x > previous_x && y > previous_y {
            ops.push(DiffKind::Equal);
            x -= 1;
            y -= 1;
        }
        ops.push(if down {
            DiffKind::Add
        } else {
            DiffKind::Remove
        });
        x = previous_x;
        y = previous_y;
    }
    while x > 0 && y > 0 {
        ops.push(DiffKind::Equal);
        x -= 1;
        y -= 1;
    }
    ops.reverse();
    Some(ops)
}

/// A line diff of `before` and `after`: Myers' algorithm, after the lines the two
/// share at the start and at the end are set aside.
///
/// `max_edits` bounds the work for very different texts; upstream's default is 4000.
/// Past it the result is "remove everything, add everything", marked `approximate`.
#[must_use]
pub fn diff_text(before: &str, after: &str, max_edits: usize) -> TextDiff {
    let a = split_lines(before);
    let b = split_lines(after);
    let mut start = 0;
    while start < a.len() && start < b.len() && a[start] == b[start] {
        start += 1;
    }
    let mut end_a = a.len();
    let mut end_b = b.len();
    while end_a > start && end_b > start && a[end_a - 1] == b[end_b - 1] {
        end_a -= 1;
        end_b -= 1;
    }
    let middle_a = &a[start..end_a];
    let middle_b = &b[start..end_b];
    let script = myers(middle_a, middle_b, max_edits);
    let approximate = script.is_none();
    let ops = script.unwrap_or_else(|| {
        let mut ops = vec![DiffKind::Remove; middle_a.len()];
        ops.extend(vec![DiffKind::Add; middle_b.len()]);
        ops
    });

    let mut lines = Vec::with_capacity(a.len() + middle_b.len());
    let mut old_number = 0;
    let mut new_number = 0;
    let mut push = |kind: DiffKind, text: &str| {
        let old = (kind != DiffKind::Add).then(|| {
            old_number += 1;
            old_number
        });
        let new = (kind != DiffKind::Remove).then(|| {
            new_number += 1;
            new_number
        });
        lines.push(DiffLine {
            kind,
            text: text.to_owned(),
            old_number: old,
            new_number: new,
        });
    };
    for line in &a[..start] {
        push(DiffKind::Equal, line);
    }
    let mut index_a = 0;
    let mut index_b = 0;
    for op in ops {
        match op {
            DiffKind::Equal => {
                push(DiffKind::Equal, middle_a[index_a]);
                index_a += 1;
                index_b += 1;
            }
            DiffKind::Remove => {
                push(DiffKind::Remove, middle_a[index_a]);
                index_a += 1;
            }
            DiffKind::Add => {
                push(DiffKind::Add, middle_b[index_b]);
                index_b += 1;
            }
        }
    }
    for line in &a[end_a..] {
        push(DiffKind::Equal, line);
    }
    TextDiff {
        added: lines
            .iter()
            .filter(|line| line.kind == DiffKind::Add)
            .count(),
        removed: lines
            .iter()
            .filter(|line| line.kind == DiffKind::Remove)
            .count(),
        lines,
        approximate,
    }
}

/// A run of lines of a text diff: shown, or folded away.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiffBlock<'a> {
    /// Lines that are shown.
    Lines(&'a [DiffLine]),
    /// Unchanged lines far from any change, behind a "Show N unchanged lines" button.
    Fold(&'a [DiffLine]),
}

/// Splits the lines into shown runs and folds: `context` unchanged lines are kept on
/// each side of a change, and a longer run of unchanged lines is folded. A fold
/// always hides at least two lines.
#[must_use]
pub fn fold_unchanged(lines: &[DiffLine], context: usize) -> Vec<DiffBlock<'_>> {
    let mut blocks = Vec::new();
    let mut index = 0;
    let mut seen_change = false;
    while index < lines.len() {
        let kind_equal = lines[index].kind == DiffKind::Equal;
        let mut end = index;
        while end < lines.len() && (lines[end].kind == DiffKind::Equal) == kind_equal {
            end += 1;
        }
        let run = &lines[index..end];
        if kind_equal {
            let keep_before = if seen_change { context } else { 0 };
            let keep_after = if end == lines.len() { 0 } else { context };
            if run.len() > keep_before + keep_after + 1 {
                if keep_before > 0 {
                    blocks.push(DiffBlock::Lines(&run[..keep_before]));
                }
                blocks.push(DiffBlock::Fold(&run[keep_before..run.len() - keep_after]));
                if keep_after > 0 {
                    blocks.push(DiffBlock::Lines(&run[run.len() - keep_after..]));
                }
            } else {
                blocks.push(DiffBlock::Lines(run));
            }
        } else {
            seen_change = true;
            blocks.push(DiffBlock::Lines(run));
        }
        index = end;
    }
    blocks
}

/// Pairs removed and added lines side by side within each change, for the split
/// layout. An unchanged line is on both sides.
#[must_use]
pub fn split_rows(lines: &[DiffLine]) -> Vec<(Option<&DiffLine>, Option<&DiffLine>)> {
    type Pair<'a> = (Option<&'a DiffLine>, Option<&'a DiffLine>);
    fn flush<'a>(
        rows: &mut Vec<Pair<'a>>,
        removed: &mut Vec<&'a DiffLine>,
        added: &mut Vec<&'a DiffLine>,
    ) {
        for index in 0..removed.len().max(added.len()) {
            rows.push((removed.get(index).copied(), added.get(index).copied()));
        }
        removed.clear();
        added.clear();
    }
    let mut rows = Vec::new();
    let mut removed: Vec<&DiffLine> = Vec::new();
    let mut added: Vec<&DiffLine> = Vec::new();
    for line in lines {
        match line.kind {
            DiffKind::Remove => removed.push(line),
            DiffKind::Add => added.push(line),
            DiffKind::Equal => {
                flush(&mut rows, &mut removed, &mut added);
                rows.push((Some(line), Some(line)));
            }
        }
    }
    flush(&mut rows, &mut removed, &mut added);
    rows
}

/// A JSON value. An object keeps its keys in the order they were written.
#[derive(Clone, Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
    /// Parses JSON text. `None` when the text is not JSON.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        let mut parser = JsonParser {
            bytes: text.as_bytes(),
            at: 0,
            depth: 0,
        };
        parser.space();
        let value = parser.value()?;
        parser.space();
        (parser.at == parser.bytes.len()).then_some(value)
    }

    /// The value as display text, as `JSON.stringify(value, null, 2)` writes it:
    /// strings quoted, objects and arrays indented by two spaces.
    #[must_use]
    pub fn format(&self) -> String {
        let mut out = String::new();
        self.write(&mut out, 0);
        out
    }

    fn write(&self, out: &mut String, indent: usize) {
        let pad = |out: &mut String, level: usize| out.push_str(&"  ".repeat(level));
        match self {
            Self::Null => out.push_str("null"),
            Self::Bool(value) => out.push_str(if *value { "true" } else { "false" }),
            Self::Number(value) => out.push_str(&format_json_number(*value)),
            Self::String(value) => write_json_string(out, value),
            Self::Array(items) if items.is_empty() => out.push_str("[]"),
            Self::Array(items) => {
                out.push_str("[\n");
                for (index, item) in items.iter().enumerate() {
                    pad(out, indent + 1);
                    item.write(out, indent + 1);
                    out.push_str(if index + 1 < items.len() { ",\n" } else { "\n" });
                }
                pad(out, indent);
                out.push(']');
            }
            Self::Object(entries) if entries.is_empty() => out.push_str("{}"),
            Self::Object(entries) => {
                out.push_str("{\n");
                for (index, (key, value)) in entries.iter().enumerate() {
                    pad(out, indent + 1);
                    write_json_string(out, key);
                    out.push_str(": ");
                    value.write(out, indent + 1);
                    out.push_str(if index + 1 < entries.len() {
                        ",\n"
                    } else {
                        "\n"
                    });
                }
                pad(out, indent);
                out.push('}');
            }
        }
    }

    fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            Self::Object(entries) => entries
                .iter()
                .find(|(name, _)| name == key)
                .map(|(_, value)| value),
            _ => None,
        }
    }
}

fn format_json_number(value: f64) -> String {
    if !value.is_finite() {
        return "null".to_owned();
    }
    #[allow(clippy::cast_possible_truncation)]
    if value.fract() == 0.0 && value.abs() < 1e15 {
        return format!("{}", value as i64);
    }
    format!("{value}")
}

fn write_json_string(out: &mut String, text: &str) {
    out.push('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            c if u32::from(c) < 0x20 => out.push_str(&format!("\\u{:04x}", u32::from(c))),
            c => out.push(c),
        }
    }
    out.push('"');
}

/// A small recursive-descent JSON parser.
struct JsonParser<'a> {
    bytes: &'a [u8],
    at: usize,
    depth: usize,
}

impl JsonParser<'_> {
    fn space(&mut self) {
        while matches!(self.bytes.get(self.at), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.at += 1;
        }
    }

    fn eat(&mut self, text: &str) -> Option<()> {
        self.bytes[self.at..]
            .starts_with(text.as_bytes())
            .then(|| self.at += text.len())
    }

    fn value(&mut self) -> Option<JsonValue> {
        // Deeply nested input must not overflow the stack.
        if self.depth > 128 {
            return None;
        }
        match *self.bytes.get(self.at)? {
            b'n' => self.eat("null").map(|()| JsonValue::Null),
            b't' => self.eat("true").map(|()| JsonValue::Bool(true)),
            b'f' => self.eat("false").map(|()| JsonValue::Bool(false)),
            b'"' => self.string().map(JsonValue::String),
            b'[' => {
                self.at += 1;
                self.depth += 1;
                let mut items = Vec::new();
                self.space();
                if self.bytes.get(self.at) == Some(&b']') {
                    self.at += 1;
                } else {
                    loop {
                        self.space();
                        items.push(self.value()?);
                        self.space();
                        match self.bytes.get(self.at)? {
                            b',' => self.at += 1,
                            b']' => {
                                self.at += 1;
                                break;
                            }
                            _ => return None,
                        }
                    }
                }
                self.depth -= 1;
                Some(JsonValue::Array(items))
            }
            b'{' => {
                self.at += 1;
                self.depth += 1;
                let mut entries: Vec<(String, JsonValue)> = Vec::new();
                self.space();
                if self.bytes.get(self.at) == Some(&b'}') {
                    self.at += 1;
                } else {
                    loop {
                        self.space();
                        let key = self.string()?;
                        self.space();
                        self.eat(":")?;
                        self.space();
                        let value = self.value()?;
                        // A repeated key keeps its place and takes the last value.
                        match entries.iter_mut().find(|(name, _)| *name == key) {
                            Some(entry) => entry.1 = value,
                            None => entries.push((key, value)),
                        }
                        self.space();
                        match self.bytes.get(self.at)? {
                            b',' => self.at += 1,
                            b'}' => {
                                self.at += 1;
                                break;
                            }
                            _ => return None,
                        }
                    }
                }
                self.depth -= 1;
                Some(JsonValue::Object(entries))
            }
            b'-' | b'0'..=b'9' => self.number(),
            _ => None,
        }
    }

    fn number(&mut self) -> Option<JsonValue> {
        let start = self.at;
        let digits = |parser: &mut Self| {
            let from = parser.at;
            while matches!(parser.bytes.get(parser.at), Some(b'0'..=b'9')) {
                parser.at += 1;
            }
            parser.at > from
        };
        if self.bytes.get(self.at) == Some(&b'-') {
            self.at += 1;
        }
        let whole = self.at;
        if !digits(self) {
            return None;
        }
        // No leading zeros: "01" is not JSON.
        if self.bytes[whole] == b'0' && self.at - whole > 1 {
            return None;
        }
        if self.bytes.get(self.at) == Some(&b'.') {
            self.at += 1;
            if !digits(self) {
                return None;
            }
        }
        if matches!(self.bytes.get(self.at), Some(b'e' | b'E')) {
            self.at += 1;
            if matches!(self.bytes.get(self.at), Some(b'+' | b'-')) {
                self.at += 1;
            }
            if !digits(self) {
                return None;
            }
        }
        std::str::from_utf8(&self.bytes[start..self.at])
            .ok()?
            .parse()
            .ok()
            .map(JsonValue::Number)
    }

    fn hex4(&mut self) -> Option<u32> {
        let text = std::str::from_utf8(self.bytes.get(self.at..self.at + 4)?).ok()?;
        let code = u32::from_str_radix(text, 16).ok()?;
        self.at += 4;
        Some(code)
    }

    fn string(&mut self) -> Option<String> {
        self.eat("\"")?;
        let mut out: Vec<u8> = Vec::new();
        loop {
            let byte = *self.bytes.get(self.at)?;
            self.at += 1;
            match byte {
                b'"' => break,
                b'\\' => {
                    let escape = *self.bytes.get(self.at)?;
                    self.at += 1;
                    let c = match escape {
                        b'"' => '"',
                        b'\\' => '\\',
                        b'/' => '/',
                        b'b' => '\u{8}',
                        b'f' => '\u{c}',
                        b'n' => '\n',
                        b'r' => '\r',
                        b't' => '\t',
                        b'u' => {
                            let high = self.hex4()?;
                            let code = if (0xD800..0xDC00).contains(&high)
                                && self.bytes[self.at..].starts_with(b"\\u")
                            {
                                self.at += 2;
                                let low = self.hex4()?;
                                if (0xDC00..0xE000).contains(&low) {
                                    0x10000 + ((high - 0xD800) << 10) + (low - 0xDC00)
                                } else {
                                    0xFFFD
                                }
                            } else {
                                high
                            };
                            char::from_u32(code).unwrap_or('\u{FFFD}')
                        }
                        _ => return None,
                    };
                    out.extend_from_slice(c.encode_utf8(&mut [0; 4]).as_bytes());
                }
                0..=0x1F => return None,
                byte => out.push(byte),
            }
        }
        String::from_utf8(out).ok()
    }
}

/// What happened to a key of a JSON value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JsonChangeKind {
    Added,
    Removed,
    Changed,
}

impl JsonChangeKind {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Changed => "changed",
        }
    }
}

/// One difference between two JSON values.
#[derive(Clone, Debug, PartialEq)]
pub struct JsonChange {
    /// Added, removed or changed.
    pub kind: JsonChangeKind,
    /// Where the change is, such as `limits.maxPages` or `tags[2]`. Empty for the
    /// root value.
    pub path: String,
    /// The old value, unless the key was added.
    pub before: Option<JsonValue>,
    /// The new value, unless the key was removed.
    pub after: Option<JsonValue>,
}

fn key_path(parent: &str, key: &str) -> String {
    let mut chars = key.chars();
    let plain = chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_' || c == '$')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$');
    if plain {
        if parent.is_empty() {
            key.to_owned()
        } else {
            format!("{parent}.{key}")
        }
    } else {
        let mut quoted = String::new();
        write_json_string(&mut quoted, key);
        format!("{parent}[{quoted}]")
    }
}

/// A structural diff of two JSON values: the added, removed and changed keys with
/// their paths, in document order (the keys of `before` first, then new keys).
/// Arrays are compared by index, and a value that changes type is "changed".
/// `None` stands for a value that is not there.
#[must_use]
pub fn diff_json(before: Option<&JsonValue>, after: Option<&JsonValue>) -> Vec<JsonChange> {
    let mut out = Vec::new();
    diff_json_at(before, after, "", &mut out);
    out
}

fn diff_json_at(
    before: Option<&JsonValue>,
    after: Option<&JsonValue>,
    path: &str,
    out: &mut Vec<JsonChange>,
) {
    if before == after {
        return;
    }
    let change = |kind| JsonChange {
        kind,
        path: path.to_owned(),
        before: before.cloned(),
        after: after.cloned(),
    };
    match (before, after) {
        (Some(JsonValue::Object(old)), Some(new @ JsonValue::Object(new_entries))) => {
            for (key, value) in old {
                diff_json_at(Some(value), new.get(key), &key_path(path, key), out);
            }
            for (key, value) in new_entries {
                if !old.iter().any(|(name, _)| name == key) {
                    diff_json_at(None, Some(value), &key_path(path, key), out);
                }
            }
        }
        (Some(JsonValue::Array(old)), Some(JsonValue::Array(new))) => {
            for index in 0..old.len().max(new.len()) {
                diff_json_at(
                    old.get(index),
                    new.get(index),
                    &format!("{path}[{index}]"),
                    out,
                );
            }
        }
        (None, Some(_)) => out.push(change(JsonChangeKind::Added)),
        (Some(_), None) => out.push(change(JsonChangeKind::Removed)),
        _ => out.push(change(JsonChangeKind::Changed)),
    }
}

/// Reads one side of a JSON diff: empty text is a value that is not there, JSON is
/// parsed, and anything else is taken as a string.
#[must_use]
pub fn parse_json_side(text: &str) -> Option<JsonValue> {
    if text.trim().is_empty() {
        return None;
    }
    Some(JsonValue::parse(text).unwrap_or_else(|| JsonValue::String(text.to_owned())))
}

/// What a [`diff_viewer`] compares.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DiffFormat {
    /// Lines of text.
    #[default]
    Text,
    /// Two JSON values.
    Json,
}

/// The layout of a [`diff_viewer`]. Rendered as `data-mode`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DiffMode {
    /// One column, old then new.
    #[default]
    Unified,
    /// Before beside after.
    Split,
}

impl DiffMode {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unified => "unified",
            Self::Split => "split",
        }
    }
}

/// The strings of a [`diff_viewer`]. Start from `DiffViewerLabels::default()`.
///
/// A summary has a form for a count of one and a form for the rest; the count that
/// chooses is the first one in the text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiffViewerLabels<'a> {
    pub before: &'a str,
    pub after: &'a str,
    pub added: &'a str,
    pub removed: &'a str,
    pub changed: &'a str,
    /// The layout toggle's two items, and the name of their group.
    pub unified: &'a str,
    pub split: &'a str,
    pub mode: &'a str,
    /// Column headers.
    pub field: &'a str,
    pub value: &'a str,
    pub content: &'a str,
    /// The header of a column of line numbers, with `{side}` for "Before" or "After".
    pub line: &'a str,
    pub no_changes: &'a str,
    /// Announced for a value that is not there.
    pub not_set: &'a str,
    /// Shown as the path of a change to the whole value.
    pub root: &'a str,
    /// "Show 12 unchanged lines", with `{count}`.
    pub show_unchanged_one: &'a str,
    pub show_unchanged_other: &'a str,
    /// "4 lines added, 1 removed", with `{added}` and `{removed}`.
    pub text_summary_one: &'a str,
    pub text_summary_other: &'a str,
    /// "3 changes: 1 added, 1 removed, 1 changed", with `{total}`, `{added}`,
    /// `{removed}` and `{changed}`.
    pub json_summary_one: &'a str,
    pub json_summary_other: &'a str,
    /// Shown when the texts were too different to align.
    pub approximate: &'a str,
}

impl Default for DiffViewerLabels<'_> {
    fn default() -> Self {
        Self {
            before: "Before",
            after: "After",
            added: "Added",
            removed: "Removed",
            changed: "Changed",
            unified: "Unified",
            split: "Split",
            mode: "Diff layout",
            field: "Field",
            value: "Value",
            content: "Content",
            line: "{side} line",
            no_changes: "No changes",
            not_set: "Not set",
            root: "(root)",
            show_unchanged_one: "Show {count} unchanged line",
            show_unchanged_other: "Show {count} unchanged lines",
            text_summary_one: "{added} line added, {removed} removed",
            text_summary_other: "{added} lines added, {removed} removed",
            json_summary_one: "{total} change: {added} added, {removed} removed, {changed} changed",
            json_summary_other: "{total} changes: {added} added, {removed} removed, {changed} changed",
            approximate: "The versions are too different to align line by line; all old lines are shown removed and all new lines added.",
        }
    }
}

fn count(n: usize) -> String {
    #[allow(clippy::cast_precision_loss)]
    let n = n as f64;
    format_number(n, 0, false)
}

impl DiffViewerLabels<'_> {
    /// "4 lines added, 1 removed".
    #[must_use]
    pub fn text_summary(&self, added: usize, removed: usize) -> String {
        if added == 1 {
            self.text_summary_one
        } else {
            self.text_summary_other
        }
        .replace("{added}", &count(added))
        .replace("{removed}", &count(removed))
    }

    /// "3 changes: 1 added, 1 removed, 1 changed".
    #[must_use]
    pub fn json_summary(&self, added: usize, removed: usize, changed: usize) -> String {
        let total = added + removed + changed;
        if total == 1 {
            self.json_summary_one
        } else {
            self.json_summary_other
        }
        .replace("{total}", &count(total))
        .replace("{added}", &count(added))
        .replace("{removed}", &count(removed))
        .replace("{changed}", &count(changed))
    }

    /// "Show 12 unchanged lines".
    #[must_use]
    pub fn show_unchanged(&self, lines: usize) -> String {
        if lines == 1 {
            self.show_unchanged_one
        } else {
            self.show_unchanged_other
        }
        .replace("{count}", &count(lines))
    }
}

/// One cell of content: its tint, its sign, the word announced before it, its line
/// number and its text. `None` text is a value that is not there.
#[derive(Clone, Default)]
struct Cell {
    kind: &'static str,
    sign: &'static str,
    word: String,
    number: String,
    text: Option<String>,
    blank: bool,
}

/// One table row of either layout.
enum Row {
    /// The button that unfolds the rows after it with the same fold number.
    Fold { id: usize, text: String },
    /// A unified text row: both line numbers, then the content.
    Unified {
        fold: Option<usize>,
        old: String,
        new: String,
        cell: Cell,
    },
    /// A split text row: a number and the content for each side.
    Split {
        fold: Option<usize>,
        left: Cell,
        right: Cell,
    },
}

/// One row of a JSON table: the path with its sign, then one or two values.
struct JsonRow {
    kind: &'static str,
    sign: &'static str,
    word: String,
    path: String,
    cells: Vec<Cell>,
}

fn sign_of(kind: DiffKind) -> &'static str {
    match kind {
        DiffKind::Add => "+",
        DiffKind::Remove => "−",
        DiffKind::Equal => " ",
    }
}

fn number(n: Option<usize>) -> String {
    n.map(|n| n.to_string()).unwrap_or_default()
}

fn line_cell(line: Option<&DiffLine>, left: bool, labels: &DiffViewerLabels<'_>) -> Cell {
    let Some(line) = line else {
        return Cell {
            kind: "none",
            blank: true,
            ..Cell::default()
        };
    };
    Cell {
        kind: line.kind.as_str(),
        sign: sign_of(line.kind),
        word: match line.kind {
            DiffKind::Add => labels.added.to_owned(),
            DiffKind::Remove => labels.removed.to_owned(),
            DiffKind::Equal => String::new(),
        },
        number: number(if left {
            line.old_number
        } else {
            line.new_number
        }),
        text: Some(line.text.clone()),
        blank: false,
    }
}

/// A before and after viewer. `attrs` go to the root element.
///
/// - `label` names the diff table and its scroll region.
/// - `before` and `after` are the two values: text, or with [`DiffFormat::Json`]
///   JSON text. Empty JSON text is a value that is not there.
/// - `mode` is the layout shown first, and `show_mode_toggle` offers the other.
/// - `context_lines` is how many unchanged lines stay around each change of a text.
/// - `max_height` is the height the scroll area stops at, a CSS length.
/// - `wrap` wraps long lines; otherwise they scroll sideways.
/// - `actions` is a slot in the header, for a copy button for example.
/// - `labels` are the strings.
#[component]
pub async fn diff_viewer(
    cx: &Cx,
    label: &str,
    before: &str,
    after: &str,
    #[default] format: DiffFormat,
    #[default] mode: DiffMode,
    #[default(true)] show_mode_toggle: bool,
    #[default(3)] context_lines: usize,
    #[default("24rem")] max_height: &str,
    #[default(true)] wrap: bool,
    #[default]
    #[into]
    actions: Child<'_>,
    #[default] labels: DiffViewerLabels<'_>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let mut unified: Vec<Row> = Vec::new();
    let mut split: Vec<Row> = Vec::new();
    let mut json_unified: Vec<JsonRow> = Vec::new();
    let mut json_split: Vec<JsonRow> = Vec::new();
    let mut approximate = false;
    let summary;
    let empty;
    match format {
        DiffFormat::Text => {
            let diff = diff_text(before, after, 4000);
            approximate = diff.approximate;
            empty = diff.added == 0 && diff.removed == 0;
            summary = if empty {
                labels.no_changes.to_owned()
            } else {
                labels.text_summary(diff.added, diff.removed)
            };
            let mut fold_id = 0;
            for block in fold_unchanged(&diff.lines, context_lines) {
                let (lines, fold) = match block {
                    DiffBlock::Lines(lines) => (lines, None),
                    DiffBlock::Fold(lines) => {
                        fold_id += 1;
                        let text = labels.show_unchanged(lines.len());
                        unified.push(Row::Fold {
                            id: fold_id,
                            text: text.clone(),
                        });
                        split.push(Row::Fold { id: fold_id, text });
                        (lines, Some(fold_id))
                    }
                };
                for line in lines {
                    unified.push(Row::Unified {
                        fold,
                        old: number(line.old_number),
                        new: number(line.new_number),
                        cell: line_cell(Some(line), true, &labels),
                    });
                }
                for (left, right) in split_rows(lines) {
                    split.push(Row::Split {
                        fold,
                        left: line_cell(left, true, &labels),
                        right: line_cell(right, false, &labels),
                    });
                }
            }
        }
        DiffFormat::Json => {
            let old = parse_json_side(before);
            let new = parse_json_side(after);
            let changes = diff_json(old.as_ref(), new.as_ref());
            let of = |kind| changes.iter().filter(|change| change.kind == kind).count();
            empty = changes.is_empty();
            summary = if empty {
                labels.no_changes.to_owned()
            } else {
                labels.json_summary(
                    of(JsonChangeKind::Added),
                    of(JsonChangeKind::Removed),
                    of(JsonChangeKind::Changed),
                )
            };
            let value = |value: &Option<JsonValue>, kind: &'static str| Cell {
                kind,
                text: value.as_ref().map(JsonValue::format),
                ..Cell::default()
            };
            for change in &changes {
                let path = if change.path.is_empty() {
                    labels.root.to_owned()
                } else {
                    change.path.clone()
                };
                let (sign, word) = match change.kind {
                    JsonChangeKind::Added => ("+", labels.added),
                    JsonChangeKind::Removed => ("−", labels.removed),
                    JsonChangeKind::Changed => ("~", labels.changed),
                };
                json_split.push(JsonRow {
                    kind: change.kind.as_str(),
                    sign,
                    word: word.to_owned(),
                    path: path.clone(),
                    cells: vec![
                        value(
                            &change.before,
                            if change.kind == JsonChangeKind::Added {
                                "none"
                            } else {
                                "remove"
                            },
                        ),
                        value(
                            &change.after,
                            if change.kind == JsonChangeKind::Removed {
                                "none"
                            } else {
                                "add"
                            },
                        ),
                    ],
                });
                let changed = change.kind == JsonChangeKind::Changed;
                if change.kind != JsonChangeKind::Added {
                    json_unified.push(JsonRow {
                        kind: "remove",
                        sign: "−",
                        word: if changed {
                            format!("{}, {}", labels.changed, labels.before.to_lowercase())
                        } else {
                            labels.removed.to_owned()
                        },
                        path: path.clone(),
                        cells: vec![value(&change.before, "")],
                    });
                }
                if change.kind != JsonChangeKind::Removed {
                    json_unified.push(JsonRow {
                        kind: "add",
                        sign: "+",
                        word: if changed {
                            format!("{}, {}", labels.changed, labels.after.to_lowercase())
                        } else {
                            labels.added.to_owned()
                        },
                        path,
                        cells: vec![value(&change.after, "")],
                    });
                }
            }
        }
    }
    let json = format == DiffFormat::Json;
    let before_line = labels.line.replace("{side}", labels.before);
    let after_line = labels.line.replace("{side}", labels.after);
    let scroll_style = format!("--diff-max-height: {max_height}");
    let not_set = labels.not_set;

    Ok(view! {
        <div
            class=(class!(css::ROOT, attrs.remove("class")))
            data-diff-viewer=""
            data-mode=(mode.as_str())
            data-wrap=(wrap.then_some(""))
            (attrs)
        >
            <div class=(css::HEADER)>
                <p class=(css::SUMMARY)>(summary)</p>
                <div class=(css::ACTIONS)>
                    (actions)
                    if show_mode_toggle && !empty {
                        toggle_group(
                            label: labels.mode,
                            size: ToggleSize::Sm,
                            variant: ToggleVariant::Outline,
                            joined: true,
                            attrs: attributes! { data-diff-modes="" },
                            toggle_group_item(
                                value: "unified",
                                pressed: mode == DiffMode::Unified,
                                (labels.unified)
                            )
                            toggle_group_item(
                                value: "split",
                                pressed: mode == DiffMode::Split,
                                (labels.split)
                            )
                        )
                    }
                </div>
            </div>
            if approximate {
                <p class=(css::NOTE)>(labels.approximate)</p>
            }
            if !empty {
                scroll_area(
                    label: label,
                    orientation: ScrollOrientation::Both,
                    attrs: attributes! { class=(css::SCROLL) style=(scroll_style) },
                    if json {
                        <table class=(css::TABLE) data-table="unified">
                            <caption class="sr-only">(label)</caption>
                            <thead class=(css::HIDDEN_HEAD)>
                                <tr>
                                    <th scope="col">(labels.field)</th>
                                    <th scope="col">(labels.value)</th>
                                </tr>
                            </thead>
                            <tbody>
                                for row in json_unified {
                                    <tr data-kind=(row.kind)>
                                        <th scope="row" class=(css::FIELD)>
                                            <span aria-hidden="true" class=(css::SIGN)>
                                                (row.sign)
                                            </span>
                                            <span class="sr-only">
                                                (row.word)
                                                ": "
                                            </span>
                                            <code class=(css::PATH)>(row.path)</code>
                                        </th>
                                        for cell in row.cells {
                                            <td class=(css::CODE)>
                                                if let Some(text) = cell.text {
                                                    <span class=(css::TEXT)>(text)</span>
                                                } else {
                                                    <span class=(css::NOT_SET)>
                                                        <span aria-hidden="true">"—"</span>
                                                        <span class="sr-only">(not_set)</span>
                                                    </span>
                                                }
                                            </td>
                                        }
                                    </tr>
                                }
                            </tbody>
                        </table>
                        <table class=(css::TABLE) data-table="split">
                            <caption class="sr-only">(label)</caption>
                            <thead>
                                <tr>
                                    <th scope="col">(labels.field)</th>
                                    <th scope="col">(labels.before)</th>
                                    <th scope="col">(labels.after)</th>
                                </tr>
                            </thead>
                            <tbody>
                                for row in json_split {
                                    <tr data-kind=(row.kind)>
                                        <th scope="row" class=(css::FIELD)>
                                            <span aria-hidden="true" class=(css::SIGN)>
                                                (row.sign)
                                            </span>
                                            <span class="sr-only">
                                                (row.word)
                                                ": "
                                            </span>
                                            <code class=(css::PATH)>(row.path)</code>
                                        </th>
                                        for cell in row.cells {
                                            <td class=(css::CODE) data-kind=(cell.kind)>
                                                if let Some(text) = cell.text {
                                                    <span class=(css::TEXT)>(text)</span>
                                                } else {
                                                    <span class=(css::NOT_SET)>
                                                        <span aria-hidden="true">"—"</span>
                                                        <span class="sr-only">(not_set)</span>
                                                    </span>
                                                }
                                            </td>
                                        }
                                    </tr>
                                }
                            </tbody>
                        </table>
                    } else {
                        <table class=(css::TABLE) data-table="unified">
                            <caption class="sr-only">(label)</caption>
                            <thead class=(css::HIDDEN_HEAD)>
                                <tr>
                                    <th scope="col">(before_line.clone())</th>
                                    <th scope="col">(after_line.clone())</th>
                                    <th scope="col">(labels.content)</th>
                                </tr>
                            </thead>
                            <tbody>
                                for row in unified {
                                    match row {
                                        Row::Fold { id, text } => {
                                            <tr class=(css::FOLD_ROW) data-fold-row=(id.to_string())>
                                                <td colspan="3">
                                                    <button
                                                        type="button"
                                                        class=(css::FOLD)
                                                        data-diff-fold=(id.to_string())
                                                    >
                                                        (text)
                                                    </button>
                                                </td>
                                            </tr>
                                        }
                                        Row::Unified { fold, old, new, cell } => {
                                            <tr
                                                data-kind=(cell.kind)
                                                data-fold=(fold.map(|id| id.to_string()))
                                                hidden=(fold.map(|_| ""))
                                            >
                                                <td class=(css::NUM)>(old)</td>
                                                <td class=(css::NUM)>(new)</td>
                                                <td class=(css::CODE)>
                                                    <span aria-hidden="true" class=(css::SIGN)>
                                                        (cell.sign)
                                                    </span>
                                                    if !cell.word.is_empty() {
                                                        <span class="sr-only">
                                                            (cell.word)
                                                            ": "
                                                        </span>
                                                    }
                                                    <span class=(css::TEXT)>
                                                        (cell.text.unwrap_or_default())
                                                    </span>
                                                </td>
                                            </tr>
                                        }
                                        Row::Split { .. } => {

                                        }
                                    }
                                }
                            </tbody>
                        </table>
                        <table class=(css::TABLE) data-table="split">
                            <caption class="sr-only">(label)</caption>
                            <thead>
                                <tr>
                                    <th scope="col" class=(css::NUM_HEAD)>
                                        <span class="sr-only">(before_line)</span>
                                    </th>
                                    <th scope="col">(labels.before)</th>
                                    <th scope="col" class=(css::NUM_HEAD)>
                                        <span class="sr-only">(after_line)</span>
                                    </th>
                                    <th scope="col">(labels.after)</th>
                                </tr>
                            </thead>
                            <tbody>
                                for row in split {
                                    match row {
                                        Row::Fold { id, text } => {
                                            <tr class=(css::FOLD_ROW) data-fold-row=(id.to_string())>
                                                <td colspan="4">
                                                    <button
                                                        type="button"
                                                        class=(css::FOLD)
                                                        data-diff-fold=(id.to_string())
                                                    >
                                                        (text)
                                                    </button>
                                                </td>
                                            </tr>
                                        }
                                        Row::Split { fold, left, right } => {
                                            <tr
                                                data-fold=(fold.map(|id| id.to_string()))
                                                hidden=(fold.map(|_| ""))
                                            >
                                                for cell in [left, right] {
                                                    <td class=(css::NUM) data-kind=(cell.kind)>
                                                        (cell.number)
                                                    </td>
                                                    <td class=(css::CODE) data-kind=(cell.kind)>
                                                        if !cell.blank {
                                                            <span aria-hidden="true" class=(css::SIGN)>
                                                                (cell.sign)
                                                            </span>
                                                            if !cell.word.is_empty() {
                                                                <span class="sr-only">
                                                                    (cell.word)
                                                                    ": "
                                                                </span>
                                                            }
                                                            <span class=(css::TEXT)>
                                                                (cell.text.unwrap_or_default())
                                                            </span>
                                                        }
                                                    </td>
                                                }
                                            </tr>
                                        }
                                        Row::Unified { .. } => {

                                        }
                                    }
                                }
                            </tbody>
                        </table>
                    }
                )
            }
        </div>
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The diff as text: "=", "+" or "-" and the line, with both line numbers.
    fn show(diff: &TextDiff) -> Vec<String> {
        diff.lines
            .iter()
            .map(|line| {
                format!(
                    "{}{} {}:{}",
                    match line.kind {
                        DiffKind::Equal => "=",
                        DiffKind::Add => "+",
                        DiffKind::Remove => "-",
                    },
                    line.text,
                    line.old_number.map_or("_".to_owned(), |n| n.to_string()),
                    line.new_number.map_or("_".to_owned(), |n| n.to_string()),
                )
            })
            .collect()
    }

    /// Applies a diff to check it: the old side must be `before`, the new `after`.
    fn sides(diff: &TextDiff) -> (Vec<&str>, Vec<&str>) {
        let old = diff
            .lines
            .iter()
            .filter(|line| line.kind != DiffKind::Add)
            .map(|line| line.text.as_str())
            .collect();
        let new = diff
            .lines
            .iter()
            .filter(|line| line.kind != DiffKind::Remove)
            .map(|line| line.text.as_str())
            .collect();
        (old, new)
    }

    #[test]
    fn splits_lines() {
        assert_eq!(split_lines(""), Vec::<&str>::new());
        assert_eq!(split_lines("a"), ["a"]);
        assert_eq!(split_lines("a\nb\n"), ["a", "b"]);
        assert_eq!(split_lines("a\r\nb\rc"), ["a", "b", "c"]);
        assert_eq!(split_lines("a\n\nb"), ["a", "", "b"]);
        assert_eq!(split_lines("\n"), [""]);
    }

    #[test]
    fn equal_texts_have_no_changes() {
        let diff = diff_text("a\nb\nc", "a\nb\nc\n", 4000);
        assert_eq!((diff.added, diff.removed, diff.approximate), (0, 0, false));
        assert_eq!(show(&diff), ["=a 1:1", "=b 2:2", "=c 3:3"]);
        let empty = diff_text("", "", 4000);
        assert!(empty.lines.is_empty());
    }

    #[test]
    fn a_changed_line_is_removed_then_added() {
        let diff = diff_text("a\nb\nc", "a\nB\nc", 4000);
        assert_eq!(show(&diff), ["=a 1:1", "-b 2:_", "+B _:2", "=c 3:3"]);
        assert_eq!((diff.added, diff.removed), (1, 1));
    }

    #[test]
    fn additions_and_removals() {
        assert_eq!(
            show(&diff_text("a\nc", "a\nb\nc", 4000)),
            ["=a 1:1", "+b _:2", "=c 2:3"]
        );
        assert_eq!(
            show(&diff_text("a\nb\nc", "a\nc", 4000)),
            ["=a 1:1", "-b 2:_", "=c 3:2"]
        );
        assert_eq!(show(&diff_text("", "a\nb", 4000)), ["+a _:1", "+b _:2"]);
        assert_eq!(show(&diff_text("a\nb", "", 4000)), ["-a 1:_", "-b 2:_"]);
    }

    #[test]
    fn the_classic_example_has_the_shortest_script() {
        // Myers' paper: ABCABBA to CBABAC takes five edits.
        let before = "A\nB\nC\nA\nB\nB\nA";
        let after = "C\nB\nA\nB\nA\nC";
        let diff = diff_text(before, after, 4000);
        assert_eq!(diff.added + diff.removed, 5);
        let (old, new) = sides(&diff);
        assert_eq!(old, split_lines(before));
        assert_eq!(new, split_lines(after));
    }

    #[test]
    fn every_diff_rebuilds_both_texts() {
        let texts = [
            "",
            "a",
            "a\nb\nc\nd\ne",
            "a\nx\nc\ny\ne",
            "x\na\nb\nc\nd\ne\ny",
            "e\nd\nc\nb\na",
            "a\na\na\nb\na\na",
            "b\na\na\na",
            "one\ntwo\nthree\nfour\nfive\nsix\nseven\neight",
            "one\nthree\nfour\n4.5\nfive\nsix\nSEVEN\neight\nnine",
        ];
        for before in texts {
            for after in texts {
                let diff = diff_text(before, after, 4000);
                let (old, new) = sides(&diff);
                assert_eq!(old, split_lines(before), "{before:?} -> {after:?}");
                assert_eq!(new, split_lines(after), "{before:?} -> {after:?}");
                assert!(!diff.approximate);
                // Line numbers count up on each side.
                let olds: Vec<usize> = diff.lines.iter().filter_map(|l| l.old_number).collect();
                let news: Vec<usize> = diff.lines.iter().filter_map(|l| l.new_number).collect();
                assert_eq!(olds, (1..=olds.len()).collect::<Vec<_>>());
                assert_eq!(news, (1..=news.len()).collect::<Vec<_>>());
                // A diff of a text with itself has no changes, so it is minimal there.
                if before == after {
                    assert_eq!(diff.added + diff.removed, 0);
                }
            }
        }
    }

    #[test]
    fn past_the_budget_everything_is_removed_and_added() {
        let diff = diff_text("a\nb\nc\nz", "x\ny\nz", 2);
        assert!(diff.approximate);
        assert_eq!(
            show(&diff),
            ["-a 1:_", "-b 2:_", "-c 3:_", "+x _:1", "+y _:2", "=z 4:3"]
        );
    }

    fn numbered(count: usize) -> String {
        (1..=count)
            .map(|n| format!("line {n}"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn folds_unchanged_lines_far_from_a_change() {
        let before = numbered(20);
        let after = before.replace("line 10", "line ten");
        let diff = diff_text(&before, &after, 4000);
        let shape: Vec<(bool, usize)> = fold_unchanged(&diff.lines, 3)
            .iter()
            .map(|block| match block {
                DiffBlock::Lines(lines) => (false, lines.len()),
                DiffBlock::Fold(lines) => (true, lines.len()),
            })
            .collect();
        // 9 lines before: 6 folded, 3 kept. The change. 10 after: 3 kept, 7 folded.
        assert_eq!(
            shape,
            [(true, 6), (false, 3), (false, 2), (false, 3), (true, 7)]
        );
    }

    #[test]
    fn short_runs_are_not_folded() {
        let diff = diff_text("a\nb\nc\nd\ne\nf\ng\nh", "A\nb\nc\nd\ne\nf\ng\nH", 4000);
        // Six unchanged lines between two changes with three lines of context each.
        let blocks = fold_unchanged(&diff.lines, 3);
        assert!(
            blocks
                .iter()
                .all(|block| matches!(block, DiffBlock::Lines(_)))
        );
        // Seven would leave one line to fold, which is not worth a button; eight folds two.
        let diff = diff_text(
            &format!("x\n{}\ny", numbered(8)),
            &format!("X\n{}\nY", numbered(8)),
            4000,
        );
        let folds: Vec<usize> = fold_unchanged(&diff.lines, 3)
            .iter()
            .filter_map(|block| match block {
                DiffBlock::Fold(lines) => Some(lines.len()),
                DiffBlock::Lines(_) => None,
            })
            .collect();
        assert_eq!(folds, [2]);
        // With no changes at all everything is one fold.
        let same = diff_text("a\nb\nc", "a\nb\nc", 4000);
        assert!(
            matches!(fold_unchanged(&same.lines, 3)[..], [DiffBlock::Fold(lines)] if lines.len() == 3)
        );
    }

    #[test]
    fn split_rows_pair_removed_with_added() {
        let diff = diff_text("a\nb\nc\nd", "a\nB\nC\nX\nd", 4000);
        let rows: Vec<(Option<&str>, Option<&str>)> = split_rows(&diff.lines)
            .iter()
            .map(|(left, right)| {
                (
                    left.map(|line| line.text.as_str()),
                    right.map(|line| line.text.as_str()),
                )
            })
            .collect();
        assert_eq!(
            rows,
            [
                (Some("a"), Some("a")),
                (Some("b"), Some("B")),
                (Some("c"), Some("C")),
                (None, Some("X")),
                (Some("d"), Some("d")),
            ]
        );
    }

    fn json(text: &str) -> JsonValue {
        JsonValue::parse(text).unwrap_or_else(|| panic!("not JSON: {text}"))
    }

    #[test]
    fn parses_json() {
        assert_eq!(json("null"), JsonValue::Null);
        assert_eq!(json(" true "), JsonValue::Bool(true));
        assert_eq!(json("-12.5e1"), JsonValue::Number(-125.0));
        assert_eq!(
            json(r#""a\"b\\c\n\u00e9\ud83d\ude00""#),
            JsonValue::String("a\"b\\c\né😀".to_owned())
        );
        assert_eq!(
            json(r#"{"a": [1, {"b": null}], "c": "d"}"#),
            JsonValue::Object(vec![
                (
                    "a".to_owned(),
                    JsonValue::Array(vec![
                        JsonValue::Number(1.0),
                        JsonValue::Object(vec![("b".to_owned(), JsonValue::Null)]),
                    ])
                ),
                ("c".to_owned(), JsonValue::String("d".to_owned())),
            ])
        );
        assert_eq!(json(r#"{"a": 1, "a": 2}"#), json(r#"{"a": 2}"#));
        assert_eq!(json("[]"), JsonValue::Array(Vec::new()));
        assert_eq!(json("{}"), JsonValue::Object(Vec::new()));
    }

    #[test]
    fn rejects_what_is_not_json() {
        for text in [
            "",
            "nul",
            "01",
            "1.",
            "+1",
            "[1,]",
            "{\"a\":}",
            "{a: 1}",
            "'a'",
            "\"a",
            "[1] 2",
            "\"tab\there\"",
            "{\"a\" 1}",
            "\"\\x\"",
            "-",
        ] {
            assert_eq!(JsonValue::parse(text), None, "{text:?}");
        }
        let deep = "[".repeat(1000) + &"]".repeat(1000);
        assert_eq!(JsonValue::parse(&deep), None);
    }

    #[test]
    fn formats_json_as_stringify_does() {
        assert_eq!(json(r#""a\"b""#).format(), r#""a\"b""#);
        assert_eq!(json("1.0").format(), "1");
        assert_eq!(json("0.25").format(), "0.25");
        assert_eq!(json("[]").format(), "[]");
        assert_eq!(
            json(r#"{"a":[1,true],"b":{}}"#).format(),
            "{\n  \"a\": [\n    1,\n    true\n  ],\n  \"b\": {}\n}"
        );
    }

    fn changes(before: &str, after: &str) -> Vec<(JsonChangeKind, String)> {
        diff_json(
            parse_json_side(before).as_ref(),
            parse_json_side(after).as_ref(),
        )
        .into_iter()
        .map(|change| (change.kind, change.path))
        .collect()
    }

    #[test]
    fn json_changes_have_paths_in_document_order() {
        use JsonChangeKind::*;
        let before = r#"{"name":"Support","limits":{"maxPages":10,"depth":2},"tags":["a","b","c"],"old key":1}"#;
        let after = r#"{"name":"Support","limits":{"maxPages":25,"depth":2},"tags":["a","B"],"model":"large"}"#;
        assert_eq!(
            changes(before, after),
            [
                (Changed, "limits.maxPages".to_owned()),
                (Changed, "tags[1]".to_owned()),
                (Removed, "tags[2]".to_owned()),
                (Removed, "[\"old key\"]".to_owned()),
                (Added, "model".to_owned()),
            ]
        );
    }

    #[test]
    fn json_edge_cases() {
        use JsonChangeKind::*;
        assert_eq!(changes(r#"{"a":1}"#, r#"{"a":1.0}"#), []);
        // Key order does not matter, and a type change is one change.
        assert_eq!(changes(r#"{"a":1,"b":2}"#, r#"{"b":2,"a":1}"#), []);
        assert_eq!(
            changes(r#"{"a":[1]}"#, r#"{"a":{"x":1}}"#),
            [(Changed, "a".to_owned())]
        );
        assert_eq!(changes("1", "2"), [(Changed, String::new())]);
        // A side that is not there, and text that is not JSON.
        assert_eq!(changes("", r#"{"a":1}"#), [(Added, String::new())]);
        assert_eq!(changes(r#"{"a":1}"#, ""), [(Removed, String::new())]);
        assert_eq!(changes("plain", "plain"), []);
        assert_eq!(changes("plain", "other"), [(Changed, String::new())]);
        assert_eq!(
            changes(r#"{"a":{"b":{}}}"#, r#"{"a":{"b":{"c d":[1]}}}"#),
            [(Added, "a.b[\"c d\"]".to_owned())]
        );
        let change = &diff_json(Some(&json(r#"{"a":1}"#)), Some(&json(r#"{"a":"1"}"#)))[0];
        assert_eq!(change.before, Some(JsonValue::Number(1.0)));
        assert_eq!(change.after, Some(JsonValue::String("1".to_owned())));
    }

    #[test]
    fn summaries() {
        let labels = DiffViewerLabels::default();
        assert_eq!(labels.text_summary(4, 1), "4 lines added, 1 removed");
        assert_eq!(labels.text_summary(1, 0), "1 line added, 0 removed");
        assert_eq!(labels.text_summary(1200, 0), "1,200 lines added, 0 removed");
        assert_eq!(
            labels.json_summary(1, 1, 1),
            "3 changes: 1 added, 1 removed, 1 changed"
        );
        assert_eq!(
            labels.json_summary(0, 0, 1),
            "1 change: 0 added, 0 removed, 1 changed"
        );
        assert_eq!(labels.show_unchanged(12), "Show 12 unchanged lines");
        assert_eq!(labels.show_unchanged(1), "Show 1 unchanged line");
    }
}
