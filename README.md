# UltraCrew (Working Title)

An offline-first, dynamic crewing command center for ultra-marathons and multi-day endurance events.

---

## The Problem

During a 100-mile or 24-hour ultra-marathon it can be helpful to make it easy for the crew. Current solutions

1. **Google Sheets:** Spreadsheets are difficult to navigate on mobile devices.
2. **Static pace charts break down when plans fail:** Standard race planners don't adjust downstream aid station arrival times when a runner deviates from planned paces.
3. **GPS trackers are cumbersome:** Separate satellite trackers are expensive and is another carry item. Cell phones might not have signal.
4. **Third-party APIs are restrictive:** Relying on live Garmin or Strava can be used for tracking when signal is present but only solves some of the crew needs for planning.

I want a very low friction signup and use for the crew.

---

## Project Goal

Build a web-native **Crew Logistics Dashboard** that operates reliably offline, dynamically recalculates downstream aid station ETAs based on real-time check-ins, and seamlessly synchronizes state across multiple users when network connectivity is available.

---

## System Architecture

The application uses an **Offline-First / Stale-While-Reconnecting** paradigm. The UI and database live entirely inside the user's browser, communicating with a lightweight Rust backend when online.

```
┌─────────────────────────────────────────────────────────────┐
│                       MOBILE BROWSER                        │
│                                                             │
│  ┌──────────────────────┐        ┌───────────────────────┐  │
│  │   Svelte SPA (UI)    │ <────> │ Service Worker Cache  │  │
│  └──────────┬───────────┘        └───────────────────────┘  │
│             │                                               │
│             ▼                                               │
│  ┌──────────────────────┐                                   │
│  │ IndexedDB            │ <─── Local State / Outbox Queue   │
│  └──────────┬───────────┘                                   │
└─────────────┼───────────────────────────────────────────────┘
              │
              │ (Background Sync via REST API when online)
              ▼
┌─────────────────────────────────────────────────────────────┐
│                      RUST BACKEND                           │
│                                                             │
│  ┌──────────────────────┐        ┌───────────────────────┐  │
│  │   Axum Web Server    │ <────> │    SQLite (SQLx)      │  │
│  └──────────────────────┘        └───────────────────────┘  │
└─────────────────────────────────────────────────────────────┘

```

### Key Technical Decisions

- **Web-Native PWA (No App Stores):** Accessible via standard mobile browsers (Safari, Chrome, Firefox) without native app store friction or installation prompts.
- **Service Worker Caching:** Serves the app bundle locally from device storage, allowing the UI to load with zero network connectivity.
- **IndexedDB Local Storage:** All timestamp logs, runner notes, and route calculations are committed locally first before attempting cloud sync.
- **Monolithic Rust Backend:** Powered by Axum and SQLite for low-resource hosting footprint and simple file-based database backups.

---

## Key Features

### 1. Dynamic "Fade Rate" Prediction Engine

- Calculates a real-time **Performance Ratio** based on actual time elapsed versus planned pace at the most recent aid station.
- Applies terrain-aware adjustments (considering distance and elevation gain/loss) to update all future downstream arrival ETAs instantly.

### 2. Crew Operations UX

- **High-Contrast "Fat-Finger" UI:** Designed with large tap targets for use in low-light, wet, or freezing conditions.
- **One-Tap Check-In/Out:** Tracks exact timestamps for arrival (`IN`), departure (`OUT`), and dwell time at each station.
- **Nutrition & Medical Logs:** Allows crews to record what food was accepted/rejected and log physical issues (e.g., "Left knee tight, drank ginger ale") so the next crew vehicle can prepare ahead of time.

### 3. Resilient Data Sync

- **Automatic Reconnection Queue:** Writes all actions to an outbox mutation queue in IndexedDB. When `navigator.onLine` fires, queued items are pushed to the server.
- **Conflict Resolution:** Employs a deterministic _Last-Write-Wins_ policy anchored on aid station sequence identifiers.

---

## Tech Stack

- **Frontend:** Svelte (SPA mode), Vite, Tailwind CSS, IndexedDB.
- **Backend:** Rust, Axum, Tokio async runtime, SQLx.
- **Database:** SQLite.
- **Deployment:** Single $5 VPS instance, containerized via Docker.

---

## Data Model Overview

### Frontend (IndexedDB Schema)

- `waypoints`: `id`, `race_id`, `name`, `distance_mile`, `elevation_gain_ft`, `is_crew_accessible`
- `runner_logs`: `id`, `waypoint_id`, `time_in`, `time_out`, `notes`, `synced`
- `sync_queue`: `id`, `payload`, `timestamp`

### Backend (SQLite Schema)

- `races`: `id`, `name`, `gpx_data`
- `waypoints`: `id`, `race_id`, `sequence`, `distance`, `elevation`
- `timestamps`: `id`, `runner_id`, `waypoint_id`, `time_in`, `time_out`, `updated_at`

---

## Initial Development Roadmap

- [ ] **Phase 1: Local Prediction Core**
- Set up Svelte SPA with Tailwind.
- Build the dynamic pace recalculation module in TypeScript.

- [ ] **Phase 2: Crew Mobile Interface**
- Build high-visibility check-in dashboard.
- Implement offline Service Worker caching via Vite.
- Create manual SMS-fallback URI generator.

- [ ] **Phase 3: Rust Sync Server**
- Spin up Axum API with SQLx + SQLite.
- Implement `POST /api/sync` payload consumption and outbox clearing logic.
- Field test live during summer crewing events.

## Helpful Dev commands:

Sample .env file:

```
DATABASE_URL="sqlite://app.db"
APP_ENV="development"
```
