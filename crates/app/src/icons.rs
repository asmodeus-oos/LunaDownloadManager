//! Toolbar icons: Lucide outlines (ISC licence, assets/icons/lucide/, v1.63.0) composed into
//! IDM 6.42's toolbar roles, stroked with the two-tone diagonal gradients measured from IDM's
//! "Neon" toolbar (tools/pixel/gradients.py). IDM's own images are never used (D-001).

use egui::{Image, ImageSource};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Icon {
    AddUrl,
    /// Add Torrent (ours, M15): Lucide "magnet".
    AddTorrent,
    Resume,
    Stop,
    StopAll,
    Delete,
    DeleteCompleted,
    Options,
    Scheduler,
    StartQueue,
    StopQueue,
    /// Download complete's drag handle: drag the file out (IDM's hand-and-arrows grip). Windows
    /// only, as the handle is (dialogs.rs).
    #[cfg_attr(not(windows), allow(dead_code))]
    DragOut,
    /// Options page headers (IDM draws its own artwork there; ours are Lucide outlines).
    PageGeneral,
    PageFileTypes,
    PageSaveTo,
    PageDownloads,
    PageConnection,
    PageProxy,
    PageLogins,
    PageSounds,
    PageRegional,
    PageArchives,
    PageRemote,
    /// The first-start helpers window (ours): yt-dlp finds a page's video,
    HelperVideo,
    /// ffmpeg joins its picture and sound,
    HelperMerge,
    /// and both are checked before they run.
    HelperChecked,
}

