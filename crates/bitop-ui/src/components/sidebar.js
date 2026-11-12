// Sidebar: in the layout on a wide screen, a popover drawer on a narrow one.

const wide = matchMedia("(min-width: 48rem)");

function syncSidebarToggles() {
  for (const toggle of document.querySelectorAll("[data-sidebar-toggle]")) {
    const layout = toggle.closest("[data-sidebar-layout]");
    // On a wide screen the popover state says nothing, so state it here. On a narrow
    // one the browser reports the popover's state for the button.
    if (wide.matches && layout) {
      toggle.setAttribute("aria-expanded", String(!layout.hasAttribute("data-collapsed")));
    } else {
      toggle.removeAttribute("aria-expanded");
    }
  }
}

document.addEventListener("click", (event) => {
  const toggle = event.target instanceof Element && event.target.closest("[data-sidebar-toggle]");
  const layout = toggle && toggle.closest("[data-sidebar-layout]");
  if (!layout || !wide.matches) return;
  // Stop the button from opening the sidebar as a popover.
  event.preventDefault();
  const collapsed = layout.toggleAttribute("data-collapsed");
  document.cookie = `bitop_sidebar=${collapsed ? "collapsed" : "expanded"}; path=/; max-age=31536000; samesite=lax`;
  syncSidebarToggles();
});

wide.addEventListener("change", () => {
  // A drawer left open would stay in the top layer after the screen widens.
  for (const sidebar of document.querySelectorAll("[data-sidebar-layout] > [popover]")) {
    if (sidebar.matches(":popover-open")) sidebar.hidePopover();
  }
  syncSidebarToggles();
});

syncSidebarToggles();
