## 2024-09-08 - Keyboard Navigation Foundations
**Learning:** Adding a "Skip to main content" link paired with proper `focus-visible` styles (`focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500 focus-visible:ring-offset-2`) makes keyboard navigation significantly better in this app. Ensure the main content area has `tabindex="-1"` and `focus:outline-none` so it receives focus seamlessly without showing an ugly ring around the entire main region.
**Action:** Use this skip link and focus ring pattern as standard for any new major layout or custom interactive component built in this repo.

## 2024-09-11 - Informative Empty States
**Learning:** Using a plain text "No data found" for empty states in panels makes the UI feel broken or unpolished. Adding a subtle icon (e.g., from Lucide), a bold title, and secondary text explaining *why* it's empty or *how* to populate it significantly improves the perception of the interface and guides the user.
**Action:** For list or data views that can be empty, always use an empty state pattern: a muted icon (text-gray-400), a primary title (text-sm font-medium text-gray-900), and a descriptive subtitle (text-sm text-gray-500) stacked vertically.
