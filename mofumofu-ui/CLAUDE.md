# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is **mofumofu-ui**, a Svelte 5 component library for a social media application called "mofu". The project is structured as a SvelteKit library that can be packaged and distributed, with a showcase/demo app in the routes directory.

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
  - `auth/` - Authentication APIs (signin, signup, OAuth)
  - `user/` - User management APIs
  - `post/` - Post/content APIs
  - `follow/` - Social following APIs
  - `error/` - Error handling with mapped error types
  - `config.ts` - API configuration using `PUBLIC_API_URL` environment variable
  - `private.ts` and `public.ts` - Authenticated and public API clients

- **Components** (`src/lib/components/`) - Reusable Svelte 5 components:
  - `navbar/` - Navigation with scroll-based visibility
  - `post/` - Post display components (PostCard, PostList) with skeleton states
  - `search/` - Search interface components
  - `settings/` - User settings panels with image upload/crop functionality
  - `ui/` - Low-level UI components (button, input, modal, etc.)
  - `modal/` - Image crop modal with canvas manipulation

- **Hooks** (`src/lib/hooks/`) - Custom Svelte 5 hooks using runes:
  - `useNavbarScroll.svelte.ts` - Navbar visibility on scroll with performance optimization
  - `useInfiniteScroll.svelte.ts` - Infinite scrolling functionality
  - `useResizable.svelte.ts` - Resizable component behavior
  - `useTextareaToolbar.svelte.ts` - Rich text editing toolbar

- **Stores** (`src/lib/stores/`) - Svelte 5 state management:
  - `auth.svelte.ts` - Authentication state with localStorage persistence
  - `settings.svelte.ts` - User settings management
  - `settings/personal.svelte.ts` - Personal information state

- **Utils** (`src/lib/utils/`) - Utility functions:
  - `utils.ts` - TailwindCSS class merging (`cn` function) and TypeScript helpers
  - `imagecrop.ts` - Image processing and cropping utilities

- **Schemas** (`src/lib/schemas/`) - Validation schemas using Valibot
- **OAuth** (`src/lib/oauth/`) - OAuth configuration and flows

### Application Routes (`src/routes/`)

- `(main)/` - Main application layout with post feed
- `account/` - Authentication pages (signin, signup, forgot-password, verify-email, OAuth callbacks)
- `profile/[slug]/` - User profile pages with server-side data loading
- `settings/` - User settings interface
- `write/` - Content creation interface

### Key Technologies & Patterns

- **Svelte 5** with runes (`$state`, `$props`, `$derived`) for reactive state
- **SvelteKit** for routing, SSR, and server-side data loading
- **TailwindCSS 4** with custom animations via `tw-animate-css`
- **TypeScript** with strict configuration
- **ky** for HTTP requests with error handling
- **mode-watcher** for theme switching
- **Cloudflare** adapter for edge deployment
- **Valibot** for schema validation
- **bits-ui** for accessible component primitives

### Component Architecture

- Components use Svelte 5 `$props()` destructuring pattern
- Skeleton loading states are implemented via derived reactive variables (`$derived`)
- Complex components are split into sub-components (e.g., PostCard → PostCardImage, PostCardContent, PostCardFooter)
- Performance optimizations include `requestAnimationFrame` for scroll handlers

### State Management Patterns

- Svelte 5 runes (`$state`) for local component state
- Stores for global state with explicit getters/setters
- Authentication tokens stored in localStorage with browser detection
- Settings state management with nested store structure

### API Integration

- Separate public/private API clients with automatic authentication
- Error handling with mapped error types and user-friendly messages
- OAuth integration for GitHub and Google authentication
- RESTful API design with versioned endpoints (`v0/`)

### Library Export System

Components must be explicitly exported in `src/lib/index.ts` to be available when the library is installed as a package. The file is currently empty and needs exports added as components are finalized.
