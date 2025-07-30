# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is **mofu-ui**, a Svelte 5 component library for a social media application called "mofu". The project is structured as a SvelteKit library that can be packaged and distributed, with a showcase/demo app in the routes directory.

## Development Commands

- `pnpm dev` - Start development server for the showcase app
- `pnpm build` - Build both the library and production app (includes prepack step)
- `pnpm preview` - Preview the production build
- `pnpm check` - Run Svelte type checking
- `pnpm check:watch` - Run type checking in watch mode
- `pnpm fmt` - Format code with Prettier
- `pnpm lint` - Check code formatting with Prettier
- `pnpm prepack` - Build the library package (svelte-kit sync, svelte-package, publint)

Note: This project uses **pnpm** as the package manager.

## Architecture

### Library Structure (`src/lib/`)

- **API Layer** (`src/lib/api/`) - HTTP client using `ky` with organized endpoints:
  - `auth/` - Authentication APIs (signin, signup, etc.)
  - `user/` - User management APIs
  - `post/` - Post/content APIs
  - `follow/` - Social following APIs
  - `error/` - Error handling and mapping
  - `config.ts` - API configuration using `PUBLIC_API_URL` environment variable

- **Components** (`src/lib/components/`) - Reusable Svelte components:
  - `navbar/` - Navigation components with scroll behavior
  - `post/` - Post display components (PostCard)
  - `search/` - Search interface components
  - Includes skeleton loading states for components

- **Stores** (`src/lib/stores/`) - Svelte 5 state management:
  - `auth.svelte.ts` - Authentication state using runes (`$state`)
  - Token stored in memory only, not persisted

- **Types** (`src/lib/types/`) - TypeScript type definitions
- **Utils** (`src/lib/utils.ts`) - Utility functions

### Application Routes (`src/routes/`)

- `(main)/` - Main application layout and homepage
- `account/` - Authentication pages (signin, signup, forgot-password, verify-email)

### Key Technologies

- **Svelte 5** with runes (`$state`, `$props`)
- **SvelteKit** for routing and SSR
- **TailwindCSS 4** for styling with custom animations (`tw-animate-css`)
- **TypeScript** for type safety
- **ky** for HTTP requests
- **mode-watcher** for dark/light theme switching
- **Cloudflare** adapter for deployment

### Styling Approach

- Uses TailwindCSS with custom font (`font-pretendard`)
- Dark mode enabled by default via `mode-watcher`
- Component-specific skeleton states for loading UX

### State Management

- Authentication uses Svelte 5 runes pattern in `authStore`
- Token management is memory-only (no persistence)
- Custom hooks for UI behavior (e.g., `useNavbarScroll`)

## Library Export

The `src/lib/index.ts` file is currently empty - components must be explicitly exported here to be available when the library is installed as a package.
