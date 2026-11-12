// Behaviour for bitop-ui components that HTML and CSS cannot express.
//
// Each component's script is one block below. Everything is delegated from the document
// and keyed on roles and data attributes, so markup that the server adds later works
// without registration.

// What scripts on the page may call, such as bitopUi.toast().
window.bitopUi ??= {};
