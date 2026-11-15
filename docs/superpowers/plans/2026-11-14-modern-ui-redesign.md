# Mister Download Manager 0.5.0 Modern UI Redesign Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:subagent-driven-development` or `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Transform Mister Download Manager desktop application (`crates/app`) from the legacy IDM 6.42 retro Win32 clone aesthetic into a modern, refined, high-craft desktop experience exactly matching the architecture, tokens, and components defined in `mister_download_manager_0_5_0.html`.

**Architecture:** Modern egui theme and widget system replacing vintage hardcoded Tahoma/Win32 pixel metrics with rounded cards, Inter-based typographic scale, slate neutral palette, emerald primary accents, SVG glyph toolbar, floating rounded popovers, dual-card workspace (padded sidebar + padded data table), collapsible find bar, and modern modal overlays.

**Tech Stack:** Rust 2024, `egui` 0.31+, `eframe`, `wgpu` / Direct3D 12, SVG glyph renderers.

**Spec Reference:** `C:\Users\vlad\Downloads\RivaDownloadManager\mister_download_manager_0_5_0.html`

---

## Global Design Constraints (Verbatim from Spec HTML)

- **Palette**:
  - App Window Background: Slate-50/95 (`#f8fafc` with backdrop blur)
  - Card Containers: White (`#ffffff`), border `slate-200/80` (`#e2e8f0cc`), subtle shadow
  - Workspace Canvas: Slate-100/70 (`#f1f5f9b3`)
  - Accent Primary: Emerald-500 (`#10b981`), Emerald-600 (`#059669`), tinted light fill `emerald-50` (`#ecfdf5`)
  - Accent Secondary: Indigo-500 (`#6366f1`) for Torrents/Magnets, Amber-500 (`#f59e0b`) for paused/stopping, Rose-500 (`#f43f5e`) for delete/errors, Sky-500 (`#0ea5e9`) for media/resume, Teal-500 (`#14b8a6`) for queues
  - Inactive/Text: Slate-800 (`#1e293b`) headings, Slate-700 (`#334155`) body, Slate-500 (`#64748b`) muted, Slate-400 (`#94a3b8`) icons/chevrons
- **Typography**: Inter / system modern sans-serif scale:
  - 10px uppercase tracking-wider (labels/badges)
  - 11px toolbar captions & category badges
  - 12px table text, sidebar rows, status bar, and menu items
  - 13px headings & modal subtitles
  - 14px modal titles
- **Corner Radii**:
  - Window & Major Cards: 16px (`rounded-2xl`)
  - Buttons, Popovers & Toolbar Items: 12px (`rounded-xl`)
  - Row file glyphs & small badges: 8px (`rounded-lg`)
  - Pill badges (selection, count, modern UI tag): 9999px (`rounded-full`)

---

## Multi-Phase Execution Plan

### Phase 1: Modern Token & Styling Foundation (`palette.rs`, `theme.rs`, `winui.rs`)
- [ ] **Task 1.1: Design Tokens**: Introduce modern Slate and Emerald token definitions into `crates/app/src/palette.rs` and `crates/app/src/theme.rs`.
- [ ] **Task 1.2: Modern Typography & Font Scale**: Replace legacy Tahoma metrics with modern Inter/system UI sans font scale and line-heights in `crates/app/src/winui.rs`.
- [ ] **Task 1.3: Modern Control & Card Primitives**: Implement rounded card containers, 12px rounded buttons, pill badges, and soft drop shadows in `crates/app/src/winui.rs`.

### Phase 2: Modern Window Chrome & Top Menubar (`chrome.rs`, `app.rs`)
- [ ] **Task 2.1: Modern Window Title Bar**: Implement the modern header with emerald gradient glyph icon, "Mister Download Manager 0.5.0" title, "Modern UI" pill badge, and custom window control buttons.
- [ ] **Task 2.2: Top Menubar Overhaul**: Re-implement Tasks, File, Downloads, View, Help with hover activation and active state pill styling.
- [ ] **Task 2.3: Modern Floating Popover Menus**: Re-implement popups with rounded-2xl containers, subtle borders, colored glyphs, check indicators, and keyboard shortcut chips.

### Phase 3: Modern Glyph Toolbar (`chrome.rs`, `app.rs`)
- [ ] **Task 3.1: Clean Glyph Buttons**: Replace retro XP toolbar with clean glyph cards:
  - "Add URL" button with emerald icon box (eliminating legacy green square border).
  - "Add Torrent" split-button with magnet icon and dedicated dropdown chevron trigger.
  - "Resume", "Stop", "Stop All" glyph buttons with disabled states.
  - "Delete", "Delete Completed".
  - "Options", "Scheduler".
  - "Start Queue" and "Stop Queue" split-buttons.
- [ ] **Task 3.2: Dropdown Handlers**: Connect magnet dropdown and queue start/stop dropdown menus.

### Phase 4: Modern Split Workspace — Left Sidebar Card (`chrome.rs`, `app.rs`)
- [ ] **Task 4.1: Sidebar Card Container**: Implement the 256px wide white card with rounded-2xl corners, padding, and subtle border.
- [ ] **Task 4.2: Category Tree Navigation**: Expandable tree for "All Downloads", "Unfinished", "Finished", "Queues", with colorful folder icons and count pills.
- [ ] **Task 4.3: Subcategory File Type Icons**: Add modern colored badges for Compressed, Documents, Music, Programs, Video, Torrents.
- [ ] **Task 4.4: Quick Speed Limiter Status Pill**: Embed the live speed limiter widget at the bottom of the sidebar.

### Phase 5: Modern Split Workspace — Right Data Table Card (`list.rs`, `app.rs`)
- [ ] **Task 5.1: Table Card Container & Collapsible Find Bar**: Add Ctrl+F filter banner with input and "Find Next" action.
- [ ] **Task 5.2: Sticky Modern Table Header**: Styled header with uppercase column labels and sort chevrons (Source, File Name, Q, Size, Status, Time Left, Transfer Rate, Last Try, Added).
- [ ] **Task 5.3: Modern Download Rows**:
  - Source pill badges (Emerald "Web", Indigo "Torrent").
  - File-type icon badge + truncated name.
  - Smooth rounded progress bars with percentages for active downloads.
  - Hover highlights and selection state with emerald-tinted background.
  - Multi-select and context menu integration.

### Phase 6: Bottom Status Bar & Interactive Overlays (`app.rs`, `dialogs.rs`)
- [ ] **Task 6.1: Modern Status Bar**: Total downloads count, selection indicator pill, pulsing green queue indicator, and real-time aggregate transfer rate.
- [ ] **Task 6.2: Modern Modal Dialogs**: Update Add URL, Magnet, Speed Limiter, and Scheduler dialogs with backdrop blur, rounded-2xl cards, and modern inputs.
- [ ] **Task 6.3: Floating Drop Target**: Add toggleable floating drop target widget with download glyph.

### Phase 7: Verification & Build Validation
- [ ] **Task 7.1: Build Compilation**: Verify clean compilation under `cargo check -p mdm-app`.
- [ ] **Task 7.2: Integration & Interactive Review**: Verify all views, toggles, menus, and layout responsiveness.