impl Icon {
    /// SVG body (24 × 24 user space, Lucide path data).
    fn body(self) -> &'static str {
        match self {
            // Modern clean plus for Add URL (mister_download_manager_0_5_0.html).
            Icon::AddUrl => r#"<line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>"#,
            Icon::AddTorrent => concat!(
                r#"<path d="m6 15-4-4 6.75-6.77a7.79 7.79 0 0 1 11 11L13 22l-4-4 6.39-6.36a2.14 2.14 0 0 0-3-3L6 15"/>"#,
                r#"<path d="m5 8 4 4"/><path d="m12 15 4 4"/>"#,
            ),
            Icon::Resume => {
                r#"<circle cx="12" cy="12" r="9"/><polygon points="10 8 16 12 10 16 10 8" fill="currentColor" stroke="none"/>"#
            }
            Icon::Stop => {
                r#"<circle cx="12" cy="12" r="9"/><rect x="9" y="9" width="6" height="6" rx="1" fill="currentColor" stroke="none"/>"#
            }
            Icon::StopAll => concat!(
                r#"<path d="M18 11V6a2 2 0 0 0-2-2a2 2 0 0 0-2 2"/>"#,
                r#"<path d="M14 10V4a2 2 0 0 0-2-2a2 2 0 0 0-2 2v2"/>"#,
                r#"<path d="M10 10.5V6a2 2 0 0 0-2-2a2 2 0 0 0-2 2v8"/>"#,
                r#"<path d="M18 8a2 2 0 1 1 4 0v6a8 8 0 0 1-8 8h-2c-2.8 0-4.5-.86-5.99-2.34l-3.6-3.6a2 2 0 0 1 2.83-2.82L7 15"/>"#,
            ),
            Icon::Delete => concat!(
                r#"<polyline points="3 6 5 6 21 6"/>"#,
                r#"<path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>"#,
                r#"<line x1="10" y1="11" x2="10" y2="17"/><line x1="14" y1="11" x2="14" y2="17"/>"#,
            ),
            Icon::DeleteCompleted => concat!(
                r#"<path d="M3 6h18"/>"#,
                r#"<path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/>"#,
                r#"<path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/>"#,
                r#"<path d="m9 14 2 2 4-4"/>"#,
            ),
            Icon::Options => concat!(
                r#"<circle cx="12" cy="12" r="3"/>"#,
                r#"<path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>"#,
            ),
            Icon::Scheduler => concat!(
                r#"<circle cx="12" cy="13" r="8"/>"#,
                r#"<path d="M12 9v4l2 2"/>"#,
                r#"<path d="M5 3 2 6"/>"#,
                r#"<path d="m22 6-3-3"/>"#,
            ),
            Icon::StartQueue => concat!(
                r#"<rect x="3" y="3" width="14" height="14" rx="2"/>"#,
                r#"<path d="M10 7v6"/>"#,
                r#"<path d="m7 10 3 3 3-3"/>"#,
                r#"<path d="M21 8v11a2 2 0 0 1-2 2H8"/>"#,
            ),
            Icon::StopQueue => concat!(
                r#"<rect x="3" y="3" width="14" height="14" rx="2"/>"#,
                r#"<rect x="8" y="8" width="4" height="4" rx="0.5" fill="currentColor" stroke="none"/>"#,
                r#"<path d="M21 8v11a2 2 0 0 1-2 2H8"/>"#,
            ),
            // Lucide "move": four arrows.
            Icon::DragOut => concat!(
                r#"<path d="M12 2v20"/><path d="m15 19-3 3-3-3"/><path d="m19 9 3 3-3 3"/>"#,
                r#"<path d="M2 12h20"/><path d="m5 9-3 3 3 3"/><path d="m9 5 3-3 3 3"/>"#,
            ),
            // Lucide "globe".
            Icon::PageGeneral => concat!(
                r#"<circle cx="12" cy="12" r="10"/><path d="M2 12h20"/>"#,
                r#"<path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/>"#,
            ),
            // Lucide "file-type".
            Icon::PageFileTypes => concat!(
                r#"<path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"/>"#,
                r#"<path d="M14 2v4a2 2 0 0 0 2 2h4"/><path d="M9 13v-1h6v1"/><path d="M12 12v6"/><path d="M11 18h2"/>"#,
            ),
            // Lucide "folder-open".
            Icon::PageSaveTo => {
                r#"<path d="m6 14 1.5-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.54 6a2 2 0 0 1-1.95 1.5H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H18a2 2 0 0 1 2 2v2"/>"#
            }
            Icon::PageDownloads => {
                r#"<circle cx="12" cy="12" r="10"/><path d="M12 7v10"/><path d="m8 13 4 4 4-4"/>"#
            }
            // Lucide "network".
            Icon::PageConnection => concat!(
                r#"<rect x="16" y="16" width="6" height="6" rx="1"/><rect x="2" y="16" width="6" height="6" rx="1"/>"#,
                r#"<rect x="9" y="2" width="6" height="6" rx="1"/>"#,
                r#"<path d="M5 16v-3a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v3"/><path d="M12 12V8"/>"#,
            ),
            // Lucide "shield".
            Icon::PageProxy => {
                r#"<path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/>"#
            }
            // Lucide "key-round".
            Icon::PageLogins => concat!(
                r#"<path d="M2.586 17.414A2 2 0 0 0 2 18.828V21a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h1a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h.172a2 2 0 0 0 1.414-.586l.814-.814a6.5 6.5 0 1 0-4-4z"/>"#,
                r#"<circle cx="16.5" cy="7.5" r=".5"/>"#,
            ),
            // Lucide "volume-2".
            // Lucide "calendar-clock".
            Icon::PageRegional => concat!(
                r#"<path d="M16 14v2.2l1.6 1"/><path d="M16 2v3"/><path d="M8 2v3"/>"#,
                r#"<path d="M21 7.338V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h2.338"/>"#,
                r#"<path d="M3 9h5.859"/><circle cx="16" cy="16" r="6"/>"#,
            ),
            // Lucide "package".
            Icon::PageArchives => concat!(
                r#"<path d="M11 21.73a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73z"/>"#,
                r#"<path d="M12 22V12"/><path d="m3.3 7 7.703 4.734a2 2 0 0 0 1.994 0L20.7 7"/><path d="m7.5 4.27 9 5.15"/>"#,
            ),
            // Lucide "smartphone" (Options → Remote, #141).
            Icon::PageRemote => {
                r#"<rect width="14" height="20" x="5" y="2" rx="2" ry="2"/><path d="M12 18h.01"/>"#
            }
            // A screen with a play mark.
            Icon::HelperVideo => {
                r#"<rect x="2" y="4" width="20" height="16" rx="3"/><path d="M10 8.5v7l6-3.5z"/>"#
            }
            // Two streams (picture, sound) joining into one file.
            Icon::HelperMerge => concat!(
                r#"<path d="M2 6h5c2.5 0 3.5 1.5 4.5 3.5L13 12h9"/>"#,
                r#"<path d="M2 18h5c2.5 0 3.5-1.5 4.5-3.5"/><path d="m18 8 4 4-4 4"/>"#,
            ),
            // Lucide "shield" with a check.
            Icon::HelperChecked => concat!(
                r#"<path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/>"#,
                r#"<path d="m9 12 2 2 4-4"/>"#,
            ),
            Icon::PageSounds => concat!(
                r#"<path d="M11 4.702a.705.705 0 0 0-1.203-.498L6.413 7.587A1.4 1.4 0 0 1 5.416 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.416a1.4 1.4 0 0 1 .997.413l3.383 3.384A.705.705 0 0 0 11 19.298z"/>"#,
                r#"<path d="M16 9a5 5 0 0 1 0 6"/><path d="M19.364 18.364a9 9 0 0 0 0-12.728"/>"#,
            ),
        }
    }

    /// The enabled gradient, top-left → bottom-right (measured from IDM 6.42).
    fn gradient(self) -> (&'static str, &'static str) {
        match self {
            Icon::AddUrl => ("#9ad9b0", "#7fb4c0"),
            // The Torrent pill's purple.
            Icon::AddTorrent => ("#b07ad9", "#7d5fd0"),
            Icon::Resume | Icon::Stop | Icon::StopAll | Icon::Delete => ("#e2e2e2", "#b4b4b4"),
            Icon::DeleteCompleted => ("#df6a9c", "#8180c0"),
            Icon::Options => ("#7883c2", "#9b84d0"),
            Icon::Scheduler => ("#a986d2", "#c0957a"),
            Icon::StartQueue => ("#93b6f2", "#90c6b6"),
            Icon::StopQueue => ("#a2c2f2", "#c4688f"),
            // Ours: IDM's grip has orange arrows.
            Icon::DragOut => ("#f5a623", "#d9661c"),
            Icon::PageGeneral | Icon::PageConnection => ("#3d8ee6", "#2bb3a3"),
            Icon::PageFileTypes | Icon::PageDownloads => ("#3fa864", "#2f7fcf"),
            Icon::PageSaveTo => ("#e6a817", "#c77a12"),
            Icon::PageProxy => ("#5b6fd6", "#8a5fd0"),
            Icon::PageLogins => ("#d9a514", "#9a7b2e"),
            Icon::PageSounds => ("#8a5fd0", "#d05f9a"),
            Icon::PageRegional => ("#2f7fcf", "#3fa864"),
            Icon::PageArchives => ("#c77a12", "#8a5fd0"),
            Icon::PageRemote => ("#3d8ee6", "#5b6fd6"),
            Icon::HelperVideo => ("#df6a9c", "#b07ad9"),
            Icon::HelperMerge => ("#3d8ee6", "#2bb3a3"),
            Icon::HelperChecked => ("#3fa864", "#2f7fcf"),
        }
    }

    fn name(self) -> &'static str {
        match self {
            Icon::AddUrl => "add-url",
            Icon::AddTorrent => "add-torrent",
            Icon::Resume => "resume",
            Icon::Stop => "stop",
            Icon::StopAll => "stop-all",
            Icon::Delete => "delete",
            Icon::DeleteCompleted => "delete-completed",
            Icon::Options => "options",
            Icon::Scheduler => "scheduler",
            Icon::StartQueue => "start-queue",
            Icon::StopQueue => "stop-queue",
            Icon::DragOut => "drag-out",
            Icon::PageGeneral => "page-general",
            Icon::PageFileTypes => "page-file-types",
            Icon::PageSaveTo => "page-save-to",
            Icon::PageDownloads => "page-downloads",
            Icon::PageConnection => "page-connection",
            Icon::PageProxy => "page-proxy",
            Icon::PageLogins => "page-logins",
            Icon::PageSounds => "page-sounds",
            Icon::PageRegional => "page-regional",
            Icon::PageArchives => "page-archives",
            Icon::PageRemote => "page-remote",
            Icon::HelperVideo => "helper-video",
            Icon::HelperMerge => "helper-merge",
            Icon::HelperChecked => "helper-checked",
        }
    }

    /// The ink's two stops, top-left to bottom-right: enabled, its gradient, one colour or
    /// the accent as the theme's icon style says; disabled, the theme's flat grey (as IDM).
    /// Official Microsoft Fluent UI System Icon paths (24 Regular).
    fn fluent_body(self) -> &'static str {
        match self {
            Icon::AddUrl => r#"<path d="M11.25 5a.75.75 0 0 1 .75.75v5.5h5.5a.75.75 0 0 1 0 1.5H12v5.5a.75.75 0 0 1-1.5 0V12.75H5a.75.75 0 0 1 0-1.5h5.5V5.75c0-.41.34-.75.75-.75Z" fill="currentColor"/>"#,
            Icon::AddTorrent => concat!(
                r#"<path d="M6 4.75a.75.75 0 0 1 .75-.75h1.5a.75.75 0 0 1 .75.75v4.5a.75.75 0 0 1-.75.75h-1.5A.75.75 0 0 1 6 9.25V4.75Zm9 0a.75.75 0 0 1 .75-.75h1.5a.75.75 0 0 1 .75.75v4.5a.75.75 0 0 1-.75.75h-1.5a.75.75 0 0 1-.75-.75V4.75ZM4.5 12a7.5 7.5 0 0 0 15 0v-1.25a.75.75 0 0 0-1.5 0V12a6 6 0 1 1-12 0v-1.25a.75.75 0 0 0-1.5 0V12Z" fill="currentColor"/>"#,
            ),
            Icon::Resume => r#"<path d="M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20ZM3.5 12a8.5 8.5 0 1 1 17 0 8.5 8.5 0 0 1-17 0Zm6.25-3.92a.75.75 0 0 0-1.15.64v6.56a.75.75 0 0 0 1.15.64l5.47-3.28a.75.75 0 0 0 0-1.28L9.75 8.08Z" fill="currentColor"/>"#,
            Icon::Stop => r#"<path d="M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20ZM3.5 12a8.5 8.5 0 1 1 17 0 8.5 8.5 0 0 1-17 0ZM9 8.75c0-.41.34-.75.75-.75h4.5c.41 0 .75.34.75.75v4.5a.75.75 0 0 1-.75.75h-4.5a.75.75 0 0 1-.75-.75v-4.5Z" fill="currentColor"/>"#,
            Icon::StopAll => r#"<path d="M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20ZM3.5 12a8.5 8.5 0 1 1 17 0 8.5 8.5 0 0 1-17 0ZM8.75 8a.75.75 0 0 0-.75.75v6.5c0 .41.34.75.75.75h1.5a.75.75 0 0 0 .75-.75v-6.5a.75.75 0 0 0-.75-.75h-1.5Zm4.5 0a.75.75 0 0 0-.75.75v6.5c0 .41.34.75.75.75h1.5a.75.75 0 0 0 .75-.75v-6.5a.75.75 0 0 0-.75-.75h-1.5Z" fill="currentColor"/>"#,
            Icon::Delete => concat!(
                r#"<path d="M10 2.5a.75.75 0 0 0-.75.75V4H5.25a.75.75 0 0 0 0 1.5h.8l.9 12.63A2.75 2.75 0 0 0 9.69 20.75h4.62a2.75 2.75 0 0 0 2.74-2.62l.9-12.63h.8a.75.75 0 0 0 0-1.5h-4V3.25A.75.75 0 0 0 14 2.5h-4ZM13.25 4h-2.5V3.75h2.5V4Zm-4.8 2.5h7.1l-.88 12.4a1.25 1.25 0 0 1-1.25 1.1H9.69a1.25 1.25 0 0 1-1.25-1.1L7.56 6.5Zm2.69 3.5a.75.75 0 0 1 .75.75v5.5a.75.75 0 0 1-1.5 0v-5.5a.75.75 0 0 1 .75-.75Zm3 0a.75.75 0 0 1 .75.75v5.5a.75.75 0 0 1-1.5 0v-5.5a.75.75 0 0 1 .75-.75Z" fill="currentColor"/>"#,
            ),
            Icon::DeleteCompleted => concat!(
                r#"<path d="M10 2.5a.75.75 0 0 0-.75.75V4H5.25a.75.75 0 0 0 0 1.5h.8l.9 12.63A2.75 2.75 0 0 0 9.69 20.75h4.62a2.75 2.75 0 0 0 2.74-2.62l.33-4.63a.75.75 0 0 0-1.5-.1l-.32 4.62a1.25 1.25 0 0 1-1.25 1.23H9.69a1.25 1.25 0 0 1-1.25-1.23L7.56 6.5h7.94a.75.75 0 0 0 0-1.5h-1.25V3.25A.75.75 0 0 0 13.5 2.5h-3.5ZM17.15 9.15a.75.75 0 0 1 1.06 0l1.77 1.77 1.77-1.77a.75.75 0 1 1 1.06 1.06l-1.77 1.77 1.77 1.77a.75.75 0 0 1-1.06 1.06L20 13.06l-1.77 1.77a.75.75 0 0 1-1.06-1.06l1.77-1.77-1.77-1.77a.75.75 0 0 1 0-1.06Z" fill="currentColor"/>"#,
            ),
            Icon::Options => concat!(
                r#"<path d="M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6Zm0-1.5a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3ZM10.05 3a1.75 1.75 0 0 0-1.72 1.44l-.22 1.23a6.83 6.83 0 0 0-1.07.62l-1.12-.56a1.75 1.75 0 0 0-2.22.56l-1.4 2.42a1.75 1.75 0 0 0 .42 2.26l.98.78a6.97 6.97 0 0 0 0 1.24l-.98.78a1.75 1.75 0 0 0-.42 2.26l1.4 2.42c.48.83 1.52 1.15 2.22.56l1.12-.56c.33.24.69.45 1.07.62l.22 1.23A1.75 1.75 0 0 0 10.05 21h3.9a1.75 1.75 0 0 0 1.72-1.44l.22-1.23c.38-.17.74-.38 1.07-.62l1.12.56c.7.35 1.74.27 2.22-.56l1.4-2.42a1.75 1.75 0 0 0-.42-2.26l-.98-.78a6.97 6.97 0 0 0 0-1.24l.98-.78a1.75 1.75 0 0 0 .42-2.26l-1.4-2.42a1.75 1.75 0 0 0-2.22-.56l-1.12.56a6.83 6.83 0 0 0-1.07-.62l-.22-1.23A1.75 1.75 0 0 0 13.95 3h-3.9Zm-.25 1.72c.07-.4.41-.72.82-.72h3.9c.41 0 .75.31.82.72l.24 1.37a.75.75 0 0 0 .54.58c.47.16.91.39 1.31.68a.75.75 0 0 0 .78.07l1.24-.62a.25.25 0 0 1 .32.08l1.4 2.42a.25.25 0 0 1-.06.32l-1.1.88a.75.75 0 0 0-.25.75c.05.5.05 1 0 1.5a.75.75 0 0 0 .25.75l1.1.88a.25.25 0 0 1 .06.32l-1.4 2.42a.25.25 0 0 1-.32.08l-1.24-.62a.75.75 0 0 0-.78.07 5.46 5.46 0 0 1-1.31.68.75.75 0 0 0-.54.58l-.24 1.37a.83.83 0 0 1-.82.72h-3.9a.83.83 0 0 1-.82-.72l-.24-1.37a.75.75 0 0 0-.54-.58 5.46 5.46 0 0 1-1.31-.68.75.75 0 0 0-.78-.07l-1.24.62a.25.25 0 0 1-.32-.08l-1.4-2.42a.25.25 0 0 1 .06-.32l1.1-.88a.75.75 0 0 0 .25-.75 5.6 5.6 0 0 1 0-1.5.75.75 0 0 0-.25-.75l-1.1-.88a.25.25 0 0 1-.06-.32l1.4-2.42a.25.25 0 0 1 .32-.08l1.24.62a.75.75 0 0 0 .78-.07c.4-.29.84-.52 1.31-.68a.75.75 0 0 0 .54-.58l.24-1.37Z" fill="currentColor"/>"#,
            ),
            Icon::Scheduler => r#"<path d="M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20ZM3.5 12a8.5 8.5 0 1 1 17 0 8.5 8.5 0 0 1-17 0Zm8.5-5.25a.75.75 0 0 0-.75.75v5c0 .41.34.75.75.75h4a.75.75 0 0 0 0-1.5h-3.25V7.5a.75.75 0 0 0-.75-.75Z" fill="currentColor"/>"#,
            Icon::StartQueue => r#"<path d="M11.25 3.75a.75.75 0 0 1 1.5 0v10.69l3.72-3.72a.75.75 0 1 1 1.06 1.06l-5 5a.75.75 0 0 1-1.06 0l-5-5a.75.75 0 1 1 1.06-1.06l3.72 3.72V3.75Zm-6.5 14.5a.75.75 0 0 0 0 1.5h14.5a.75.75 0 0 0 0-1.5H4.75Z" fill="currentColor"/>"#,
            Icon::StopQueue => r#"<path d="M6.5 5.25a.75.75 0 0 1 .75-.75h2.5a.75.75 0 0 1 .75.75v13.5a.75.75 0 0 1-.75.75H7.25a.75.75 0 0 1-.75-.75V5.25Zm7 0a.75.75 0 0 1 .75-.75h2.5a.75.75 0 0 1 .75.75v13.5a.75.75 0 0 1-.75.75H14.25a.75.75 0 0 1-.75-.75V5.25Z" fill="currentColor"/>"#,
            _ => self.body(),
        }
    }

    /// Stops (colour A, colour B) for linear gradient (classic mode) or monochromatic ink.
    fn stops(self, enabled: bool) -> (String, String) {
        use crate::palette::{IconStyle, to_hex};
        let c = crate::theme::colors();
        let is_modern = crate::theme::metrics().button_radius > 4.0;
        if !enabled {
            if is_modern {
                let dis = if crate::theme::dark() { "#64748b" } else { "#94a3b8" };
                return (dis.to_owned(), dis.to_owned());
            }
            return (to_hex(c.icon_disabled), to_hex(c.icon_disabled_end));
        }
        if is_modern {
            let ink = if crate::theme::dark() { "#f1f5f9" } else { "#1e293b" };
            return (ink.to_owned(), ink.to_owned());
        }
        match crate::theme::current().icons {
            IconStyle::Color => {
                let (a, b) = self.gradient();
                (a.to_owned(), b.to_owned())
            }
            IconStyle::Mono => (to_hex(c.text), to_hex(c.text)),
            IconStyle::Accent => (to_hex(c.accent), to_hex(c.accent)),
        }
    }

    /// The SVG for this icon, inked from `a` to `b`.
    fn svg(self, a: &str, b: &str) -> String {
        let is_modern = crate::theme::metrics().button_radius > 4.0;
        if is_modern {
            format!(
                r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" color="{a}" fill="none">{}</svg>"#,
                self.fluent_body()
            )
        } else {
            format!(
                r#"<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="url(#g)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><defs><linearGradient id="g" gradientUnits="userSpaceOnUse" x1="2" y1="2" x2="22" y2="22"><stop offset="0" stop-color="{a}"/><stop offset="1" stop-color="{b}"/></linearGradient></defs>{}</svg>"#,
                self.body()
            )
        }
    }

    /// The theme icon set's name for this icon (`crate::themeicons`), for the ones a set can
    /// replace: the toolbar's.
    fn need(self) -> Option<&'static str> {
        Some(match self {
            Icon::AddUrl => "tb-add-url",
            Icon::AddTorrent => "tb-add-torrent",
            Icon::Resume => "tb-resume",
            Icon::Stop => "tb-stop",
            Icon::StopAll => "tb-stop-all",
            Icon::Delete => "tb-delete",
            Icon::DeleteCompleted => "tb-delete-completed",
            Icon::Options => "tb-options",
            Icon::Scheduler => "tb-scheduler",
            Icon::StartQueue => "tb-start-queue",
            Icon::StopQueue => "tb-stop-queue",
            _ => return None,
        })
    }

    /// The icon, enabled or greyed: the theme's own where its icon set has one (a one-colour
    /// icon in the text colour), else the classic outline.
    pub fn image(self, enabled: bool) -> Image<'static> {
        let ink = crate::themeicons::ink(enabled);
        if let Some((src, tint)) = self
            .need()
            .and_then(|need| crate::themeicons::source(need, 32.0, ink, enabled))
        {
            return Image::new(src).tint(tint);
        }
        Image::new(self.source(enabled))
    }

    /// The icon as an image source (fileicons.rs shows Add Torrent's magnet for a .torrent).
    /// Made once per icon and ink (a theme's change makes new ones): drawing it again each frame
    /// allocates nothing.
    pub fn source(self, enabled: bool) -> ImageSource<'static> {
        use crate::palette::IconStyle;
        use std::cell::RefCell;
        use std::collections::HashMap;
        type Key = (Icon, bool, IconStyle, [u8; 4], [u8; 4]);
        thread_local! {
            static MADE: RefCell<HashMap<Key, ImageSource<'static>>> = RefCell::default();
        }
        let c = crate::theme::colors();
        let style = crate::theme::current().icons;
        let (x, y) = match (enabled, style) {
            (false, _) => (c.icon_disabled, c.icon_disabled_end),
            (true, IconStyle::Color) => (egui::Color32::TRANSPARENT, egui::Color32::TRANSPARENT),
            (true, IconStyle::Mono) => (c.text, c.text),
            (true, IconStyle::Accent) => (c.accent, c.accent),
        };
        let key = (self, enabled, style, x.to_array(), y.to_array());
        MADE.with(|made| {
            made.borrow_mut()
                .entry(key)
                .or_insert_with(|| {
                    let (a, b) = self.stops(enabled);
                    // Keyed by its ink too: another theme's icon is another texture. Kept for
                    // the program's life, as its texture is (a few per theme).
                    let uri: &'static str = Box::leak(
                        format!(
                            "bytes://mdm/toolbar/{}-{}-{}{}.svg",
                            self.name(),
                            if enabled { "on" } else { "off" },
                            a.trim_start_matches('#'),
                            b.trim_start_matches('#'),
                        )
                        .into_boxed_str(),
                    );
                    ImageSource::Bytes {
                        uri: uri.into(),
                        bytes: egui::load::Bytes::Shared(self.svg(&a, &b).into_bytes().into()),
                    }
                })
                .clone()
        })
    }
}
