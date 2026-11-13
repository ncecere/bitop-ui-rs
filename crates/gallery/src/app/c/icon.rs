use bitop_ui::components::icon::{Icon, icon};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::app::kit::{example, page_header};

#[page]
async fn show(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        page_header(slug: "icon")
        example(
            title: "The set",
            <div
                style="display: flex; flex-wrap: wrap; gap: var(--space-4); font-size: 1.25rem"
            >
                icon(name: Icon::Info)
                icon(name: Icon::CircleCheck)
                icon(name: Icon::TriangleAlert)
                icon(name: Icon::CircleAlert)
                icon(name: Icon::ArrowUpRight)
                icon(name: Icon::X)
                icon(name: Icon::Check)
                icon(name: Icon::ChevronDown)
                icon(name: Icon::ChevronRight)
                icon(name: Icon::Search)
                icon(name: Icon::Inbox)
                icon(name: Icon::Minus)
                icon(name: Icon::Plus)
                icon(name: Icon::Copy)
                icon(name: Icon::ChevronLeft)
                icon(name: Icon::Sun)
                icon(name: Icon::Moon)
                icon(name: Icon::CalendarDays)
                icon(name: Icon::Upload)
                icon(name: Icon::Ellipsis)
                icon(name: Icon::GripVertical)
                icon(name: Icon::ArrowDownRight)
                icon(name: Icon::ArrowRight)
                icon(name: Icon::ArrowUp)
                icon(name: Icon::ArrowDown)
                icon(name: Icon::ChevronsUpDown)
                icon(name: Icon::Columns3)
                icon(name: Icon::File)
                icon(name: Icon::FileText)
                icon(name: Icon::Image)
                icon(name: Icon::Paperclip)
                icon(name: Icon::Square)
                icon(name: Icon::BookOpen)
                icon(name: Icon::Brain)
                icon(name: Icon::ImageOff)
                icon(name: Icon::ThumbsUp)
                icon(name: Icon::ThumbsDown)
                icon(name: Icon::RefreshCw)
                icon(name: Icon::MessageSquare)
                icon(name: Icon::ExternalLink)
                icon(name: Icon::Globe)
                icon(name: Icon::SquarePen)
                icon(name: Icon::BookOpenText)
            </div>
        )
        example(
            title: "Takes the text colour",
            <span style="color: var(--color-danger-text); font-size: 1.5rem">
                icon(name: Icon::CircleAlert)
            </span>
        )
    })
}
