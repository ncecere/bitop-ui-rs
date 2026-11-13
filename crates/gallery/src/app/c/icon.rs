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
                icon(name: Icon::Bookmark)
                icon(name: Icon::RotateCcw)
                icon(name: Icon::RotateCw)
                icon(name: Icon::Circle)
                icon(name: Icon::ListTree)
                icon(name: Icon::CircleX)
                icon(name: Icon::ShieldQuestion)
                icon(name: Icon::Wrench)
                icon(name: Icon::Bot)
                icon(name: Icon::CircleSlash)
                icon(name: Icon::MessageCircleQuestion)
                icon(name: Icon::Pause)
                icon(name: Icon::Play)
                icon(name: Icon::Volume1)
                icon(name: Icon::Volume2)
                icon(name: Icon::VolumeX)
                icon(name: Icon::Mic)
                icon(name: Icon::AudioLines)
                icon(name: Icon::Package)
                icon(name: Icon::Eye)
                icon(name: Icon::EyeOff)
                icon(name: Icon::GitCommitHorizontal)
                icon(name: Icon::CircleMinus)
                icon(name: Icon::LoaderCircle)
                icon(name: Icon::FileBraces)
                icon(name: Icon::FileCode)
                icon(name: Icon::FileImage)
                icon(name: Icon::Folder)
                icon(name: Icon::FolderOpen)
                icon(name: Icon::Eraser)
                icon(name: Icon::SquareTerminal)
                icon(name: Icon::ArrowLeft)
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
