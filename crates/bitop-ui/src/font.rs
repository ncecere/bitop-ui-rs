//! Inter, the library's typeface, served with the app so that components look the same
//! on machines that do not have it installed. The faces and ranges are those of
//! Fontsource's `@fontsource-variable/inter`, which the React library imports: one
//! variable file per script, downloaded only when a page uses its characters. The font
//! is under the SIL Open Font License; see `fonts/LICENSE-Inter.txt`.

use topcoat::{
    asset::{Asset, asset},
    font::{
        Font, FontDisplay, FontFace, FontFormat, FontSource, FontStyle, FontWeightRange,
        UnicodeRange, UnicodeRanges, font,
    },
};

/// The file for Latin text, which nearly every page needs; [`head`](crate::head)
/// preloads it.
pub const INTER_LATIN: Asset = asset!("../fonts/inter-latin-wght-normal.woff2");

/// The family name that `--font-family-sans` asks for first.
const FAMILY: &str = "Inter Variable";

fn face(file: Asset, ranges: &'static [UnicodeRange]) -> FontFace {
    let source = FontSource::url(file, Some(FontFormat::Woff2), None);
    FontFace::new(FAMILY, vec![source])
        .with_weight(FontWeightRange::from_u16(100, 900))
        .with_style(FontStyle::Normal)
        .with_display(FontDisplay::Swap)
        .with_unicode_range(UnicodeRanges::new(ranges))
}

fn faces() -> Vec<FontFace> {
    vec![
        face(
            asset!("../fonts/inter-cyrillic-ext-wght-normal.woff2"),
            const {
                &[
                    UnicodeRange::from_u32(0x0460, 0x052F),
                    UnicodeRange::from_u32(0x1C80, 0x1C8A),
                    UnicodeRange::from_u32(0x20B4, 0x20B4),
                    UnicodeRange::from_u32(0x2DE0, 0x2DFF),
                    UnicodeRange::from_u32(0xA640, 0xA69F),
                    UnicodeRange::from_u32(0xFE2E, 0xFE2F),
                ]
            },
        ),
        face(
            asset!("../fonts/inter-cyrillic-wght-normal.woff2"),
            const {
                &[
                    UnicodeRange::from_u32(0x0301, 0x0301),
                    UnicodeRange::from_u32(0x0400, 0x045F),
                    UnicodeRange::from_u32(0x0490, 0x0491),
                    UnicodeRange::from_u32(0x04B0, 0x04B1),
                    UnicodeRange::from_u32(0x2116, 0x2116),
                ]
            },
        ),
        face(
            asset!("../fonts/inter-greek-ext-wght-normal.woff2"),
            const { &[UnicodeRange::from_u32(0x1F00, 0x1FFF)] },
        ),
        face(
            asset!("../fonts/inter-greek-wght-normal.woff2"),
            const {
                &[
                    UnicodeRange::from_u32(0x0370, 0x0377),
                    UnicodeRange::from_u32(0x037A, 0x037F),
                    UnicodeRange::from_u32(0x0384, 0x038A),
                    UnicodeRange::from_u32(0x038C, 0x038C),
                    UnicodeRange::from_u32(0x038E, 0x03A1),
                    UnicodeRange::from_u32(0x03A3, 0x03FF),
                ]
            },
        ),
        face(
            asset!("../fonts/inter-vietnamese-wght-normal.woff2"),
            const {
                &[
                    UnicodeRange::from_u32(0x0102, 0x0103),
                    UnicodeRange::from_u32(0x0110, 0x0111),
                    UnicodeRange::from_u32(0x0128, 0x0129),
                    UnicodeRange::from_u32(0x0168, 0x0169),
                    UnicodeRange::from_u32(0x01A0, 0x01A1),
                    UnicodeRange::from_u32(0x01AF, 0x01B0),
                    UnicodeRange::from_u32(0x0300, 0x0301),
                    UnicodeRange::from_u32(0x0303, 0x0304),
                    UnicodeRange::from_u32(0x0308, 0x0309),
                    UnicodeRange::from_u32(0x0323, 0x0323),
                    UnicodeRange::from_u32(0x0329, 0x0329),
                    UnicodeRange::from_u32(0x1EA0, 0x1EF9),
                    UnicodeRange::from_u32(0x20AB, 0x20AB),
                ]
            },
        ),
        face(
            asset!("../fonts/inter-latin-ext-wght-normal.woff2"),
            const {
                &[
                    UnicodeRange::from_u32(0x0100, 0x02BA),
                    UnicodeRange::from_u32(0x02BD, 0x02C5),
                    UnicodeRange::from_u32(0x02C7, 0x02CC),
                    UnicodeRange::from_u32(0x02CE, 0x02D7),
                    UnicodeRange::from_u32(0x02DD, 0x02FF),
                    UnicodeRange::from_u32(0x0304, 0x0304),
                    UnicodeRange::from_u32(0x0308, 0x0308),
                    UnicodeRange::from_u32(0x0329, 0x0329),
                    UnicodeRange::from_u32(0x1D00, 0x1DBF),
                    UnicodeRange::from_u32(0x1E00, 0x1E9F),
                    UnicodeRange::from_u32(0x1EF2, 0x1EFF),
                    UnicodeRange::from_u32(0x2020, 0x2020),
                    UnicodeRange::from_u32(0x20A0, 0x20AB),
                    UnicodeRange::from_u32(0x20AD, 0x20C0),
                    UnicodeRange::from_u32(0x2113, 0x2113),
                    UnicodeRange::from_u32(0x2C60, 0x2C7F),
                    UnicodeRange::from_u32(0xA720, 0xA7FF),
                ]
            },
        ),
        face(
            INTER_LATIN,
            const {
                &[
                    UnicodeRange::from_u32(0x0000, 0x00FF),
                    UnicodeRange::from_u32(0x0131, 0x0131),
                    UnicodeRange::from_u32(0x0152, 0x0153),
                    UnicodeRange::from_u32(0x02BB, 0x02BC),
                    UnicodeRange::from_u32(0x02C6, 0x02C6),
                    UnicodeRange::from_u32(0x02DA, 0x02DA),
                    UnicodeRange::from_u32(0x02DC, 0x02DC),
                    UnicodeRange::from_u32(0x0304, 0x0304),
                    UnicodeRange::from_u32(0x0308, 0x0308),
                    UnicodeRange::from_u32(0x0329, 0x0329),
                    UnicodeRange::from_u32(0x2000, 0x206F),
                    UnicodeRange::from_u32(0x20AC, 0x20AC),
                    UnicodeRange::from_u32(0x2122, 0x2122),
                    UnicodeRange::from_u32(0x2191, 0x2191),
                    UnicodeRange::from_u32(0x2193, 0x2193),
                    UnicodeRange::from_u32(0x2212, 0x2212),
                    UnicodeRange::from_u32(0x2215, 0x2215),
                    UnicodeRange::from_u32(0xFEFF, 0xFEFF),
                    UnicodeRange::from_u32(0xFFFD, 0xFFFD),
                ]
            },
        ),
    ]
}

/// Inter as a variable font, weights 100 to 900. [`head`](crate::head) loads it.
pub const INTER: Font = font!(FAMILY, faces());
