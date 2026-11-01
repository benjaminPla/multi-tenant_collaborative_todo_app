# Frontend

Date: 2026-11-01

## Options

**React, library-free (chosen)**
- React because it's the framework I know best; Vue or Svelte would serve the app equally well.
- Runtime deps: `react` and `react-dom` only. No UI kit, router, data-fetching or CSS library.
- Plain semantic HTML elements; native drag and drop for the board.
- `styles.css` holds the design tokens (colors, fonts, spacing, borders) and shared primitives (buttons, fields, tables, headings). No hardcoded values outside variables.
- Each component or view with styles lives in its own folder next to its `.css`; component-only variables are declared on its root class.
- API calls go through one `fetch` wrapper on `/api`, proxied to the backend by Vite in dev and nginx in Docker, so the session cookie stays same-origin.

**Component library, e.g. MUI, Radix, shadcn (rejected)**
- Faster accessible widgets, but adds dependencies and a styling system to learn for a few tables and forms.

**Data-fetching library, e.g. TanStack Query (rejected)**
- Caching and retries aren't needed at this size; two small hooks (`useLoad`, `useAction`) cover loading, errors and reloads.

## Consequences

- Small bundle and dependency surface; nothing to keep up to date beyond React and Vite.
- Accessibility is hand-rolled: native drag and drop has no keyboard alternative yet.
- No client router, so views have no URLs or deep links.
- Class names are global; BEM-style prefixes (`.kanban__column`, `.sidebar__link`) avoid collisions instead of CSS Modules.
