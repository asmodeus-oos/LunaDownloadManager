//! The main window's chrome painted to IDM 6.42's pixels (notes/ui-spec/main-window.md,
//! notes/ui-spec/menus.md): menu bar, popup menus, toolbar, Categories tree, shell icons.
//!
//! Our client area also holds the menu bar (IDM's is non-client), so every main-window
//! spec y is shifted by `MENU_H` = 20. Main-window spec "baseline" is origin + ascent (the row
//! under the glyphs); `winui::text_font` takes the last ink row, i.e. spec baseline − 1.

use egui::{Color32, Pos2, Rect, Stroke, Ui, pos2, vec2};

use crate::icons::Icon;
use crate::scrollbar;
use crate::winui::{self, text_font, text_width_font};

/// Height of our menu bar band incl. its #f0f0f0 bottom line (IDM: non-client y −20…−1).
pub const MENU_H: f32 = 20.0;
pub const MENU_MODERN_H: f32 = 46.0;

pub fn is_modern() -> bool {
    crate::theme::metrics().button_radius > 4.0
}

pub fn menu_h() -> f32 {
    if is_modern() {
        MENU_MODERN_H
    } else {
        MENU_H
    }
}

pub fn slot_h() -> f32 {
    if is_modern() {
        34.0
    } else {
        22.0
    }
}

pub fn sep_h() -> f32 {
    if is_modern() {
        12.0
    } else {
        9.0
    }
}

const fn rgb(r: u8, g: u8, b: u8) -> Color32 {
    Color32::from_rgb(r, g, b)
}
use crate::theme::themed;
// The active theme's colours (crate::palette, #45).
themed! {
    pub WINDOW_BG => window;
    MENUBAR_BG => menubar;
    /// The bar's last row (light: #f2f2f2).
    MENUBAR_BOTTOM => menubar_bottom;
    /// The open menu's item. Light: #f9f9f9, the popup's own background (captured 2026-10-30).
    MENUBAR_OPEN => menubar_open;
    /// An item under the pointer, no menu open (captured 2026-10-30): dark #505050 from 1 px
    /// below the bar's top; light #f5f5f5, 1 px inside the bar's top and bottom.
    MENUBAR_HOT => menubar_hot;
    MENUBAR_LINE => menubar_line;
    POPUP_BG => menu;
    // The native menu window's frame (Windows draws menus in their own window), and every
    // in-window popup's (popup_surface).
    POPUP_BORDER => menu_border;
    POPUP_SEP => menu_sep;
    /// Light: #f0f0f0, rounded, captured 2026-10-30.
    POPUP_HOVER => menu_hot;
    POPUP_ITEM => menu_item;
    TEXT => menu_text;
    /// Bullets and submenu arrows (light: dark cores #1a1a1a / #282828).
    GLYPH => menu_glyph;
    TEXT_DISABLED_MENU => menu_text_disabled;
    TB_DISABLED => toolbar_text_disabled;
    /// A drop-down's arrow (dark: light on the dark band, where #282828 could not be seen).
    TB_ARROW => toolbar_arrow;
    CAT_FRAME => tree_frame;
    TREE_BG => tree;
    TREE_LINES => tree_lines;
    /// The tree's chevrons (dark look).
    TREE_EXPANDER => tree_expander;
}

use crate::winui::{fill, px};

// ---- shell icons ---------------------------------------------------------------------------

/// What a 16 × 16 icon shows: a file type (by extension) or a folder.
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum IconKey {
    Ext(String),
    /// 32 × 32 (Download File Info).
    ExtLarge(String),
    /// A FamFamFam Silk icon (CC BY 2.5, assets/icons/silk/), by name.
    Silk(&'static str),
    /// The theme icon set's icon for a need (`crate::themeicons`), else this Silk icon.
    Themed(&'static str, &'static str),
}

/// The embedded Silk icons (XP-era look, like IDM's tree icons).
fn silk(name: &str) -> Option<egui::ImageSource<'static>> {
    macro_rules! icons {
        ($($n:literal),*) => {
            match name {
                $($n => Some(egui::include_image!(concat!("../assets/icons/silk/", $n, ".png"))),)*
                _ => None,
            }
        };
    }
    icons!(
        "folder",
        "folder_go",
        "folder_page",
        "page_white_zip",
        "page_white",
        "music",
        "application_xp",
        "film",
        "tick",
        "page_white_stack",
        "arrow_down",
        "arrow_up",
        "cross",
        "folder_link",
        "folder_delete"
    )
}

/// The small icons drawn by key: Silk's (the Categories tree, the Scheduler's queues) and the
/// file types', which fileicons.rs draws (the system's own).
#[derive(Default)]
pub struct ShellIcons;

impl ShellIcons {
    /// Paint the icon for `key` with its top-left at English point `min` (16 × 16, the large ones
    /// 32): placed where that is on screen, not mirrored. A generic page if unavailable.
    pub fn paint(&mut self, ui: &Ui, key: IconKey, min: Pos2) {
        self.paint_in(ui, ui.painter(), key, min);
    }

    /// `paint` through `painter`, so the icon keeps to its clip (a list scrolled sideways must
    /// not draw its icons over the pane beside it).
    pub fn paint_in(&mut self, ui: &Ui, painter: &egui::Painter, key: IconKey, min: Pos2) {
        let side = if matches!(key, IconKey::ExtLarge(_)) {
            32.0
        } else {
            16.0
        };
        let rc = winui::rect(Rect::from_min_size(min, vec2(side, side)));
        match &key {
            IconKey::Silk(name) => {
                if let Some(src) = silk(name) {
                    paint_image(ui, painter, src, rc);
                }
            }
            IconKey::Themed(need, name) => {
                if !crate::themeicons::paint(ui, painter, need, rc, true)
                    && let Some(src) = silk(name)
                {
                    paint_image(ui, painter, src, rc);
                }
            }
            IconKey::Ext(e) | IconKey::ExtLarge(e) => {
                // The key holds the extension alone (empty when there is none).
                let name = if e.is_empty() {
                    "file".to_string()
                } else {
                    format!("x.{e}")
                };
                crate::fileicons::paint(ui, painter, crate::fileicons::Kind::File(&name), rc);
            }
        }
    }
}

/// An image source drawn in `rc` through `painter` (its clip), pixel-sharp.
pub fn paint_image(ui: &Ui, painter: &egui::Painter, src: egui::ImageSource<'static>, rc: Rect) {
    let img = egui::Image::new(src).texture_options(egui::TextureOptions::NEAREST);
    if let Ok(egui::load::TexturePoll::Ready { texture }) = img.load_for_size(ui.ctx(), rc.size()) {
        painter.image(
            texture.id,
            rc,
            Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
            Color32::WHITE,
        );
    }
}

// ---- menu bar --------------------------------------------------------------------------------

/// Top-level menus: label, item rect x and width, text x (measured, "Ta" kerned by the font):
/// the standard Win32 bar (text at item.x + 7, width = text + 14; light-main.md), in both modes
/// (user decision 2026-11-01: dark mode only changes colours, never sizes or places; IDM's dark
/// bar was wider).
const MENUS: [(&str, f32, f32, f32); 5] = [
    ("&Tasks", 0.0, 42.0, 7.0),
    ("&File", 42.0, 32.0, 49.0),
    ("&Downloads", 74.0, 73.0, 81.0),
    ("&View", 147.0, 39.0, 154.0),
    ("&Help", 186.0, 39.0, 193.0),
];

/// The menu bar in the interface language: English keeps the measured rects; a translation is
/// laid out by the same rule (Win32's: text at item.x + 7, width = text + 14) from the translated label's width: GDI's, or the
/// width it is drawn at when wider (a script drawn in a fallback font, such as Japanese).
pub fn menus(p: &egui::Painter) -> Vec<(&'static str, f32, f32, f32)> {
    let table = &MENUS;
    let translated: Vec<&'static str> = table
        .iter()
        .map(|(l, ..)| crate::i18n::tc("menubar", l))
        .collect();
    if is_modern() {
        let mut x = 12.0;
        return translated
            .into_iter()
            .map(|label| {
                let clean = plain(label).0;
                let tw = text_width_font(p, winui::segoe(), &clean).ceil();
                let w = (tw + 24.0).max(52.0);
                let tx = x + (w - tw) / 2.0;
                let item = (label, x, w, tx);
                x += w + 6.0;
                item
            })
            .collect();
    }
    if translated
        .iter()
        .zip(table.iter())
        .all(|(t, (l, ..))| t == l)
    {
        return table.to_vec();
    }
    let (pad, inset) = (14.0, 7.0);
    let mut x = 0.0;
    translated
        .into_iter()
        .map(|label| {
            let w = menu_width(p, &plain(label).0) + pad;
            let item = (label, x, w, x + inset);
            x += w;
            item
        })
        .collect()
}

/// Paint the menu bar; returns what happened to its titles this frame, for [`menubar_next`]: a
/// title clicked (or opened and shut by a screen reader), or the pointer over one (which only
/// moves to it from another open menu).
pub fn menu_bar(ui: &mut Ui, o: Pos2, width: f32, open: Option<usize>) -> Option<MenuBarEvent> {
    let p = ui.painter().clone();
    let h = menu_h();
    let modern = is_modern();
    let dark = crate::theme::dark();
    if modern {
        let bar_bg = if dark {
            Color32::from_rgb(15, 23, 42)
        } else {
            Color32::WHITE
        };
        fill(&p, o.x, o.y, o.x + width - 1.0, o.y + h - 1.0, bar_bg);
    } else {
        fill(
            &p,
            o.x,
            o.y,
            o.x + width - 1.0,
            o.y + h - 2.0,
            MENUBAR_BG(),
        );
        fill(
            &p,
            o.x,
            o.y + h - 2.0,
            o.x + width - 1.0,
            o.y + h - 2.0,
            MENUBAR_BOTTOM(),
        );
        fill(
            &p,
            o.x,
            o.y + h - 1.0,
            o.x + width - 1.0,
            o.y + h - 1.0,
            MENUBAR_LINE(),
        );
    }
    let mut hit = None;
    let bar = ui.interact(
        winui::rect(Rect::from_min_size(o, vec2(width, h))),
        egui::Id::unique("menubar-node"),
        egui::Sense::hover(),
    );
    winui::expose(&bar, egui::Role::MenuBar, "", true, None);
    let titles_ui = ui.new_child(egui::UiBuilder::new().accessibility_parent(bar.id));
    let released = ui.input(|x| x.pointer.any_released());
    for (i, (label, x, w, tx)) in menus(&p).iter().enumerate() {
        let rc = winui::rect(Rect::from_min_size(o + vec2(*x, 0.0), vec2(*w, h)));
        let resp = titles_ui.interact(rc, egui::Id::unique(("menubar", i)), winui::pointer_only());
        winui::expose(&resp, egui::Role::MenuItem, &plain(label).0, true, None);
        ui.ctx().accesskit_node_builder(resp.id, |n| {
            n.set_has_popup(egui::accesskit::HasPopup::Menu);
            n.set_expanded(open == Some(i));
        });
        if modern {
            let btn_h = 32.0;
            let btn_y = o.y + (h - btn_h) / 2.0;
            let btn_rc = winui::rect(Rect::from_min_size(pos2(o.x + *x, btn_y), vec2(*w, btn_h)));
            let (shown, letter) = plain(label);

            let (btn_bg, text_color) = if open == Some(i) {
                (
                    if dark {
                        Color32::from_rgba_unmultiplied(16, 185, 129, 45)
                    } else {
                        Color32::from_rgba_unmultiplied(16, 185, 129, 30)
                    },
                    if dark {
                        Color32::from_rgb(52, 211, 153)
                    } else {
                        Color32::from_rgb(4, 120, 87)
                    },
                )
            } else if resp.hovered() {
                (
                    if dark {
                        Color32::from_white_alpha(22)
                    } else {
                        Color32::from_black_alpha(15)
                    },
                    if dark {
                        Color32::from_rgb(248, 250, 252)
                    } else {
                        Color32::from_rgb(15, 23, 42)
                    },
                )
            } else {
                (
                    Color32::TRANSPARENT,
                    if dark {
                        Color32::from_rgb(203, 213, 225)
                    } else {
                        Color32::from_rgb(51, 65, 85)
                    },
                )
            };

            if btn_bg != Color32::TRANSPARENT {
                p.rect_filled(btn_rc, 8.0, btn_bg);
                if open == Some(i) {
                    p.rect_stroke(
                        btn_rc,
                        8.0,
                        Stroke::new(1.0, if dark { Color32::from_rgba_unmultiplied(52, 211, 153, 90) } else { Color32::from_rgba_unmultiplied(16, 185, 129, 90) }),
                        egui::StrokeKind::Inside,
                    );
                }
            }
            let text_baseline = btn_y + btn_h / 2.0 + 4.5;
            let drawn = text_font(
                &p,
                winui::segoe(),
                o.x + tx,
                text_baseline,
                &shown,
                text_color,
            );
            if cues(ui.ctx()) {
                let (x0, x1) =
                    winui::char_span(&p, &winui::segoe(), &shown, letter.unwrap_or(0), drawn);
                fill(&p, x0, text_baseline + 1.0, x1, text_baseline + 1.0, text_color);
            }
        } else {
            let r = crate::theme::metrics().control_radius.min(6.0);
            if open == Some(i) {
                let fill_rc = if r > 0.0 { rc.shrink2(vec2(2.0, 1.0)) } else { rc };
                p.rect_filled(fill_rc, r, MENUBAR_OPEN());
            } else if open.is_none() && resp.hovered() {
                let hot = if r > 0.0 {
                    rc.shrink2(vec2(2.0, 1.0))
                } else {
                    Rect::from_min_max(rc.min + vec2(0.0, 1.0), rc.max - vec2(0.0, 1.0))
                };
                p.rect_filled(hot, r, MENUBAR_HOT());
            }
            let (shown, letter) = plain(label);
            let drawn = text_font(&p, winui::segoe(), o.x + tx, o.y + 13.0, &shown, TEXT());
            if cues(ui.ctx()) {
                let (x0, x1) =
                    winui::char_span(&p, &winui::segoe(), &shown, letter.unwrap_or(0), drawn);
                fill(&p, x0, o.y + 15.0, x1, o.y + 15.0, TEXT());
            }
        }
        let (expand, collapse) = ui.input(|x| {
            (
                x.has_accesskit_action_request(resp.id, egui::accesskit::Action::Expand),
                x.has_accesskit_action_request(resp.id, egui::accesskit::Action::Collapse),
            )
        });
        if collapse && open == Some(i) {
            hit = Some(MenuBarEvent::Title { i, released: true });
        } else if resp.clicked() || (expand && open != Some(i)) {
            hit = Some(MenuBarEvent::Title { i, released });
        } else if resp.hovered() && open.is_some_and(|o| o != i) {
            hit = Some(MenuBarEvent::Hover(i));
        }
        if resp.clicked() || expand || collapse {
            ui.ctx().request_repaint();
        }
    }
    hit
}

/// The menu-bar title under `pos` (main-window coordinates), as [`menu_bar`] lays them out at
/// `o`: the first menu at the right in RTL.
pub fn menubar_title_at(p: &egui::Painter, o: Pos2, pos: Pos2) -> Option<usize> {
    let h = menu_h();
    menus(p).iter().position(|(_, x, w, _)| {
        winui::rect(Rect::from_min_size(o + vec2(*x, 0.0), vec2(*w, h))).contains(pos)
    })
}

/// What happened to the menu bar this frame, for [`menubar_next`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MenuBarEvent {
    /// Title `i` clicked (`released`: the click's release, or a screen reader's Collapse), or
    /// opened by a screen reader's Expand.
    Title { i: usize, released: bool },
    /// The pointer over title `i`, no click: while another menu is open, that one's menu opens
    /// instead (Windows' menu bars, and IDM's); otherwise nothing.
    Hover(usize),
    /// The open menu shut itself (Escape, a press outside it, the app losing the focus);
    /// `press_on_title`: the press that shut it went down on a title of the bar.
    Dismissed { press_on_title: bool },
    /// An item was chosen.
    Chosen,
    /// Left or Right, or the keyboard (Alt, F10, an access letter), opened menu `i`.
    Open(usize),
}

/// The menu bar's open menu after `event` (`open`: the one open before it).
///
/// A menu that shut itself is closed here too, unless the press that shut it went down on a
/// title: that press's release is the title's click, which shuts the menu (its own title) or
/// opens another one. Escape, or the app losing the focus, with the pointer resting on the bar
/// (where the click that opened the menu left it) used to leave the menu marked open with
/// nothing shown, so the next click on its title only "shut" it, and nothing opened.
///
/// The pointer alone never opens a menu: it only moves from an open one to another (#277). Once
/// a menu closes in any way (a command chosen, Escape, a click elsewhere, another window taking
/// the focus), the bar waits for a click.
pub fn menubar_next(open: Option<usize>, event: MenuBarEvent) -> Option<usize> {
    match event {
        MenuBarEvent::Hover(i) => open.map(|_| i),
        MenuBarEvent::Title { i, released } => {
            if open == Some(i) && released {
                None
            } else {
                Some(i)
            }
        }
        MenuBarEvent::Dismissed {
            press_on_title: true,
        } => open,
        MenuBarEvent::Dismissed {
            press_on_title: false,
        }
        | MenuBarEvent::Chosen => None,
        MenuBarEvent::Open(i) => Some(i),
    }
}

// ---- popup menus ---------------------------------------------------------------------------

#[derive(Clone)]
pub enum MenuEntry<C> {
    /// Label (with `&` before the mnemonic letter), command, enabled, checked.
    Item(&'static str, C, bool, bool),
    /// Like `Item`, but checked shows a bullet (Win32 radio-check items).
    Radio(&'static str, C, bool, bool),
    /// Label, enabled, children.
    Sub(&'static str, bool, Vec<MenuEntry<C>>),
    Sep,
}

/// A menu label made at run time (a queue's name): each distinct text is kept once for the
/// life of the app, so menus rebuilt every frame don't allocate or leak more than that.
pub fn label(text: &str) -> &'static str {
    use std::collections::HashSet;
    use std::sync::{Mutex, OnceLock};
    static LABELS: OnceLock<Mutex<HashSet<&'static str>>> = OnceLock::new();
    let mut set = LABELS.get_or_init(Default::default).lock().unwrap();
    if let Some(l) = set.get(text) {
        return l;
    }
    let l: &'static str = Box::leak(text.to_owned().into_boxed_str());
    set.insert(l);
    l
}


/// A menu title's access letter, lower case ("&Tasks": "t"; none marked: its first letter).
pub fn menu_letter(label: &str) -> String {
    let (shown, at) = plain(label);
    shown
        .chars()
        .nth(at.unwrap_or(0))
        .map(|c| c.to_lowercase().to_string())
        .unwrap_or_default()
}

fn plain(label: &str) -> (String, Option<usize>) {
    let mut out = String::new();
    let mut mnemonic = None;
    let mut chars = label.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '&' && chars.peek() == Some(&'&') {
            // "&&": an "&" in the text (a queue named "Tom & Jerry"), no access key.
            out.push(chars.next().unwrap());
        } else if c == '&' && chars.peek().is_some() {
            mnemonic = Some(out.chars().count());
            out.push(chars.next().unwrap());
        } else {
            out.push(c);
        }
    }
    (out, mnemonic)
}

/// The access key of `label`: the letter after its `&`, in lower case (None without one). An
/// open menu runs its first enabled item with the key pressed.
pub fn access_key(label: &str) -> Option<String> {
    let (text, at) = plain(label);
    at.and_then(|at| text.chars().nth(at))
        .map(|c| c.to_lowercase().to_string())
}

/// The menu bar's titles, in English (their translations: context "menubar").
#[cfg(test)]
pub fn bar_titles() -> Vec<&'static str> {
    MENUS.iter().map(|m| m.0).collect()
}

pub enum PopupResult<C> {
    Open,
    Chosen(C),
    Dismissed,
    /// Left or Right on a menu of the bar: open the menu before (-1) or after (+1) it.
    Neighbor(i32),
}

/// A key or key combination as written in the shortcuts list ("Ctrl+N", "Shift+Delete"), as
/// this system shows it: macOS's symbols (⌘N, ⇧⌦), elsewhere the translated key names.
pub fn key_label(spec: &'static str) -> String {
    if cfg!(target_os = "macos") {
        // Translated first ("Drag on blank space", "Home / End"...), then the Mac's symbols.
        crate::i18n::tc("key", spec)
            .replace("Ctrl+", "\u{2318}")
            .replace("Shift+", "\u{21e7}")
            .replace("Alt+", "\u{2325}")
            .replace("Delete", "\u{2326}")
            .replace("Enter", "\u{21a9}")
    } else {
        crate::i18n::tc("key", spec).to_owned()
    }
}

/// A menu label without the shortcut IDM writes into some labels ("Find (Ctrl-F)"), when the
/// menu shows the shortcut in its own column.
fn without_shortcut(text: &str) -> &str {
    let t = text.trim_end();
    if let Some(open) = t.rfind(" (")
        && t.ends_with(')')
    {
        let inner = &t[open + 2..t.len() - 1];
        let keyish = inner.len() <= 12
            && !inner.contains(' ')
            && (inner.contains('-') || inner.contains('+') || inner.starts_with('F'));
        if keyish {
            return &t[..open];
        }
    }
    t
}

/// Open a menu from the keyboard: its first item is highlighted.
pub fn start_keyboard(ctx: &egui::Context, id: &str) {
    show_cues(ctx);
    let kb = egui::Id::unique(("popup-sub", id)).with("kb");
    ctx.data_mut(|d| d.insert_temp::<(Option<usize>, Option<usize>)>(kb, (Some(usize::MAX), None)));
}

/// Keyboard cues on: the user reached for the keyboard (Alt, F10, a menu opened by keys), so
/// light mode underlines access letters from now on, the Windows rule (dark mode always does).
pub fn show_cues(ctx: &egui::Context) {
    ctx.data_mut(|d| d.insert_temp(egui::Id::unique("keyboard-cues"), true));
}

/// Whether access letters are underlined: always in dark mode, after [`show_cues`] in light.
pub fn cues(ctx: &egui::Context) -> bool {
    crate::theme::colors().access_keys_always
        || ctx.data(|d| {
            d.get_temp::<bool>(egui::Id::unique("keyboard-cues"))
                .unwrap_or(false)
        })
}

/// The next item of `level` from `from` in direction `dir`, skipping separators (wraps).
fn step<C>(level: &[MenuEntry<C>], from: Option<usize>, dir: i32) -> Option<usize> {
    let n = level.len() as i32;
    if n == 0 {
        return None;
    }
    let mut i = match from {
        Some(f) if f < level.len() => f as i32,
        _ if dir > 0 => -1,
        _ => n,
    };
    for _ in 0..n {
        i = (i + dir).rem_euclid(n);
        if !matches!(level[i as usize], MenuEntry::Sep) {
            return Some(i as usize);
        }
    }
    None
}

fn entry_label<C>(e: &MenuEntry<C>) -> Option<(&'static str, bool)> {
    match e {
        MenuEntry::Item(l, _, en, _) | MenuEntry::Radio(l, _, en, _) | MenuEntry::Sub(l, en, _) => {
            Some((l, *en))
        }
        MenuEntry::Sep => None,
    }
}

/// The keys an open menu takes from `i` (without Ctrl or Command: those are shortcuts).
fn menu_keys(i: &egui::InputState) -> Vec<egui::Key> {
    i.events
        .iter()
        .filter_map(|e| match e {
            egui::Event::Key {
                key,
                pressed: true,
                modifiers,
                ..
            } if !modifiers.command && !modifiers.ctrl => Some(*key),
            _ => None,
        })
        .collect()
}

/// Keys for an open menu (pressed in the main window or in the menu's own windows): Up and Down
/// move the highlight (Home / End to the first / last), Right opens a submenu or the next menu, Left closes a submenu or opens
/// the previous menu, Enter or an item's underlined letter runs it, Escape closes a submenu.
fn keyboard<C: Clone>(
    ctx: &egui::Context,
    entries: &[MenuEntry<C>],
    sub_key: egui::Id,
) -> Option<PopupResult<C>> {
    let kb_key = sub_key.with("kb");
    let mut keys = ctx.input(menu_keys);
    // Pressed in the menu's own windows: popup_level noted them as those windows drew.
    keys.extend(
        ctx.data_mut(|d| d.remove_temp::<Vec<egui::Key>>(sub_key.with("window-keys")))
            .unwrap_or_default(),
    );
    let (mut hl0, mut hl1): (Option<usize>, Option<usize>) =
        ctx.data(|d| d.get_temp(kb_key)).unwrap_or_default();
    if hl0 == Some(usize::MAX) {
        hl0 = step(entries, None, 1);
        ctx.data_mut(|d| d.insert_temp(kb_key, (hl0, hl1)));
    }
    if keys.is_empty() {
        return None;
    }
    // The keys are the menu's: the list underneath doesn't move too.
    ctx.input_mut(|i| {
        for k in &keys {
            i.consume_key(egui::Modifiers::NONE, *k);
        }
    });
    let mut open_sub = ctx.data(|d| d.get_temp::<usize>(sub_key));
    let mut out = None;
    for key in keys {
        let children: Option<&[MenuEntry<C>]> = open_sub.and_then(|i| match entries.get(i) {
            Some(MenuEntry::Sub(_, _, c)) => Some(c.as_slice()),
            _ => None,
        });
        let in_sub = children.is_some() && hl1.is_some();
        let level = if in_sub {
            children.unwrap_or(&[])
        } else {
            entries
        };
        let hl = if in_sub { hl1 } else { hl0 };
        let mut target = None;
        match key {
            egui::Key::ArrowDown | egui::Key::ArrowUp => {
                let n = step(level, hl, if key == egui::Key::ArrowDown { 1 } else { -1 });
                if in_sub { hl1 = n } else { hl0 = n }
            }
            // Home / End: the first / last item, as Windows' menus have them.
            egui::Key::Home | egui::Key::End => {
                let n = step(level, None, if key == egui::Key::Home { 1 } else { -1 });
                if in_sub { hl1 = n } else { hl0 = n }
            }
            egui::Key::Escape if in_sub => {
                open_sub = None;
                hl1 = None;
            }
            egui::Key::Escape => out = Some(PopupResult::Dismissed),
            egui::Key::ArrowLeft if in_sub => {
                open_sub = None;
                hl1 = None;
            }
            egui::Key::ArrowLeft => out = Some(PopupResult::Neighbor(-1)),
            egui::Key::ArrowRight => match hl.and_then(|h| level.get(h)) {
                Some(MenuEntry::Sub(_, true, c)) if !in_sub => {
                    open_sub = hl;
                    hl1 = step(c, None, 1);
                }
                _ => out = Some(PopupResult::Neighbor(1)),
            },
            egui::Key::Enter | egui::Key::Space => target = hl,
            other => {
                let name = other.name().to_lowercase();
                if name.chars().count() == 1 {
                    target = level.iter().position(|e| {
                        entry_label(e).is_some_and(|(l, en)| {
                            en && access_key(crate::i18n::tc("menu", l)).is_some_and(|k| k == name)
                        })
                    });
                }
            }
        }
        if let Some(t) = target {
            if in_sub {
                hl1 = Some(t)
            } else {
                hl0 = Some(t)
            }
            match level.get(t) {
                Some(MenuEntry::Item(_, c, true, _) | MenuEntry::Radio(_, c, true, _)) => {
                    out = Some(PopupResult::Chosen(c.clone()));
                }
                Some(MenuEntry::Sub(_, true, c)) if !in_sub => {
                    open_sub = Some(t);
                    hl1 = step(c, None, 1);
                }
                _ => {}
            }
        }
        if out.is_some() {
            break;
        }
    }
    ctx.data_mut(|d| {
        d.insert_temp(kb_key, (hl0, hl1));
        match open_sub {
            Some(i) => {
                d.insert_temp(sub_key, i);
            }
            None => {
                d.remove::<usize>(sub_key);
            }
        }
    });
    out
}

/// What a popup menu is: how its window says so to the window system (X11's window types) and,
/// on X11, whether it shares its window with others ([`menu_viewport`]).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MenuKind {
    /// A menu of the menu bar, and its submenus.
    Bar,
    /// A context menu: a right click, the Menu key, a toolbar button's drop-down.
    Context,
    /// A drop-down list's choices: a combo box's, a text box's history.
    List,
}

/// Show a popup menu with its top-left at `pos` (main-window client coordinates, on screen) —
/// its top-right in a right-to-left interface, where menus open to the left.
///
/// Like a Win32 menu, each level is its own borderless top-level window (an immediate egui
/// viewport): it can extend past the main window, flips to stay on the monitor, never takes
/// activation (so Escape still reaches the main window), and clicks on it never reach the
/// widgets underneath. Call every frame while open, from the main window's `ui`.
pub fn popup<C: Clone>(
    ctx: &egui::Context,
    kind: MenuKind,
    id: &str,
    pos: Pos2,
    entries: &[MenuEntry<C>],
    hint: &dyn Fn(&C) -> Option<&'static str>,
) -> PopupResult<C> {
    let sub_key = egui::Id::unique(("popup-sub", id));
    // The control that has the keys while the menu is open (a drop-down list, a text box): the
    // menu's windows never take them, so Escape reached its window, and egui let them go there
    // (Escape drops the focus). Escape closing the menu gives them back.
    let opener = sub_key.with("opener");
    let focused_now = ctx.memory(|m| m.focused());
    if let Some(f) = focused_now {
        ctx.data_mut(|d| d.insert_temp(opener, f));
    }
    let give_back = |ctx: &egui::Context| {
        let had = ctx.data_mut(|d| {
            let had = d.get_temp::<egui::Id>(opener);
            d.remove::<egui::Id>(opener);
            had
        });
        if let Some(had) = had
            && focused_now.is_none()
            && ctx.input(|i| i.key_pressed(egui::Key::Escape))
        {
            ctx.memory_mut(|m| m.request_focus(had));
        }
    };
    if let Some(r) = keyboard(ctx, entries, sub_key) {
        ctx.data_mut(|d| {
            d.remove::<usize>(sub_key);
            d.remove::<(Option<usize>, Option<usize>)>(sub_key.with("kb"));
        });
        give_back(ctx);
        return r;
    }
    let origin = ctx
        .input(|i| i.viewport().inner_rect)
        .map_or(Pos2::ZERO, |r| r.min);
    let mut result = None;
    // Set by the menu's windows as they draw (popup_level) when one of them has the focus.
    let window_focus = sub_key.with("window-focus");
    ctx.data_mut(|d| d.insert_temp(window_focus, false));
    popup_level(
        ctx,
        kind,
        id,
        origin + pos.to_vec2(),
        None,
        entries,
        sub_key,
        &mut result,
        0,
        hint,
    );
    if let Some(c) = result {
        ctx.data_mut(|d| {
            d.remove::<usize>(sub_key);
            d.remove::<(Option<usize>, Option<usize>)>(sub_key.with("kb"));
        });
        give_back(ctx);
        return PopupResult::Chosen(c);
    }
    // The menus are other windows: any press that reaches the main window, Escape, or the
    // app losing focus closes them (as Win32 menus do) — except on the frame the menu opened,
    // whose press may be the very click that opened it.
    let pass = ctx.cumulative_pass_nr();
    let shown = sub_key.with("shown");
    // Focus in the menu's own windows is still ours: on macOS and Linux a click in a menu window
    // gives that window the focus, and must not count as clicking away.
    let focused = ctx.input(|i| i.viewport().focused) == Some(true)
        || ctx.data(|d| d.get_temp::<bool>(window_focus).unwrap_or(false));
    // Away from our windows, the system may say who has it now: another program (#859).
    let elsewhere = !focused && another_program_in_front();
    // (first pass, last pass, the focus): focus is only *lost* if it was had, and then at once
    // when another program has it, else when no window of ours gets it back within FOCUS_GRACE
    // (#250).
    let now = ctx.input(|i| i.time);
    let (first, verdict) = ctx.data_mut(|d| {
        let (first, last, seen) = d.get_temp::<(u64, u64, MenuFocus)>(shown).unwrap_or((
            pass,
            pass,
            MenuFocus::default(),
        ));
        let (first, mut seen) = if last + 1 < pass {
            (pass, MenuFocus::default())
        } else {
            (first, seen)
        };
        let verdict = seen.pass(focused, elsewhere, now);
        d.insert_temp(shown, (first, pass, seen));
        (first == pass, verdict)
    });
    if let FocusVerdict::Away { wait } = verdict {
        // No event may come to look again (the pointer resting on the menu).
        ctx.request_repaint_after(std::time::Duration::from_secs_f64(wait));
    }
    let lost_focus = verdict == FocusVerdict::Left;
    if !first
        && (lost_focus
            || ctx.input(|i| i.key_pressed(egui::Key::Escape) || i.pointer.any_pressed()))
    {
        // Closed by a press: where it was, so the control that opened it doesn't take that
        // press's click (on release) as opening it again (winui::opener_click). Read before
        // the data lock (asking the context inside it deadlocks).
        let press = ctx.input(|i| i.pointer.press_origin().filter(|_| i.pointer.any_pressed()));
        let dismissed = dismissed_at_key(ctx);
        ctx.data_mut(|d| {
            d.remove::<usize>(sub_key);
            d.remove::<(Option<usize>, Option<usize>)>(sub_key.with("kb"));
            if let Some(at) = press {
                d.insert_temp(dismissed, at);
            }
        });
        give_back(ctx);
        return PopupResult::Dismissed;
    }
    PopupResult::Open
}

/// How long the focus may be away from every window of ours before an open menu counts as left,
/// when the system doesn't say that another program has it ([`another_program_in_front`]).
/// On Linux a window manager may give a menu's window the focus (KWin, sway, and GNOME under
/// Wayland give it to a submenu's as it opens). When that submenu closes (the pointer back on
/// another item of its parent), its window passes the focus on, and for a frame or more no
/// window of ours has it: counted as the app losing the focus, it closed the whole menu (#250).
const FOCUS_GRACE: f64 = 0.4;

/// Whether the system says another program's window is in front now: the click that took the
/// focus away landed outside MDM (the desktop, the taskbar, another program's window), and an
/// open menu closes on this pass, as Win32's own menus do. Waiting FOCUS_GRACE for it kept a
/// menu on the screen 0.4 s after such a click (#859): a menu's window never takes the focus on
/// Windows (WS_EX_NOACTIVATE), so there the grace only ever delayed the close.
fn another_program_in_front() -> bool {
    #[cfg(windows)]
    {
        crate::win32::foreground_is_another_program()
    }
    // Linux and macOS: a program isn't told whose window has the focus (Wayland never says; X11
    // and macOS would, through calls MDM doesn't make), so FOCUS_GRACE decides there, as before.
    #[cfg(not(windows))]
    {
        false
    }
}

/// Whether an open menu still has the focus: the main window's or one of the menu's windows'.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct MenuFocus {
    /// One of ours has had the focus since the menu opened.
    had: bool,
    /// Since when none of ours has had it (the app's clock, in seconds).
    away_since: Option<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum FocusVerdict {
    /// Ours, or never had (a menu opened while another app was in front).
    Here,
    /// Gone for less than FOCUS_GRACE: look again in `wait` seconds.
    Away { wait: f64 },
    /// Gone for FOCUS_GRACE: another app has it, the menu closes.
    Left,
}

impl MenuFocus {
    /// After a pass at `now`: `focused` is whether one of our windows had the focus in it, and
    /// `elsewhere` whether the system said another program has it.
    fn pass(&mut self, focused: bool, elsewhere: bool, now: f64) -> FocusVerdict {
        if focused {
            self.had = true;
            self.away_since = None;
            return FocusVerdict::Here;
        }
        if !self.had {
            return FocusVerdict::Here;
        }
        // Another program has it: no window of ours is about to take it back (#859).
        if elsewhere {
            return FocusVerdict::Left;
        }
        let since = *self.away_since.get_or_insert(now);
        let away = now - since;
        if away >= FOCUS_GRACE {
            FocusVerdict::Left
        } else {
            FocusVerdict::Away {
                wait: FOCUS_GRACE - away,
            }
        }
    }
}

/// Where the last press that closed a popup went down, in this window.
pub fn dismissed_at_key(ctx: &egui::Context) -> egui::Id {
    egui::Id::unique(("popup-dismissed-at", ctx.viewport_id()))
}

/// Open a menu from the keyboard with item `i` highlighted (a drop-down list on its choice).
pub fn start_at(ctx: &egui::Context, id: &str, i: usize) {
    show_cues(ctx);
    let kb = egui::Id::unique(("popup-sub", id)).with("kb");
    ctx.data_mut(|d| d.insert_temp::<(Option<usize>, Option<usize>)>(kb, (Some(i), None)));
}

/// Open a menu with its entry `i`'s submenu open and the submenu's first entry highlighted, as
/// the keyboard leaves it (`/show menu:torrent`, for screenshots).
pub fn open_sub_at(ctx: &egui::Context, id: &str, i: usize) {
    show_cues(ctx);
    let sub_key = egui::Id::unique(("popup-sub", id));
    ctx.data_mut(|d| {
        d.insert_temp(sub_key, i);
        d.insert_temp::<(Option<usize>, Option<usize>)>(sub_key.with("kb"), (Some(i), Some(0)));
    });
}

/// The surface of a popup drawn inside a window (the date picker's calendar, a column's filter
/// box): the menus' colours and border, rounded as Windows 11's flyouts (they had their own).
pub fn popup_surface(p: &egui::Painter, rc: egui::Rect) {
    let radius = crate::theme::metrics().flyout_radius;
    p.rect_filled(rc, radius, POPUP_BG());
    p.rect_stroke(
        rc,
        radius,
        egui::Stroke::new(1.0, POPUP_BORDER()),
        egui::StrokeKind::Inside,
    );
}

/// A menu label's width as Win32 sizes menus: GDI's whole-pixel width (egui's advances differ
/// by up to a pixel either way: IDM's menus are 236, 291 wide where egui gives 237, 290), or the
/// width it is drawn at when that is wider by more (a script drawn in a fallback font, such as
/// Japanese).
fn menu_width(p: &egui::Painter, label: &str) -> f32 {
    let drawn = text_width_font(p, winui::segoe(), label).ceil();
    #[cfg(windows)]
    {
        let gdi = crate::win32::menu_text_width(label) as f32;
        if drawn > gdi + 1.0 { drawn } else { gdi }
    }
    #[cfg(not(windows))]
    {
        drawn
    }
}

/// Size of a menu window: the check column (38), the widest label, then 30 to the edge (48 when an
/// item opens a submenu: its arrow), slots of 22 (separators 9), 3 px top and bottom. Fitted to
/// its texts, so they don't float in a wide window.
fn paint_menu_icon(p: &egui::Painter, label: &str, c: Pos2, enabled: bool) {
    let lower = label.to_lowercase();
    let dark = crate::theme::dark();
    let disabled_color = if dark {
        Color32::from_rgb(100, 116, 139)
    } else {
        Color32::from_rgb(148, 163, 184)
    };

    let col = |r: u8, g: u8, b: u8| -> Color32 {
        if enabled {
            Color32::from_rgb(r, g, b)
        } else {
            disabled_color
        }
    };

    if lower.contains("add new") || lower.contains("add url") {
        let ic = col(16, 185, 129);
        p.circle_stroke(c, 7.0, Stroke::new(1.6, ic));
        p.line_segment([pos2(c.x - 3.5, c.y), pos2(c.x + 3.5, c.y)], Stroke::new(1.8, ic));
        p.line_segment([pos2(c.x, c.y - 3.5), pos2(c.x, c.y + 3.5)], Stroke::new(1.8, ic));
    } else if lower.contains("open torrent") {
        let ic = col(99, 102, 241);
        let doc = Rect::from_center_size(c, vec2(12.0, 14.0));
        p.rect_stroke(doc, 2.5, Stroke::new(1.5, ic), egui::StrokeKind::Inside);
        p.line_segment([pos2(c.x, c.y - 2.5), pos2(c.x, c.y + 2.5)], Stroke::new(1.5, ic));
        p.line(vec![pos2(c.x - 2.5, c.y + 0.5), pos2(c.x, c.y + 3.0), pos2(c.x + 2.5, c.y + 0.5)], Stroke::new(1.5, ic));
    } else if lower.contains("magnet") {
        let ic = col(99, 102, 241);
        p.line(
            vec![
                pos2(c.x - 4.5, c.y - 4.5),
                pos2(c.x - 4.5, c.y + 1.0),
                pos2(c.x - 2.5, c.y + 4.5),
                pos2(c.x + 2.5, c.y + 4.5),
                pos2(c.x + 4.5, c.y + 1.0),
                pos2(c.x + 4.5, c.y - 4.5),
            ],
            Stroke::new(2.0, ic),
        );
        p.rect_filled(Rect::from_center_size(pos2(c.x - 4.5, c.y - 4.5), vec2(3.0, 2.0)), 0.5, ic);
        p.rect_filled(Rect::from_center_size(pos2(c.x + 4.5, c.y - 4.5), vec2(3.0, 2.0)), 0.5, ic);
    } else if lower.contains("torrent settings") || lower.contains("settings...") {
        let ic = col(100, 116, 139);
        p.line_segment([pos2(c.x - 3.5, c.y - 5.5), pos2(c.x - 3.5, c.y + 5.5)], Stroke::new(1.4, ic));
        p.line_segment([pos2(c.x + 3.5, c.y - 5.5), pos2(c.x + 3.5, c.y + 5.5)], Stroke::new(1.4, ic));
        p.circle_filled(pos2(c.x - 3.5, c.y - 1.5), 2.2, ic);
        p.circle_filled(pos2(c.x + 3.5, c.y + 1.5), 2.2, ic);
    } else if lower.contains("clipboard") {
        let ic = col(2, 132, 199);
        let board = Rect::from_center_size(pos2(c.x, c.y + 1.0), vec2(12.0, 13.0));
        p.rect_stroke(board, 2.5, Stroke::new(1.5, ic), egui::StrokeKind::Inside);
        p.rect_filled(Rect::from_center_size(pos2(c.x, c.y - 5.5), vec2(5.5, 3.0)), 1.0, ic);
        p.line(vec![pos2(c.x - 3.0, c.y + 1.0), pos2(c.x - 1.0, c.y + 3.0), pos2(c.x + 3.0, c.y - 1.0)], Stroke::new(1.6, ic));
    } else if lower.contains("batch") {
        let ic = col(2, 132, 199);
        p.rect_stroke(Rect::from_center_size(pos2(c.x + 2.5, c.y - 2.5), vec2(11.0, 11.0)), 2.5, Stroke::new(1.3, ic.gamma_multiply(0.6)), egui::StrokeKind::Inside);
        p.rect_stroke(Rect::from_center_size(pos2(c.x - 2.0, c.y + 2.0), vec2(11.0, 11.0)), 2.5, Stroke::new(1.5, ic), egui::StrokeKind::Inside);
        p.line_segment([pos2(c.x - 5.0, c.y + 2.0), pos2(c.x + 1.0, c.y + 2.0)], Stroke::new(1.5, ic));
    } else if lower.contains("video") {
        let ic = col(168, 85, 247);
        p.rect_stroke(Rect::from_center_size(pos2(c.x - 2.5, c.y), vec2(10.0, 8.5)), 2.0, Stroke::new(1.5, ic), egui::StrokeKind::Inside);
        p.add(egui::Shape::convex_polygon(
            vec![pos2(c.x + 3.0, c.y - 2.0), pos2(c.x + 6.5, c.y - 4.5), pos2(c.x + 6.5, c.y + 4.5), pos2(c.x + 3.0, c.y + 2.0)],
            ic,
            Stroke::NONE,
        ));
    } else if lower.contains("link") {
        let ic = col(13, 148, 136);
        p.line(vec![pos2(c.x - 4.5, c.y + 2.5), pos2(c.x - 2.0, c.y), pos2(c.x, c.y - 2.0)], Stroke::new(2.0, ic));
        p.line(vec![pos2(c.x, c.y + 2.0), pos2(c.x + 2.0, c.y), pos2(c.x + 4.5, c.y - 2.5)], Stroke::new(2.0, ic));
    } else if lower.contains("rss") {
        let ic = col(249, 115, 22);
        p.circle_filled(pos2(c.x - 4.5, c.y + 4.5), 1.6, ic);
        p.line(vec![pos2(c.x - 4.5, c.y + 0.5), pos2(c.x - 2.0, c.y + 1.5), pos2(c.x - 0.5, c.y + 4.5)], Stroke::new(1.6, ic));
        p.line(vec![pos2(c.x - 4.5, c.y - 3.5), pos2(c.x + 1.0, c.y - 1.5), pos2(c.x + 3.5, c.y + 4.5)], Stroke::new(1.6, ic));
    } else if lower.contains("drop target") || lower.contains("target") {
        let ic = col(13, 148, 136);
        p.circle_stroke(c, 6.5, Stroke::new(1.4, ic));
        p.circle_stroke(c, 3.5, Stroke::new(1.4, ic));
        p.circle_filled(c, 1.3, ic);
    } else if lower.contains("export") {
        let ic = col(124, 58, 237);
        p.line(vec![pos2(c.x - 5.5, c.y + 1.5), pos2(c.x - 5.5, c.y + 5.5), pos2(c.x + 5.5, c.y + 5.5), pos2(c.x + 5.5, c.y + 1.5)], Stroke::new(1.5, ic));
        p.line_segment([pos2(c.x, c.y + 2.5), pos2(c.x, c.y - 5.0)], Stroke::new(1.6, ic));
        p.line(vec![pos2(c.x - 3.0, c.y - 2.0), pos2(c.x, c.y - 5.0), pos2(c.x + 3.0, c.y - 2.0)], Stroke::new(1.6, ic));
    } else if lower.contains("import") {
        let ic = col(71, 85, 105);
        p.line(vec![pos2(c.x - 5.5, c.y + 1.5), pos2(c.x - 5.5, c.y + 5.5), pos2(c.x + 5.5, c.y + 5.5), pos2(c.x + 5.5, c.y + 1.5)], Stroke::new(1.5, ic));
        p.line_segment([pos2(c.x, c.y - 5.0), pos2(c.x, c.y + 2.5)], Stroke::new(1.6, ic));
        p.line(vec![pos2(c.x - 3.0, c.y - 0.5), pos2(c.x, c.y + 2.5), pos2(c.x + 3.0, c.y - 0.5)], Stroke::new(1.6, ic));
    } else if lower.contains("exit") {
        let ic = col(225, 29, 72);
        p.line(vec![pos2(c.x + 1.0, c.y - 5.5), pos2(c.x - 4.5, c.y - 5.5), pos2(c.x - 4.5, c.y + 5.5), pos2(c.x + 1.0, c.y + 5.5)], Stroke::new(1.5, ic));
        p.line_segment([pos2(c.x - 1.0, c.y), pos2(c.x + 5.5, c.y)], Stroke::new(1.6, ic));
        p.line(vec![pos2(c.x + 3.0, c.y - 2.5), pos2(c.x + 5.5, c.y), pos2(c.x + 3.0, c.y + 2.5)], Stroke::new(1.6, ic));
    } else if lower.contains("stop all") || lower.contains("pause all") {
        let ic = col(234, 88, 12);
        p.rect_filled(Rect::from_center_size(pos2(c.x - 2.5, c.y), vec2(2.5, 9.0)), 1.2, ic);
        p.rect_filled(Rect::from_center_size(pos2(c.x + 2.5, c.y), vec2(2.5, 9.0)), 1.2, ic);
    } else if lower.contains("stop") {
        let ic = col(217, 119, 6);
        p.circle_stroke(c, 6.5, Stroke::new(1.4, ic));
        p.rect_filled(Rect::from_center_size(c, vec2(5.5, 5.5)), 1.2, ic);
    } else if lower.contains("remove with files") || lower.contains("remove") || lower.contains("delete") {
        let ic = col(225, 29, 72);
        p.line_segment([pos2(c.x - 5.5, c.y - 3.5), pos2(c.x + 5.5, c.y - 3.5)], Stroke::new(1.5, ic));
        p.line_segment([pos2(c.x - 2.0, c.y - 5.0), pos2(c.x + 2.0, c.y - 5.0)], Stroke::new(1.4, ic));
        p.line(vec![pos2(c.x - 4.0, c.y - 3.5), pos2(c.x - 3.5, c.y + 5.0), pos2(c.x + 3.5, c.y + 5.0), pos2(c.x + 4.0, c.y - 3.5)], Stroke::new(1.4, ic));
        p.line_segment([pos2(c.x, c.y - 1.5), pos2(c.x, c.y + 3.0)], Stroke::new(1.2, ic));
    } else if lower.contains("download now") || lower.contains("start queue") || lower.contains("resume") {
        let ic = col(16, 185, 129);
        p.circle_stroke(c, 7.0, Stroke::new(1.4, ic));
        let tri = vec![pos2(c.x - 2.0, c.y - 3.5), pos2(c.x + 3.5, c.y), pos2(c.x - 2.0, c.y + 3.5)];
        p.add(egui::Shape::convex_polygon(tri, ic, Stroke::NONE));
    } else if lower.contains("redownload") {
        let ic = col(2, 132, 199);
        p.circle_stroke(c, 5.5, Stroke::new(1.4, ic));
        p.add(egui::Shape::convex_polygon(vec![pos2(c.x + 2.5, c.y - 6.5), pos2(c.x + 5.5, c.y - 4.5), pos2(c.x + 2.5, c.y - 2.5)], ic, Stroke::NONE));
    } else if lower.contains("find") {
        let ic = col(100, 116, 139);
        p.circle_stroke(pos2(c.x - 1.8, c.y - 1.8), 4.5, Stroke::new(1.5, ic));
        p.line_segment([pos2(c.x + 1.8, c.y + 1.8), pos2(c.x + 6.0, c.y + 6.0)], Stroke::new(2.0, ic));
    } else if lower.contains("select all") {
        let ic = col(16, 185, 129);
        let sq = Rect::from_center_size(c, vec2(12.0, 12.0));
        p.rect_stroke(sq, 2.5, Stroke::new(1.4, ic), egui::StrokeKind::Inside);
        p.line(vec![pos2(c.x - 3.5, c.y), pos2(c.x - 1.0, c.y + 2.5), pos2(c.x + 3.5, c.y - 2.5)], Stroke::new(1.8, ic));
    } else if lower.contains("scheduler") {
        let ic = col(192, 38, 211);
        p.circle_stroke(c, 6.5, Stroke::new(1.5, ic));
        p.line_segment([c, pos2(c.x, c.y - 4.0)], Stroke::new(1.5, ic));
        p.line_segment([c, pos2(c.x + 3.0, c.y)], Stroke::new(1.5, ic));
    } else if lower.contains("speed limiter") || lower.contains("limiter") {
        let ic = col(16, 185, 129);
        p.circle_stroke(c, 6.5, Stroke::new(1.5, ic));
        p.line_segment([c, pos2(c.x + 3.5, c.y - 3.5)], Stroke::new(2.0, ic));
        p.circle_filled(c, 1.8, ic);
    } else if lower.contains("options") || lower.contains("setting") {
        let ic = col(100, 116, 139);
        p.circle_stroke(c, 3.8, Stroke::new(1.8, ic));
        for i in 0..6 {
            let a = (i as f32) * std::f32::consts::PI / 3.0;
            let (sa, ca) = a.sin_cos();
            p.line_segment([pos2(c.x + ca * 3.8, c.y + sa * 3.8), pos2(c.x + ca * 6.5, c.y + sa * 6.5)], Stroke::new(1.8, ic));
        }
    } else if lower.contains("categories") || lower.contains("sidebar") {
        let ic = col(100, 116, 139);
        let frame = Rect::from_center_size(c, vec2(13.0, 12.0));
        p.rect_stroke(frame, 2.5, Stroke::new(1.4, ic), egui::StrokeKind::Inside);
        p.line_segment([pos2(c.x - 2.0, c.y - 6.0), pos2(c.x - 2.0, c.y + 6.0)], Stroke::new(1.4, ic));
    } else if lower.contains("dark") {
        let ic = col(99, 102, 241);
        p.circle_stroke(c, 5.5, Stroke::new(1.5, ic));
        p.circle_filled(pos2(c.x + 2.2, c.y - 1.8), 4.5, if dark { Color32::from_rgb(30, 41, 59) } else { Color32::WHITE });
    } else if lower.contains("language") {
        let ic = col(2, 132, 199);
        p.circle_stroke(c, 6.0, Stroke::new(1.4, ic));
        p.line_segment([pos2(c.x - 6.0, c.y), pos2(c.x + 6.0, c.y)], Stroke::new(1.2, ic));
        p.line(vec![pos2(c.x, c.y - 6.0), pos2(c.x - 2.5, c.y), pos2(c.x, c.y + 6.0), pos2(c.x + 2.5, c.y), pos2(c.x, c.y - 6.0)], Stroke::new(1.2, ic));
    } else if lower.contains("help") || lower.contains("tutorial") || lower.contains("shortcut") || lower.contains("content") || lower.contains("readme") || lower.contains("about") {
        let ic = col(2, 132, 199);
        p.circle_stroke(c, 6.5, Stroke::new(1.4, ic));
        p.circle_filled(pos2(c.x, c.y - 2.8), 1.1, ic);
        p.line_segment([pos2(c.x, c.y - 0.5), pos2(c.x, c.y + 3.0)], Stroke::new(1.5, ic));
    } else if lower.contains("open folder") || lower.contains("browse") {
        let ic = col(245, 158, 11);
        let fld = Rect::from_center_size(pos2(c.x, c.y + 1.0), vec2(13.0, 9.5));
        p.rect_stroke(fld, 1.5, Stroke::new(1.4, ic), egui::StrokeKind::Inside);
        p.line_segment([pos2(c.x - 6.5, c.y - 3.5), pos2(c.x - 2.0, c.y - 3.5)], Stroke::new(1.4, ic));
    } else if lower.contains("open with") {
        let ic = col(2, 132, 199);
        let win = Rect::from_center_size(pos2(c.x - 1.5, c.y + 1.5), vec2(10.0, 9.0));
        p.rect_stroke(win, 1.5, Stroke::new(1.4, ic), egui::StrokeKind::Inside);
        p.line_segment([pos2(c.x - 0.5, c.y + 0.5), pos2(c.x + 5.5, c.y - 5.5)], Stroke::new(1.6, ic));
        p.line(vec![pos2(c.x + 2.0, c.y - 5.5), pos2(c.x + 5.5, c.y - 5.5), pos2(c.x + 5.5, c.y - 2.0)], Stroke::new(1.6, ic));
    } else if lower == "open" || (lower.starts_with("open") && !lower.contains("torrent")) {
        let ic = col(2, 132, 199);
        let doc = Rect::from_center_size(c, vec2(11.0, 13.0));
        p.rect_stroke(doc, 2.0, Stroke::new(1.4, ic), egui::StrokeKind::Inside);
        p.line_segment([pos2(c.x - 1.0, c.y + 1.0), pos2(c.x + 4.5, c.y - 4.5)], Stroke::new(1.6, ic));
        p.line(vec![pos2(c.x + 1.5, c.y - 4.5), pos2(c.x + 4.5, c.y - 4.5), pos2(c.x + 4.5, c.y - 1.5)], Stroke::new(1.6, ic));
    } else if lower.contains("move") || lower.contains("rename") {
        let ic = col(168, 85, 247);
        p.line_segment([pos2(c.x - 4.5, c.y + 4.5), pos2(c.x + 3.5, c.y - 3.5)], Stroke::new(2.2, ic));
        p.line_segment([pos2(c.x + 2.0, c.y - 5.0), pos2(c.x + 5.0, c.y - 2.0)], Stroke::new(1.6, ic));
        p.circle_filled(pos2(c.x - 4.5, c.y + 4.5), 1.2, ic);
    } else if lower.contains("refresh") {
        let ic = col(16, 185, 129);
        p.circle_stroke(c, 5.5, Stroke::new(1.5, ic));
        p.add(egui::Shape::convex_polygon(vec![pos2(c.x + 2.5, c.y - 6.5), pos2(c.x + 5.5, c.y - 4.5), pos2(c.x + 2.5, c.y - 2.5)], ic, Stroke::NONE));
        p.add(egui::Shape::convex_polygon(vec![pos2(c.x - 2.5, c.y + 6.5), pos2(c.x - 5.5, c.y + 4.5), pos2(c.x - 2.5, c.y + 2.5)], ic, Stroke::NONE));
    } else if lower.contains("checksum") || lower.contains("verify") {
        let ic = col(5, 150, 105);
        p.line(vec![pos2(c.x - 5.5, c.y - 4.0), pos2(c.x + 5.5, c.y - 4.0), pos2(c.x + 5.5, c.y), pos2(c.x, c.y + 5.5), pos2(c.x - 5.5, c.y), pos2(c.x - 5.5, c.y - 4.0)], Stroke::new(1.4, ic));
        p.line(vec![pos2(c.x - 2.5, c.y), pos2(c.x - 0.5, c.y + 2.0), pos2(c.x + 2.5, c.y - 1.5)], Stroke::new(1.6, ic));
    } else if lower.contains("extract") {
        let ic = col(217, 119, 6);
        let box_r = Rect::from_center_size(pos2(c.x, c.y + 1.0), vec2(12.0, 10.0));
        p.rect_stroke(box_r, 1.5, Stroke::new(1.4, ic), egui::StrokeKind::Inside);
        p.line_segment([pos2(c.x, c.y + 1.0), pos2(c.x, c.y - 4.5)], Stroke::new(1.6, ic));
        p.line(vec![pos2(c.x - 2.5, c.y - 2.5), pos2(c.x, c.y - 4.5), pos2(c.x + 2.5, c.y - 2.5)], Stroke::new(1.6, ic));
    } else if lower.contains("queue") {
        let ic = col(99, 102, 241);
        p.line_segment([pos2(c.x - 5.5, c.y - 3.5), pos2(c.x + 5.5, c.y - 3.5)], Stroke::new(1.8, ic));
        p.line_segment([pos2(c.x - 5.5, c.y), pos2(c.x + 5.5, c.y)], Stroke::new(1.8, ic));
        p.line_segment([pos2(c.x - 5.5, c.y + 3.5), pos2(c.x + 5.5, c.y + 3.5)], Stroke::new(1.8, ic));
    } else if lower.contains("properties") {
        let ic = col(100, 116, 139);
        p.circle_stroke(c, 6.5, Stroke::new(1.4, ic));
        p.circle_filled(pos2(c.x, c.y - 2.5), 1.1, ic);
        p.line_segment([pos2(c.x, c.y - 0.5), pos2(c.x, c.y + 3.0)], Stroke::new(1.6, ic));
    } else if lower.contains("files...") {
        let ic = col(100, 116, 139);
        p.rect_stroke(Rect::from_center_size(pos2(c.x + 2.0, c.y - 2.0), vec2(9.0, 11.0)), 1.5, Stroke::new(1.2, ic.gamma_multiply(0.6)), egui::StrokeKind::Inside);
        p.rect_stroke(Rect::from_center_size(pos2(c.x - 1.5, c.y + 1.5), vec2(9.0, 11.0)), 1.5, Stroke::new(1.4, ic), egui::StrokeKind::Inside);
    } else if lower.contains("recheck") {
        let ic = col(2, 132, 199);
        p.rect_stroke(Rect::from_center_size(c, vec2(11.0, 13.0)), 2.0, Stroke::new(1.4, ic), egui::StrokeKind::Inside);
        p.line(vec![pos2(c.x - 3.0, c.y), pos2(c.x - 1.0, c.y + 2.0), pos2(c.x + 3.0, c.y - 2.0)], Stroke::new(1.6, ic));
    } else if lower.contains("reannounce") {
        let ic = col(6, 182, 212);
        p.circle_filled(pos2(c.x, c.y + 3.0), 1.5, ic);
        p.line_segment([pos2(c.x, c.y + 3.0), pos2(c.x, c.y - 3.0)], Stroke::new(1.5, ic));
        p.line(vec![pos2(c.x - 4.5, c.y - 1.5), pos2(c.x, c.y - 4.5), pos2(c.x + 4.5, c.y - 1.5)], Stroke::new(1.4, ic));
    } else if lower.contains("tracker") {
        let ic = col(6, 182, 212);
        p.line_segment([pos2(c.x - 3.0, c.y + 4.0), pos2(c.x - 3.0, c.y - 3.0)], Stroke::new(1.5, ic));
        p.circle_filled(pos2(c.x - 3.0, c.y - 3.0), 1.6, ic);
        p.line_segment([pos2(c.x + 1.5, c.y), pos2(c.x + 6.0, c.y)], Stroke::new(1.6, ic));
        p.line_segment([pos2(c.x + 3.75, c.y - 2.25), pos2(c.x + 3.75, c.y + 2.25)], Stroke::new(1.6, ic));
    } else if lower.contains("order") || lower.contains("pieces first") {
        let ic = col(139, 92, 246);
        p.line_segment([pos2(c.x - 4.5, c.y - 4.0), pos2(c.x + 4.5, c.y - 4.0)], Stroke::new(1.5, ic));
        p.line_segment([pos2(c.x - 4.5, c.y), pos2(c.x + 2.0, c.y)], Stroke::new(1.5, ic));
        p.line_segment([pos2(c.x - 4.5, c.y + 4.0), pos2(c.x - 1.0, c.y + 4.0)], Stroke::new(1.5, ic));
        p.line(vec![pos2(c.x + 2.0, c.y + 1.5), pos2(c.x + 4.5, c.y + 4.0), pos2(c.x + 2.0, c.y + 6.5)], Stroke::new(1.4, ic));
    } else if lower.contains("add category") {
        let ic = col(16, 185, 129);
        let fld = Rect::from_center_size(pos2(c.x - 1.0, c.y + 1.0), vec2(11.0, 8.5));
        p.rect_stroke(fld, 1.5, Stroke::new(1.3, ic), egui::StrokeKind::Inside);
        p.line_segment([pos2(c.x + 2.0, c.y - 1.0), pos2(c.x + 6.0, c.y - 1.0)], Stroke::new(1.5, ic));
        p.line_segment([pos2(c.x + 4.0, c.y - 3.0), pos2(c.x + 4.0, c.y + 1.0)], Stroke::new(1.5, ic));
    } else if lower.contains("filter") {
        let ic = col(239, 68, 68);
        p.line(vec![pos2(c.x - 5.5, c.y - 4.5), pos2(c.x + 5.5, c.y - 4.5), pos2(c.x + 1.5, c.y), pos2(c.x + 1.5, c.y + 4.5), pos2(c.x - 1.5, c.y + 3.0), pos2(c.x - 1.5, c.y), pos2(c.x - 5.5, c.y - 4.5)], Stroke::new(1.4, ic));
    } else if lower.contains("arrange") || lower.contains("sort") {
        let ic = col(100, 116, 139);
        p.line_segment([pos2(c.x - 2.5, c.y - 4.5), pos2(c.x - 2.5, c.y + 4.5)], Stroke::new(1.5, ic));
        p.line(vec![pos2(c.x - 4.5, c.y - 2.5), pos2(c.x - 2.5, c.y - 4.5), pos2(c.x - 0.5, c.y - 2.5)], Stroke::new(1.4, ic));
        p.line_segment([pos2(c.x + 2.5, c.y - 4.5), pos2(c.x + 2.5, c.y + 4.5)], Stroke::new(1.5, ic));
        p.line(vec![pos2(c.x + 0.5, c.y + 2.5), pos2(c.x + 2.5, c.y + 4.5), pos2(c.x + 4.5, c.y + 2.5)], Stroke::new(1.4, ic));
    } else if lower == "undo" {
        let ic = col(2, 132, 199);
        p.line(vec![pos2(c.x - 1.5, c.y - 4.5), pos2(c.x - 4.5, c.y - 2.0), pos2(c.x - 1.5, c.y + 0.5)], Stroke::new(1.5, ic));
        p.line(vec![pos2(c.x - 4.5, c.y - 2.0), pos2(c.x + 2.0, c.y - 2.0), pos2(c.x + 4.5, c.y + 1.0), pos2(c.x + 4.5, c.y + 4.0)], Stroke::new(1.5, ic));
    } else if lower == "cut" {
        let ic = col(225, 29, 72);
        p.circle_stroke(pos2(c.x - 3.5, c.y + 3.0), 2.2, Stroke::new(1.3, ic));
        p.circle_stroke(pos2(c.x + 3.5, c.y + 3.0), 2.2, Stroke::new(1.3, ic));
        p.line_segment([pos2(c.x - 2.5, c.y + 1.5), pos2(c.x + 4.0, c.y - 4.5)], Stroke::new(1.5, ic));
        p.line_segment([pos2(c.x + 2.5, c.y + 1.5), pos2(c.x - 4.0, c.y - 4.5)], Stroke::new(1.5, ic));
    } else if lower == "copy" {
        let ic = col(2, 132, 199);
        p.rect_stroke(Rect::from_center_size(pos2(c.x + 2.0, c.y - 2.0), vec2(9.0, 11.0)), 1.5, Stroke::new(1.2, ic.gamma_multiply(0.6)), egui::StrokeKind::Inside);
        p.rect_stroke(Rect::from_center_size(pos2(c.x - 1.5, c.y + 1.5), vec2(9.0, 11.0)), 1.5, Stroke::new(1.5, ic), egui::StrokeKind::Inside);
    } else if lower == "paste" {
        let ic = col(16, 185, 129);
        let bd = Rect::from_center_size(pos2(c.x, c.y + 1.0), vec2(11.0, 12.0));
        p.rect_stroke(bd, 1.8, Stroke::new(1.4, ic), egui::StrokeKind::Inside);
        p.rect_filled(Rect::from_center_size(pos2(c.x, c.y - 5.0), vec2(5.0, 2.5)), 0.8, ic);
        p.line_segment([pos2(c.x - 2.5, c.y + 1.0), pos2(c.x + 2.5, c.y + 1.0)], Stroke::new(1.2, ic));
        p.line_segment([pos2(c.x - 2.5, c.y + 3.5), pos2(c.x + 2.5, c.y + 3.5)], Stroke::new(1.2, ic));
    } else if lower.contains("site") {
        let ic = col(2, 132, 199);
        p.circle_stroke(c, 6.0, Stroke::new(1.4, ic));
        p.line_segment([pos2(c.x - 6.0, c.y), pos2(c.x + 6.0, c.y)], Stroke::new(1.2, ic));
        p.line(vec![pos2(c.x - 2.5, c.y - 5.5), pos2(c.x, c.y), pos2(c.x - 2.5, c.y + 5.5)], Stroke::new(1.2, ic));
        p.line(vec![pos2(c.x + 2.5, c.y - 5.5), pos2(c.x, c.y), pos2(c.x + 2.5, c.y + 5.5)], Stroke::new(1.2, ic));
    } else if lower.contains("interface size") || lower.contains("size") {
        let ic = col(100, 116, 139);
        p.rect_stroke(Rect::from_center_size(c, vec2(12.0, 12.0)), 2.0, Stroke::new(1.4, ic), egui::StrokeKind::Inside);
        p.line_segment([pos2(c.x - 2.0, c.y + 2.0), pos2(c.x + 3.5, c.y - 3.5)], Stroke::new(1.5, ic));
    } else if lower.contains("text") || lower.contains("smooth") {
        let ic = col(100, 116, 139);
        p.line(vec![pos2(c.x - 4.5, c.y + 5.0), pos2(c.x, c.y - 5.0), pos2(c.x + 4.5, c.y + 5.0)], Stroke::new(1.5, ic));
        p.line_segment([pos2(c.x - 2.5, c.y + 1.5), pos2(c.x + 2.5, c.y + 1.5)], Stroke::new(1.5, ic));
    } else if lower.contains("draw with") {
        let ic = col(139, 92, 246);
        p.circle_stroke(c, 6.0, Stroke::new(1.4, ic));
        p.circle_filled(pos2(c.x - 2.5, c.y - 2.0), 1.2, ic);
        p.circle_filled(pos2(c.x + 2.0, c.y - 2.5), 1.2, ic);
        p.circle_filled(pos2(c.x + 2.5, c.y + 1.5), 1.2, ic);
    } else if lower.contains("toolbar") {
        let ic = col(100, 116, 139);
        p.rect_stroke(Rect::from_center_size(c, vec2(13.0, 8.0)), 1.5, Stroke::new(1.4, ic), egui::StrokeKind::Inside);
        p.line_segment([pos2(c.x - 2.0, c.y - 4.0), pos2(c.x - 2.0, c.y + 4.0)], Stroke::new(1.2, ic));
        p.line_segment([pos2(c.x + 2.0, c.y - 4.0), pos2(c.x + 2.0, c.y + 4.0)], Stroke::new(1.2, ic));
    } else if lower.contains("status") {
        let ic = col(16, 185, 129);
        p.circle_stroke(c, 6.0, Stroke::new(1.8, ic));
        p.circle_filled(c, 2.5, ic);
    } else if lower.contains("time left") || lower.contains("last try") || lower.contains("date") || lower.contains("added") || lower.contains("finished") {
        let ic = col(100, 116, 139);
        p.circle_stroke(c, 6.0, Stroke::new(1.4, ic));
        p.line_segment([c, pos2(c.x, c.y - 3.5)], Stroke::new(1.4, ic));
        p.line_segment([c, pos2(c.x + 2.5, c.y)], Stroke::new(1.4, ic));
    } else if lower.contains("speed") || lower.contains("transfer rate") {
        let ic = col(245, 158, 11);
        p.line(vec![pos2(c.x + 1.0, c.y - 5.5), pos2(c.x - 3.5, c.y), pos2(c.x, c.y), pos2(c.x - 1.0, c.y + 5.5), pos2(c.x + 3.5, c.y), pos2(c.x, c.y), pos2(c.x + 1.0, c.y - 5.5)], Stroke::new(1.4, ic));
    } else if lower.contains("name") || lower.contains("description") {
        let ic = col(100, 116, 139);
        p.rect_stroke(Rect::from_center_size(c, vec2(11.0, 13.0)), 2.0, Stroke::new(1.4, ic), egui::StrokeKind::Inside);
        p.line_segment([pos2(c.x - 3.0, c.y - 2.0), pos2(c.x + 3.0, c.y - 2.0)], Stroke::new(1.2, ic));
        p.line_segment([pos2(c.x - 3.0, c.y + 1.5), pos2(c.x + 1.0, c.y + 1.5)], Stroke::new(1.2, ic));
    } else if lower.contains("reset columns") {
        let ic = col(2, 132, 199);
        let sq = Rect::from_center_size(c, vec2(12.0, 10.0));
        p.rect_stroke(sq, 1.5, Stroke::new(1.3, ic), egui::StrokeKind::Inside);
        p.line_segment([pos2(c.x - 2.0, c.y - 5.0), pos2(c.x - 2.0, c.y + 5.0)], Stroke::new(1.2, ic));
        p.line_segment([pos2(c.x + 2.0, c.y - 5.0), pos2(c.x + 2.0, c.y + 5.0)], Stroke::new(1.2, ic));
    } else if lower.contains("clear all filters") || lower.contains("clear filters") {
        let ic = col(239, 68, 68);
        p.line(vec![pos2(c.x - 5.0, c.y - 4.5), pos2(c.x + 5.0, c.y - 4.5), pos2(c.x + 1.5, c.y), pos2(c.x + 1.5, c.y + 4.5), pos2(c.x - 1.5, c.y + 3.0), pos2(c.x - 1.5, c.y), pos2(c.x - 5.0, c.y - 4.5)], Stroke::new(1.3, ic));
        p.line_segment([pos2(c.x + 2.0, c.y + 2.0), pos2(c.x + 5.5, c.y + 5.5)], Stroke::new(1.5, ic));
        p.line_segment([pos2(c.x + 5.5, c.y + 2.0), pos2(c.x + 2.0, c.y + 5.5)], Stroke::new(1.5, ic));
    } else if lower.contains("tell a friend") || lower.contains("friend") {
        let ic = col(16, 185, 129);
        p.circle_stroke(pos2(c.x - 2.5, c.y - 2.0), 3.0, Stroke::new(1.4, ic));
        p.circle_stroke(pos2(c.x + 3.5, c.y - 2.0), 2.5, Stroke::new(1.2, ic));
        p.line(vec![pos2(c.x - 5.5, c.y + 4.5), pos2(c.x - 2.5, c.y + 1.5), pos2(c.x + 0.5, c.y + 4.5)], Stroke::new(1.4, ic));
        p.line(vec![pos2(c.x + 1.0, c.y + 4.5), pos2(c.x + 3.5, c.y + 2.0), pos2(c.x + 6.0, c.y + 4.5)], Stroke::new(1.2, ic));
    } else if lower.contains("support") || lower.contains("sponsor") {
        let ic = col(225, 29, 72);
        p.line(vec![pos2(c.x, c.y + 4.5), pos2(c.x - 4.5, c.y), pos2(c.x - 4.5, c.y - 2.5), pos2(c.x - 2.0, c.y - 4.0), pos2(c.x, c.y - 2.0), pos2(c.x + 2.0, c.y - 4.0), pos2(c.x + 4.5, c.y - 2.5), pos2(c.x + 4.5, c.y), pos2(c.x, c.y + 4.5)], Stroke::new(1.5, ic));
    } else if lower.contains("report a problem") || lower.contains("report") {
        let ic = col(234, 88, 12);
        p.circle_stroke(c, 6.0, Stroke::new(1.4, ic));
        p.line_segment([pos2(c.x, c.y - 3.0), pos2(c.x, c.y + 1.0)], Stroke::new(1.6, ic));
        p.circle_filled(pos2(c.x, c.y + 3.2), 1.0, ic);
    } else if lower.contains("update") {
        let ic = col(2, 132, 199);
        p.circle_stroke(c, 5.5, Stroke::new(1.4, ic));
        p.line_segment([pos2(c.x, c.y + 2.0), pos2(c.x, c.y - 3.5)], Stroke::new(1.5, ic));
        p.line(vec![pos2(c.x - 2.5, c.y - 1.0), pos2(c.x, c.y - 3.5), pos2(c.x + 2.5, c.y - 1.0)], Stroke::new(1.5, ic));
    } else if lower.contains("edit queue") {
        let ic = col(168, 85, 247);
        p.line_segment([pos2(c.x - 5.5, c.y - 3.0), pos2(c.x + 2.0, c.y - 3.0)], Stroke::new(1.6, ic));
        p.line_segment([pos2(c.x - 5.5, c.y + 1.0), pos2(c.x + 1.0, c.y + 1.0)], Stroke::new(1.6, ic));
        p.line_segment([pos2(c.x - 5.5, c.y + 4.0), pos2(c.x - 1.0, c.y + 4.0)], Stroke::new(1.6, ic));
        p.line_segment([pos2(c.x + 2.0, c.y + 5.0), pos2(c.x + 5.5, c.y + 1.5)], Stroke::new(1.6, ic));
    } else if lower.contains("delete from queue") || lower.contains("remove from queue") {
        let ic = col(225, 29, 72);
        p.line_segment([pos2(c.x - 5.5, c.y - 3.0), pos2(c.x + 1.0, c.y - 3.0)], Stroke::new(1.6, ic));
        p.line_segment([pos2(c.x - 5.5, c.y + 1.0), pos2(c.x + 1.0, c.y + 1.0)], Stroke::new(1.6, ic));
        p.line_segment([pos2(c.x + 2.5, c.y - 0.5), pos2(c.x + 5.5, c.y + 2.5)], Stroke::new(1.6, ic));
        p.line_segment([pos2(c.x + 5.5, c.y - 0.5), pos2(c.x + 2.5, c.y + 2.5)], Stroke::new(1.6, ic));
    } else if lower.contains("luna") || lower.contains("mister") || lower.contains("download manager") {
        let ic = col(2, 132, 199);
        p.circle_stroke(c, 6.0, Stroke::new(1.5, ic));
        p.circle_filled(pos2(c.x + 2.2, c.y - 1.8), 4.8, if dark { Color32::from_rgb(30, 41, 59) } else { Color32::WHITE });
    } else if lower.contains("double click") {
        let ic = col(100, 116, 139);
        p.rect_stroke(Rect::from_center_size(c, vec2(9.0, 13.0)), 3.5, Stroke::new(1.4, ic), egui::StrokeKind::Inside);
        p.line_segment([pos2(c.x, c.y - 5.0), pos2(c.x, c.y - 1.5)], Stroke::new(1.4, ic));
    } else if lower.contains("turn on") || lower.contains("turn of") {
        let ic = if lower.contains("on") { col(16, 185, 129) } else { col(100, 116, 139) };
        p.rect_stroke(Rect::from_center_size(c, vec2(13.0, 8.0)), 4.0, Stroke::new(1.4, ic), egui::StrokeKind::Inside);
        let knob_x = if lower.contains("on") { c.x + 2.5 } else { c.x - 2.5 };
        p.circle_filled(pos2(knob_x, c.y), 2.5, ic);
    } else if lower.contains("theme") {
        let ic = col(168, 85, 247);
        p.circle_stroke(c, 6.0, Stroke::new(1.4, ic));
        p.circle_filled(pos2(c.x - 2.5, c.y - 2.0), 1.2, col(239, 68, 68));
        p.circle_filled(pos2(c.x + 2.0, c.y - 2.5), 1.2, col(16, 185, 129));
        p.circle_filled(pos2(c.x + 2.5, c.y + 1.5), 1.2, col(2, 132, 199));
    } else {
        // No icon: draw nothing (never draw a stray dot)
    }
}

fn menu_size<C>(
    ctx: &egui::Context,
    entries: &[MenuEntry<C>],
    hint: &dyn Fn(&C) -> Option<&'static str>,
) -> egui::Vec2 {
    let painter = ctx.layer_painter(egui::LayerId::background());
    let width = |t: &str| menu_width(&painter, t);
    let widest = entries
        .iter()
        .filter_map(|e| match e {
            MenuEntry::Item(l, c, ..) | MenuEntry::Radio(l, c, ..) => Some(match hint(c) {
                Some(k) => {
                    width(without_shortcut(&plain(crate::i18n::tc("menu", l)).0))
                        + 28.0
                        + text_width_font(&painter, winui::segoe(), &key_label(k)).ceil()
                }
                None => width(&plain(crate::i18n::tc("menu", l)).0),
            }),
            MenuEntry::Sub(l, ..) => Some(width(&plain(crate::i18n::tc("menu", l)).0)),
            MenuEntry::Sep => None,
        })
        .fold(0.0, f32::max);
    let sh = slot_h();
    let sph = sep_h();
    let h: f32 = entries
        .iter()
        .map(|e| {
            if matches!(e, MenuEntry::Sep) {
                sph
            } else {
                sh
            }
        })
        .sum();
    let has_sub = entries.iter().any(|e| matches!(e, MenuEntry::Sub(..)));
    if is_modern() {
        let base_w = (widest + if has_sub { 80.0 } else { 68.0 }).max(250.0);
        vec2(base_w, h + 12.0)
    } else {
        vec2(38.0 + widest + if has_sub { 48.0 } else { 30.0 }, h + 6.0)
    }
}

/// How a borderless window of ours looks and takes the pointer ([`style_popup`]).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PopupLook {
    /// A menu: never activated (the main window keeps the focus), small rounded corners and
    /// the menu's border, no open animation.
    Menu,
    /// A small floating window (the drop target): activated as any window, rounded corners.
    Floating,
}

/// The builder of every borderless window of ours (a menu, the drop target): no title bar, not
/// resizable, no taskbar button, always on top. On Windows it is created hidden: its first pass
/// makes it a plain popup ([`style_popup`]) and [`show_popup`] shows it then.
pub fn popup_viewport(title: &str) -> egui::ViewportBuilder {
    egui::ViewportBuilder::default()
        .with_title(crate::i18n::os_text(title))
        .with_decorations(false)
        .with_resizable(false)
        .with_taskbar(false)
        .with_always_on_top()
        // Hidden until styled, then shown (show_popup).
        .with_visible(!cfg!(windows))
}

/// Call first in the pass of a borderless window titled `title` (made with [`popup_viewport`]),
/// before anything is drawn in it. On Windows it becomes a plain popup window, its client the
/// whole window (win32::popup_window): winit's borderless windows keep a caption style, which
/// on a Windows 11 Insider build (22635) put a black band a title bar tall across the top of
/// every menu and of the drop target, their picture moved down by as much (#208). Styled in
/// the window's own pass: its frame changes, and a picture drawn before that was lost with the
/// old frame (black until something made it draw again). True on the pass that styled it: show
/// it then ([`show_popup`]). Elsewhere nothing to do (false).
#[cfg_attr(not(windows), allow(unused_variables))]
pub fn style_popup(title: &str, look: PopupLook) -> bool {
    #[cfg(windows)]
    {
        use crate::win32::Popup;
        let kind = match look {
            PopupLook::Menu => Popup::Menu {
                border: u32::from_le_bytes([
                    POPUP_BORDER().r(),
                    POPUP_BORDER().g(),
                    POPUP_BORDER().b(),
                    0,
                ]),
            },
            PopupLook::Floating => Popup::Floating,
        };
        crate::win32::popup_window(title, kind)
    }
    #[cfg(not(windows))]
    false
}

/// Show the borderless window `vid` [`style_popup`] just styled (from its parent's pass).
pub fn show_popup(ctx: &egui::Context, vid: egui::ViewportId) {
    ctx.send_viewport_cmd_to(vid, egui::ViewportCommand::Visible(true));
    // The command is carried out on the window's next frame: a menu opened from the keyboard
    // (Shift+F10, the Menu key) or a screen reader moves no pointer to bring one, and stayed
    // hidden (#113).
    ctx.request_repaint_of(vid);
    ctx.request_repaint();
}

/// Place a menu of `size` at screen point `at`, flipped to stay inside the monitor's work area:
/// up (or left) of the point, or for a submenu (`parent`: the parent menu's screen rect) to the
/// parent's left side. In a right-to-left interface `at` is the menu's top-right: it opens to
/// the left, and flips to the right (a submenu to the parent's right side). All in points: the
/// work area too (it was taken in pixels, which differ at a display or interface scale other
/// than 100 %, #262).
fn place_menu(ctx: &egui::Context, at: Pos2, size: egui::Vec2, parent: Option<Rect>) -> Pos2 {
    // An off-screen test copy (MDM_OFFSCREEN) keeps its menus beside its window, off the
    // screens too (scale::work_area has none there): the nearest monitor's work area brought
    // them onto the screen, over the user's work.
    if let Some(area) = crate::scale::work_area(ctx, at) {
        let (l, t, r, b) = (area.min.x, area.min.y, area.max.x, area.max.y);
        let mut p = at;
        if crate::i18n::rtl() {
            p.x = at.x - size.x;
            if p.x < l {
                p.x = match parent {
                    Some(pr) => pr.right() - 4.0,
                    None => at.x,
                };
            }
        } else if p.x + size.x > r {
            p.x = match parent {
                Some(pr) => pr.left() - size.x + 4.0,
                None => at.x - size.x,
            };
        }
        if p.y + size.y > b {
            p.y = match parent {
                Some(_) => b - size.y,
                None => at.y - size.y,
            };
        }
        if crate::i18n::rtl() {
            p.x = p.x.min(r - size.x);
        }
        return pos2(p.x.max(l), p.y.max(t));
    }
    let _ = parent;
    if crate::i18n::rtl() {
        return pos2(at.x - size.x, at.y);
    }
    at
}

/// Whether the menu bar's menus share one window per level ([`menu_viewport`]): on X11 and
/// XWayland, where MDM places its menus itself.
fn bar_shares_window() -> bool {
    #[cfg(target_os = "linux")]
    {
        !crate::unix::on_wayland()
    }
    // Windows and macOS: one window per menu, as before. Native Wayland (MDM_WAYLAND=1) too: a
    // program can't move its windows there (winit 0.30 has no popup windows on Wayland), so a
    // shared window would stay where the first menu opened.
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}

/// The window menu `id`'s level `depth` is drawn in. With `bar_shared` (X11: [`bar_shares_window`])
/// every menu of the bar is drawn in one window per level, moved and resized as the pointer or
/// the arrow keys go from title to title, as a submenu's window already is from item to item.
/// A window of its own for each menu was mapped and unmapped at every switch, and a compositor
/// that animates new windows (Hyprland, picom with fading) faded each one in, and the last one
/// out, every time (#277). Other menus: a window of their own, as before.
fn menu_viewport(kind: MenuKind, id: &str, depth: usize, bar_shared: bool) -> egui::ViewportId {
    let key = if bar_shared && kind == MenuKind::Bar {
        "menu-bar"
    } else {
        id
    };
    egui::ViewportId::from_hash_of(("menu", key, depth))
}

/// The entry under `p` in a menu drawn at `o`, `w` wide: its index, none on a separator or the
/// padding (the slots as `popup_level` lays them out).
fn item_at<C>(entries: &[MenuEntry<C>], o: Pos2, w: f32, p: Pos2) -> Option<usize> {
    let sh = slot_h();
    let sph = sep_h();
    let mut y = o.y + if is_modern() { 6.0 } else { 3.0 };
    for (i, e) in entries.iter().enumerate() {
        if matches!(e, MenuEntry::Sep) {
            y += sph;
            continue;
        }
        let slot = Rect::from_min_max(pos2(o.x + 6.0, y), pos2(o.x + w - 6.0, y + sh));
        if slot.contains(p) {
            return Some(i);
        }
        y += sh;
    }
    None
}

/// Whether the pointer at `p`, having left an open submenu's entry at `from`, is on its way into
/// the submenu at `sub` (#579): inside the triangle from `from` to the submenu's near edge, with
/// a pixel's slack. A pointer going up or down the menu instead is not.
fn heading_into(from: Pos2, p: Pos2, sub: Rect) -> bool {
    let near = if sub.center().x >= from.x {
        sub.left()
    } else {
        sub.right()
    };
    let (a, b, c) = (from, pos2(near, sub.top()), pos2(near, sub.bottom()));
    let side = |p1: Pos2, p2: Pos2| (p2.x - p1.x) * (p.y - p1.y) - (p2.y - p1.y) * (p.x - p1.x);
    let (d1, d2, d3) = (side(a, b), side(b, c), side(c, a));
    let slack = 1.0 * (b - a).length().max(1.0);
    let neg = d1 < -slack || d2 < -slack || d3 < -slack;
    let pos = d1 > slack || d2 > slack || d3 > slack;
    !(neg && pos)
}

#[allow(clippy::too_many_arguments)]
fn popup_level<C: Clone>(
    ctx: &egui::Context,
    kind: MenuKind,
    id: &str,
    at: Pos2,
    parent: Option<Rect>,
    entries: &[MenuEntry<C>],
    sub_key: egui::Id,
    result: &mut Option<C>,
    depth: usize,
    hint: &dyn Fn(&C) -> Option<&'static str>,
) {
    let size = menu_size(ctx, entries, hint);
    let kb_key = sub_key.with("kb");
    let kb: Option<usize> = ctx
        .data(|d| d.get_temp::<(Option<usize>, Option<usize>)>(kb_key))
        .and_then(|(a, b)| if depth == 0 { a } else { b });
    let screen = Rect::from_min_size(place_menu(ctx, at, size, parent), size);
    if depth == 1 {
        // Where the submenu is, for its menu's next pass (`heading_into`, #579).
        ctx.data_mut(|d| d.insert_temp(sub_key.with("sub-rect"), screen));
    }
    let title = format!("lunadownloadmanager menu {depth}"); // not translated: an id
    let builder = popup_viewport(&title)
        .with_active(false)
        .with_position(screen.min)
        .with_inner_size(size);
    // Linux: a menu window as X11 menus are (unix::menu_window).
    #[cfg(target_os = "linux")]
    let builder = crate::unix::menu_window(builder, kind);
    let open_sub = if depth == 0 {
        ctx.data(|d| d.get_temp::<usize>(sub_key))
    } else {
        None
    };
    // (index, slot top) of the submenu to show beside this menu.
    let mut sub_at: Option<(usize, f32)> = None;
    let vid = menu_viewport(kind, id, depth, bar_shares_window());
    let parent_vid = ctx.viewport_id();
    // Set when this pass made the new (hidden) window a menu window: shown below. (Elsewhere a
    // menu's window is created visible.)
    let mut styled = false;
    ctx.show_viewport_immediate(vid, builder, |ui, _| {
        // Made a menu window in its own pass, before anything is drawn in it (style_popup).
        styled = style_popup(&title, PopupLook::Menu);
        // What happens in this window, for popup() and keyboard(), which run in the parent's
        // pass: they must not ask egui about a menu window by its id, which creates one when it
        // isn't open (a submenu, mostly): an untitled copy of the main window, opened and closed
        // every frame (issues #4 and #5: fading windows; on a full-screen Mac, a new space).
        let (focused, keys) = ui.input(|i| (i.viewport().focused == Some(true), menu_keys(i)));
        ui.ctx().data_mut(|d| {
            if focused {
                d.insert_temp(sub_key.with("window-focus"), true);
            }
            if !keys.is_empty() {
                d.get_temp_mut_or_default::<Vec<egui::Key>>(sub_key.with("window-keys"))
                    .extend(keys.iter().copied());
            }
        });
        if !keys.is_empty() {
            // keyboard() takes them on the parent's next pass.
            ui.ctx().request_repaint_of(parent_vid);
        }
        let w = size.x;
        let painter = ui.painter().clone();
        let o = ui.max_rect().min;
        // Where the items are, for the tests that point at them.
        #[cfg(test)]
        ui.ctx()
            .data_mut(|d| d.insert_temp(sub_key.with(("origin", depth)), o));
        // Right to left: text right-aligned, check marks and bullets on the right, the submenu
        // arrow on the left pointing left.
        let _surface = winui::Surface::enter(o.x, o.x + w);
        let rtl = winui::rtl();
        let modern = is_modern();
        let dark = crate::theme::dark();
        if modern {
            let bg = if dark {
                Color32::from_rgb(30, 41, 59)
            } else {
                Color32::WHITE
            };
            let border_c = if dark {
                Color32::from_rgb(51, 65, 85)
            } else {
                Color32::from_rgb(226, 232, 240)
            };
            let full_rect = Rect::from_min_size(o, size);
            // Fill entire client area cleanly so DWM handles corner clipping with no black corner artifacts
            painter.rect_filled(full_rect, 0.0, bg);
            painter.rect_stroke(full_rect, 0.0, Stroke::new(1.0, border_c), egui::StrokeKind::Inside);
        } else {
            fill(
                &painter,
                o.x,
                o.y,
                o.x + w - 1.0,
                o.y + size.y - 1.0,
                POPUP_BG(),
            );
        }
        // Where the pointer is, or was on this pass: a tap on a touch screen ends with the pointer
        // gone (egui-winit lets go of it as the finger lifts, so nothing stays lit), in the very
        // pass that releases it, where the hover position is none and no item was chosen (#733).
        let pointer = ui.input(|i| i.pointer.hover_pos().or(i.pointer.interact_pos()));
        // The item under the pointer (none on a separator or the padding): one item is lit at a
        // time, so an open submenu's entry gives up its light to it (#579).
        let under = pointer.and_then(|p| item_at(entries, o, w, p));
        // An open submenu stays open while the pointer is on another item only as long as it
        // heads into the submenu (#579): from where it left the submenu's entry, inside the
        // triangle to the submenu's near edge. Its screen rect, from the pass that drew it, in
        // this window's points.
        let anchor_key = sub_key.with("anchor");
        let sub_rect = (depth == 0)
            .then(|| {
                ui.ctx()
                    .data(|d| d.get_temp::<Rect>(sub_key.with("sub-rect")))
            })
            .flatten()
            .map(|r| r.translate(o.to_vec2() - screen.min.to_vec2()));
        let clicked = ui.input(|i| i.pointer.primary_released());
        // The pointer moving over an item takes the highlight from the keyboard.
        let moving = ui.input(|i| i.pointer.is_moving());
        // For screen readers and UI Automation (#113): the window is a menu, its entries its
        // items (made in a child whose accessibility parent is the menu); invoking one chooses
        // it, as a click does.
        let menu_node = ui.interact(
            Rect::from_min_size(o, size),
            egui::Id::unique(("menu-node", id, depth)),
            egui::Sense::hover(),
        );
        winui::expose(&menu_node, egui::Role::Menu, "", true, None);
        let items_ui = ui.new_child(egui::UiBuilder::new().accessibility_parent(menu_node.id));
        let sh = slot_h();
        let sph = sep_h();
        let mut y = o.y + if modern { 6.0 } else { 3.0 };
        for (i, e) in entries.iter().enumerate() {
            if matches!(e, MenuEntry::Sep) {
                if modern {
                    let sep_y = y + sph / 2.0;
                    let sep_color = if dark {
                        Color32::from_rgb(51, 65, 85)
                    } else {
                        Color32::from_rgb(226, 232, 240)
                    };
                    painter.line_segment(
                        [pos2(o.x + 12.0, sep_y), pos2(o.x + w - 12.0, sep_y)],
                        Stroke::new(1.0, sep_color),
                    );
                } else {
                    winui::fill_l(
                        &painter,
                        o.x + 9.0,
                        y + 4.0,
                        o.x + w - 10.0,
                        y + 4.0,
                        POPUP_SEP(),
                    );
                }
                y += sph;
                continue;
            }
            let (label, enabled, checked) = match e {
                MenuEntry::Item(l, _, en, ch) | MenuEntry::Radio(l, _, en, ch) => (*l, *en, *ch),
                MenuEntry::Sub(l, en, _) => (*l, *en, false),
                MenuEntry::Sep => unreachable!(),
            };
            let slot = Rect::from_min_max(pos2(o.x + 6.0, y), pos2(o.x + w - 6.0, y + sh));
            let hovered = pointer.is_some_and(|p| slot.contains(p));
            if hovered && moving && kb.is_some() && kb != Some(i) {
                ui.ctx().data_mut(|d| {
                    let (a, b) = d
                        .get_temp::<(Option<usize>, Option<usize>)>(kb_key)
                        .unwrap_or_default();
                    d.insert_temp(
                        kb_key,
                        if depth == 0 {
                            (Some(i), b)
                        } else {
                            (a, Some(i))
                        },
                    );
                });
            }
            let lit = match kb {
                Some(k) => k == i,
                None => hovered,
            };
            let opened = open_sub == Some(i) && under.is_none_or(|u| u == i);
            if modern {
                if lit || opened {
                    let hover_c = if dark {
                        Color32::from_rgb(51, 65, 85)
                    } else {
                        Color32::from_rgb(241, 245, 249)
                    };
                    painter.rect_filled(slot, 8.0, hover_c);
                }
            } else if lit || opened {
                painter.rect_filled(
                    slot,
                    crate::theme::metrics().menu_item_radius,
                    POPUP_HOVER(),
                );
            } else {
                fill(
                    &painter,
                    slot.left(),
                    y,
                    slot.right() - 1.0,
                    y + sh - 1.0,
                    POPUP_ITEM(),
                );
            }

            let color = if modern {
                if !enabled {
                    if dark { Color32::from_rgb(100, 116, 139) } else { Color32::from_rgb(148, 163, 184) }
                } else if dark {
                    Color32::from_rgb(241, 245, 249)
                } else {
                    Color32::from_rgb(51, 65, 85)
                }
            } else if enabled {
                TEXT()
            } else {
                TEXT_DISABLED_MENU()
            };
            let glyph = if enabled {
                GLYPH()
            } else {
                TEXT_DISABLED_MENU()
            };
            let (t, mn) = plain(crate::i18n::tc("menu", label));
            let keys = match e {
                MenuEntry::Item(_, c, ..) | MenuEntry::Radio(_, c, ..) => hint(c),
                _ => None,
            };
            let t = if keys.is_some() {
                without_shortcut(&t).to_owned()
            } else {
                t
            };
            let item = items_ui.interact(
                slot,
                egui::Id::unique(("menu-item", id, depth, i)),
                winui::pointer_only(),
            );
            let (role, on) = match e {
                MenuEntry::Radio(..) => (egui::Role::MenuItemRadio, Some(checked)),
                MenuEntry::Item(..) if checked => (egui::Role::MenuItemCheckBox, Some(true)),
                _ => (egui::Role::MenuItem, None),
            };
            winui::expose(&item, role, &t, enabled, on);
            if let MenuEntry::Sub(..) = e {
                let open = open_sub == Some(i);
                ui.ctx().accesskit_node_builder(item.id, |n| {
                    n.set_has_popup(egui::accesskit::HasPopup::Menu);
                    n.set_expanded(open);
                });
            }
            let expand_sub = matches!(e, MenuEntry::Sub(..))
                && ui.input(|x| {
                    x.has_accesskit_action_request(item.id, egui::accesskit::Action::Expand)
                });
            let invoked = (item.clicked() && !item.clicked_by(egui::PointerButton::Primary)
                || expand_sub)
                && enabled;
            match e {
                MenuEntry::Item(_, c, ..) | MenuEntry::Radio(_, c, ..) if invoked => {
                    *result = Some(c.clone());
                    ui.ctx().request_repaint_of(parent_vid);
                }
                MenuEntry::Sub(..) if invoked && depth == 0 => {
                    ui.ctx().data_mut(|d| d.insert_temp(sub_key, i));
                    sub_at = Some((i, y - o.y));
                    ui.ctx().request_repaint_of(parent_vid);
                }
                _ => {}
            }

            let text_baseline = y + if modern { (sh - 13.0) / 2.0 + 10.0 } else { 14.0 };
            let text_x = if modern { o.x + 42.0 } else { o.x + 38.0 };
            let drawn = text_font(&painter, winui::segoe(), text_x, text_baseline, &t, color);

            // Paint modern icon or radio indicator
            if modern {
                if matches!(e, MenuEntry::Radio(..)) {
                    let radio_c = if checked {
                        Color32::from_rgb(16, 185, 129)
                    } else if dark {
                        Color32::from_rgb(71, 85, 105)
                    } else {
                        Color32::from_rgb(203, 213, 225)
                    };
                    let center = pos2(o.x + 22.0, y + sh / 2.0);
                    painter.circle_stroke(center, 5.5, Stroke::new(1.4, radio_c));
                    if checked {
                        painter.circle_filled(center, 2.8, Color32::from_rgb(16, 185, 129));
                    }
                } else {
                    paint_menu_icon(&painter, &t, pos2(o.x + 22.0, y + sh / 2.0), enabled);
                }
            }

            // The shortcut at the trailing edge
            if let Some(k) = keys {
                let kl = key_label(k);
                if modern {
                    let kw = text_width_font(&painter, winui::segoe(), &kl).ceil();
                    let pill_rect = Rect::from_center_size(
                        pos2(o.x + w - 20.0 - kw / 2.0, y + sh / 2.0),
                        vec2(kw + 10.0, 18.0),
                    );
                    let pill_bg = if dark {
                        Color32::from_rgb(51, 65, 85)
                    } else {
                        Color32::from_rgb(241, 245, 249)
                    };
                    let pill_text = if dark {
                        Color32::from_rgb(148, 163, 184)
                    } else {
                        Color32::from_rgb(100, 116, 139)
                    };
                    painter.rect_filled(pill_rect, 4.0, pill_bg);
                    text_font(
                        &painter,
                        winui::segoe(),
                        pill_rect.min.x + 5.0,
                        pill_rect.min.y + 13.0,
                        &kl,
                        pill_text,
                    );
                } else {
                    winui::text_right(
                        &painter,
                        o.x + w - 24.0,
                        y + 14.0,
                        &kl,
                        TEXT_DISABLED_MENU(),
                    );
                }
            }

            // Underlined mnemonics
            if let Some(m) = mn && cues(ctx) {
                let (x0, x1) = winui::char_span(&painter, &winui::segoe(), &t, m, drawn);
                fill(&painter, x0, text_baseline + 2.0, x1, text_baseline + 2.0, color);
            }

            // Checkmarks
            let gx = |x: f32| if rtl { x + w - 33.0 } else { x };
            if modern {
                if checked && !matches!(e, MenuEntry::Radio(..)) {
                    let emerald = Color32::from_rgb(5, 150, 105);
                    let check_x = o.x + w - 20.0;
                    let check_y = y + sh / 2.0;
                    painter.line(
                        vec![
                            pos2(check_x - 5.0, check_y),
                            pos2(check_x - 2.0, check_y + 3.5),
                            pos2(check_x + 4.0, check_y - 3.5),
                        ],
                        Stroke::new(1.8, emerald),
                    );
                }
            } else if checked && matches!(e, MenuEntry::Radio(..)) {
                const BULLET: [[u8; 4]; 4] = [
                    [0x54, 0xca, 0xd7, 0x54],
                    [0xd7, 0xff, 0xff, 0xca],
                    [0xca, 0xff, 0xff, 0xd7],
                    [0x54, 0xd7, 0xca, 0x54],
                ];
                for (dy, row) in BULLET.iter().enumerate() {
                    for (dx, v) in row.iter().enumerate() {
                        let a = ((*v as f32 - 44.0) / (255.0 - 44.0) * 255.0).round() as u8;
                        let c = Color32::from_rgba_unmultiplied(glyph.r(), glyph.g(), glyph.b(), a);
                        px(&painter, gx(o.x + 15.0 + dx as f32), y + 9.0 + dy as f32, c);
                    }
                }
            } else if checked {
                painter.line(
                    vec![
                        pos2(gx(o.x + 12.5), y + 11.5),
                        pos2(gx(o.x + 15.5), y + 14.5),
                        pos2(gx(o.x + 20.5), y + 8.5),
                    ],
                    Stroke::new(1.3, glyph),
                );
            }

            match e {
                MenuEntry::Sub(..) => {
                    let chev_y = y + if modern { sh / 2.0 } else { 11.5 };
                    if modern {
                        let cx = winui::mx(o.x + w - 20.0);
                        let cy = chev_y;
                        let btn_rect = Rect::from_center_size(pos2(cx, cy), vec2(20.0, 20.0));
                        let (btn_bg, border_c, glyph_c) = if hovered {
                            if dark {
                                (Color32::from_rgb(71, 85, 105), Color32::from_rgb(100, 116, 139), Color32::WHITE)
                            } else {
                                (Color32::from_rgb(203, 213, 225), Color32::from_rgb(148, 163, 184), Color32::from_rgb(15, 23, 42))
                            }
                        } else if dark {
                            (Color32::from_rgb(51, 65, 85), Color32::from_rgb(71, 85, 105), Color32::from_rgb(226, 232, 240))
                        } else {
                            (Color32::from_rgb(226, 232, 240), Color32::from_rgb(203, 213, 225), Color32::from_rgb(71, 85, 105))
                        };
                        painter.rect_filled(winui::rect(btn_rect), 5.0, btn_bg);
                        painter.rect_stroke(winui::rect(btn_rect), 5.0, Stroke::new(1.0, border_c), egui::StrokeKind::Inside);
                        let stroke = Stroke::new(1.6, glyph_c);
                        painter.line_segment([pos2(cx - 2.0, cy - 3.5), pos2(cx + 1.5, cy)], stroke);
                        painter.line_segment([pos2(cx + 1.5, cy), pos2(cx - 2.0, cy + 3.5)], stroke);
                    } else {
                        painter.line(
                            vec![
                                pos2(winui::mx(o.x + w - 20.5), chev_y - 4.0),
                                pos2(winui::mx(o.x + w - 16.5), chev_y),
                                pos2(winui::mx(o.x + w - 20.5), chev_y + 4.0),
                            ],
                            Stroke::new(1.2, glyph),
                        );
                    }
                    if depth == 0 && enabled && hovered && (kb.is_none() || moving) {
                        ui.ctx().data_mut(|d| d.insert_temp(sub_key, i));
                    }
                    if depth == 0
                        && hovered
                        && let Some(p) = pointer
                    {
                        ui.ctx().data_mut(|d| d.insert_temp(anchor_key, p));
                    }
                    if depth == 0 && (open_sub == Some(i) || (enabled && hovered && kb.is_none())) {
                        sub_at = Some((i, y - o.y));
                    }
                }
                MenuEntry::Item(_, c, ..) | MenuEntry::Radio(_, c, ..) => {
                    let since_key = sub_key.with(("leave-since", i));
                    if depth == 0 && hovered && open_sub.is_some() {
                        let now = ui.input(|inp| inp.time);
                        let since = ui
                            .ctx()
                            .data_mut(|d| *d.get_temp_mut_or_insert_with(since_key, || now));
                        let anchor = ui.ctx().data(|d| d.get_temp::<Pos2>(anchor_key));
                        let heading = match (anchor, pointer, sub_rect) {
                            (Some(a), Some(p), Some(r)) => heading_into(a, p, r),
                            _ => false,
                        };
                        if !heading || now - since >= 0.35 {
                            ui.ctx().data_mut(|d| {
                                d.remove::<usize>(sub_key);
                                d.remove::<f64>(since_key);
                                d.remove::<Pos2>(anchor_key);
                                d.remove::<Rect>(sub_key.with("sub-rect"));
                            });
                            sub_at = None;
                        } else {
                            ui.ctx().request_repaint_after_for(
                                std::time::Duration::from_millis(60),
                                parent_vid,
                            );
                        }
                    } else if depth == 0 {
                        ui.ctx().data_mut(|d| d.remove::<f64>(since_key));
                    }
                    if hovered && clicked && enabled {
                        *result = Some(c.clone());
                    }
                }
                MenuEntry::Sep => {}
            }
            y += sh;
        }
    });
    if styled {
        show_popup(ctx, vid);
    }
    if let Some((i, slot_y)) = sub_at
        && let MenuEntry::Sub(_, _, children) = &entries[i]
    {
        // The submenu's top-left: parent right − 6, slot top − 3 (as measured); in RTL its
        // top-right at parent left + 6.
        let x = if crate::i18n::rtl() {
            screen.min.x + 6.0
        } else {
            screen.max.x - 6.0
        };
        let at = pos2(x, screen.min.y + slot_y - 3.0);
        popup_level(
            ctx,
            kind,
            id,
            at,
            Some(screen),
            children,
            sub_key,
            result,
            depth + 1,
            hint,
        );
    }
}

// ---- toolbar -------------------------------------------------------------------------------

pub enum ToolbarEvent {
    Click(usize),
    /// A drop-down button's arrow: its menu opens at this point (button left, y 87).
    Drop(usize, Pos2),
}

/// Paint the etch (y 20, 21) and the toolbar band (y 22–91).
/// Client y where the panes (Categories, list) start: below the toolbar band, in both modes.
pub fn pane_top() -> f32 {
    70.0
}



pub fn toolbar(
    ui: &mut Ui,
    o: Pos2,
    width: f32,
    buttons: &[(Icon, &str, bool)],
) -> Option<ToolbarEvent> {
    let p = ui.painter().clone();
    let top = o.y + menu_h();
    let c = crate::theme::colors();
    let is_modern = crate::theme::metrics().button_radius > 4.0;
    if !is_modern {
        fill(&p, o.x, top, o.x + width - 1.0, top, c.toolbar_line);
        fill(
            &p,
            o.x,
            top + 1.0,
            o.x + width - 1.0,
            top + 1.0,
            c.toolbar_line_light,
        );
    }
    // Both modes (light-main.md; dark changes only colours): Segoe labels when modern, Tahoma when classic.
    let (font, base_w, band_h) = if is_modern {
        (winui::segoe(), 66.0f32, 68.0)
    } else {
        (winui::tahoma(), 66.0f32, 68.0)
    };
    let band = Rect::from_min_size(pos2(o.x, top + if is_modern { 0.0 } else { 2.0 }), vec2(width, band_h));
    let dark = crate::theme::dark();
    let t_dark = ui.ctx().animate_bool(egui::Id::unique("theme_dark_transition"), dark);
    let band_bg = winui::caption::lerp_color(
        crate::theme::current().palette(false).window,
        crate::theme::current().palette(true).window,
        t_dark,
    );
    p.rect_filled(band, 0.0, band_bg);
    let is_dropdown =
        |icon: &Icon| matches!(icon, Icon::StartQueue | Icon::StopQueue | Icon::AddTorrent);
    let mut total_needed = 0.0f32;
    for (icon, label, _) in buttons {
        let full = crate::i18n::tc("toolbar", label);
        let full_ext = text_width_font(&p, font.clone(), full).round();
        let bw = base_w.max(full_ext + 12.0);
        let total = bw + if is_dropdown(icon) { 15.0 } else { 0.0 };
        total_needed += total;
        if is_modern && matches!(icon, Icon::AddTorrent | Icon::StopAll | Icon::DeleteCompleted | Icon::Scheduler) {
            total_needed += 8.0;
        }
    }

    let scroll_id = egui::Id::unique("tb-scroll-offset");
    let mut scroll_offset: f32 = ui.ctx().data(|d| d.get_temp(scroll_id)).unwrap_or(0.0);
    let nav_btn_w = 28.0;
    let overflow = is_modern && total_needed > width;
    let max_scroll = if overflow {
        (total_needed - width + nav_btn_w * 2.0 + 8.0).max(0.0)
    } else {
        0.0
    };
    if overflow && ui.rect_contains_pointer(winui::rect(band)) {
        let delta = ui.input(|i| i.smooth_scroll_delta.x + i.smooth_scroll_delta.y);
        if delta.abs() > 0.0 {
            scroll_offset = (scroll_offset - delta).clamp(0.0, max_scroll);
            ui.ctx().data_mut(|d| d.insert_temp(scroll_id, scroll_offset));
            ui.ctx().request_repaint();
        }
    }
    scroll_offset = if overflow { scroll_offset.clamp(0.0, max_scroll) } else { 0.0 };

    let show_left = overflow && scroll_offset > 2.0;
    let show_right = overflow && scroll_offset < max_scroll - 2.0;

    let left_bound = if show_left { o.x + nav_btn_w + 6.0 } else { o.x };
    let right_bound = if show_right { o.x + width - nav_btn_w - 6.0 } else { o.x + width };
    let content_clip = Rect::from_min_max(pos2(left_bound, band.top()), pos2(right_bound, band.bottom()));
    let tp = if overflow { p.with_clip_rect(winui::rect(content_clip)) } else { p.clone() };

    let mut x = o.x + if show_left { nav_btn_w + 6.0 } else { 0.0 } - scroll_offset;
    let mut event = None;

    // A toolbar wider than the window (long translations): the longest labels give way first,
    // cut with "..." and whole on hover, so every button stays on screen.
    let natural: Vec<(f32, f32)> = buttons
        .iter()
        .map(|(icon, label, _)| {
            let ext = text_width_font(&p, font.clone(), crate::i18n::tc("toolbar", label)).round();
            (ext, if is_dropdown(icon) { 15.0 } else { 0.0 })
        })
        .collect();
    let cap = if overflow { f32::INFINITY } else { label_cap(&natural, base_w, width) };
    for (i, (icon, label, enabled)) in buttons.iter().enumerate() {
        let full = crate::i18n::tc("toolbar", label);
        let dropdown = is_dropdown(icon);
        // IDM's buttons are 66 px; ours grow only when a full label needs it (user
        // decision), up to the cap that keeps the toolbar inside the window.
        let full_ext = text_width_font(&p, font.clone(), full).round();
        let bw = if overflow { base_w.max(full_ext + 12.0) } else { base_w.max((full_ext + 12.0).min(cap)) };
        let cut = !overflow && full_ext + 12.0 > bw;
        let shown = if cut {
            winui::ellipsize(&p, &font, full, bw - 12.0)
        } else {
            full.to_owned()
        };
        let label = &shown;
        let ext = text_width_font(&p, font.clone(), label).round();
        let total = bw + if dropdown { 15.0 } else { 0.0 };
        // Laid out in English from the leading edge; each part made where it is on screen (the
        // first button at the right in RTL, a drop-down's arrow after its button).
        let rc = winui::rect(Rect::from_min_size(
            pos2(x, band.top()),
            vec2(total, band_h),
        ));
        let btn_interactable = !overflow || (rc.right() >= left_bound && rc.left() <= right_bound);
        let resp = if btn_interactable {
            ui.interact(rc, egui::Id::unique(("tb", i)), winui::pointer_only())
        } else {
            ui.interact(Rect::NOTHING, egui::Id::unique(("tb", i)), winui::pointer_only())
        };
        let resp = if cut { resp.on_hover_text(full) } else { resp };
        let past_button = |q: Pos2| winui::mx(q.x) >= x + bw;
        let on_arrow = dropdown && resp.hover_pos().is_some_and(past_button);
        if *enabled && resp.hovered() {
            if is_modern {
                let card = winui::rect(Rect::from_min_max(
                    pos2(x + 2.0, band.top() + 3.0),
                    pos2(x + total - 2.0, band.top() + band_h - 3.0),
                ));
                let (bg, border) = if resp.is_pointer_button_down_on() {
                    if crate::theme::dark() {
                        (Color32::from_rgb(40, 53, 72), Color32::from_rgb(71, 85, 105))
                    } else {
                        (Color32::from_rgb(226, 232, 240), Color32::from_rgb(203, 213, 225))
                    }
                } else if crate::theme::dark() {
                    (Color32::from_rgb(51, 65, 85), Color32::from_rgb(71, 85, 105))
                } else {
                    (Color32::from_rgb(241, 245, 249), Color32::from_rgb(203, 213, 225))
                };
                tp.rect_filled(card, 12.0, bg);
                tp.rect_stroke(card, 12.0, egui::Stroke::new(1.0, border), egui::StrokeKind::Inside);
            } else {
                let h = 65.0;
                let (hot_c, faint_c, down_c, edge_c, border_c) = (
                    c.toolbar_hot,
                    c.toolbar_hot_faint,
                    c.toolbar_pressed,
                    c.toolbar_hot_edge,
                    c.toolbar_hot_border,
                );
                let down = resp.is_pointer_button_down_on();
                let main = winui::rect(Rect::from_min_size(pos2(x, band.top()), vec2(bw, h)));
                let arrow = winui::rect(Rect::from_min_size(
                    pos2(x + bw - 1.0, band.top()),
                    vec2(total - bw + 1.0, h),
                ));
                let lit = |r: Rect, under: bool| {
                    let col = match (under, down) {
                        (true, true) => down_c,
                        (true, false) => hot_c,
                        (false, _) => faint_c,
                    };
                    p.rect_filled(r, 0.0, col);
                    p.rect_stroke(
                        r,
                        0.0,
                        egui::Stroke::new(1.0, border_c),
                        egui::StrokeKind::Inside,
                    );
                };
                lit(main, !on_arrow);
                if dropdown {
                    lit(arrow, on_arrow);
                    winui::fill_l(
                        &p,
                        x + bw - 1.0,
                        band.top(),
                        x + bw - 1.0,
                        band.top() + h - 1.0,
                        edge_c,
                    );
                }
            }
        }
        let cx = x + bw / 2.0;
        let icon_sz = if is_modern { vec2(24.0, 24.0) } else { vec2(32.0, 32.0) };
        let icon_rect = Rect::from_center_size(pos2(cx, top + if is_modern { 24.0 } else { 25.5 }), icon_sz);
        // Monochromatic glyph: background badge completely removed per modern Fluent UI specification
        // Placed, not mirrored: an icon keeps its look.
        if !overflow || (icon_rect.right() >= left_bound && icon_rect.left() <= right_bound) {
            icon.image(*enabled).paint_at(ui, winui::rect(icon_rect));
        }
        // Label: x = btn.x + floor((w − ext)/2), +1 for drop-downs, last ink row 58. Every label
        // on one line (IDM draws the drop-downs' 1 px lower: in one row it read as a mistake).
        let mut lx = x + ((bw - ext) / 2.0).floor();
        if dropdown {
            lx += 1.0;
        }
        let ly = top + if is_modern { 55.0 } else { 58.0 };
        text_font(
            &tp,
            font.clone(),
            lx,
            ly,
            label,
            if *enabled { TEXT() } else { TB_DISABLED() },
        );
        if dropdown {
            let ax = x + bw + 1.0;
            let ay = top + 31.0;
            if is_modern {
                let glyph_c = if on_arrow {
                    if crate::theme::dark() {
                        Color32::WHITE
                    } else {
                        Color32::from_rgb(15, 23, 42)
                    }
                } else if crate::theme::dark() {
                    Color32::from_rgb(148, 163, 184)
                } else {
                    Color32::from_rgb(100, 116, 139)
                };
                let cx = x + bw + 7.5;
                let cy = ay + 3.0;
                let stroke = Stroke::new(1.5, glyph_c);
                tp.line_segment([pos2(cx - 3.5, cy - 1.5), pos2(cx, cy + 2.0)], stroke);
                tp.line_segment([pos2(cx, cy + 2.0), pos2(cx + 3.5, cy - 1.5)], stroke);
            } else {
                if !(*enabled && resp.hovered()) {
                    let (shadow, light) = (c.toolbar_split_shadow, c.toolbar_split_light);
                    let (y0, y1) = (ay - 10.0, ay + 13.0);
                    winui::fill_l(&p, x + bw - 1.0, y0, x + bw - 1.0, y1, shadow);
                    winui::fill_l(&p, x + bw, y0, x + bw, y1, light);
                }
                for (row, half) in [3.0f32, 2.0, 1.0, 0.0].iter().enumerate() {
                    winui::fill_l(
                        &p,
                        ax + 3.0 - half,
                        ay + row as f32,
                        ax + 3.0 + half,
                        ay + row as f32,
                        TB_ARROW(),
                    );
                }
            }
        }
        // No right-click menu: it would offer toolbar looks (sizes, styles) and only one is
        // built.
        // Not a `let` inside the `if`: in a let-chain `&&` ends the binding, so the pointer test
        // became a condition of its own and only the arrow part ever clicked (0.2.3).
        let on_arrow = dropdown && resp.interact_pointer_pos().is_some_and(past_button);
        // The arrow is a button of its own (a split button): its menu lists every queue, so it
        // works while the main one's button is greyed.
        if resp.clicked() && (*enabled || on_arrow) {
            // The drop-down menu's top-left is the button's left edge at client y 85; in RTL its
            // top-right at the button's right edge.
            event = Some(if on_arrow {
                let drop_y = if is_modern { band.bottom() + 4.0 } else { o.y + 85.0 };
                ToolbarEvent::Drop(i, pos2(winui::mx(x), drop_y))
            } else {
                ToolbarEvent::Click(i)
            });
        }
        x += total;
        if is_modern && matches!(icon, Icon::AddTorrent | Icon::StopAll | Icon::DeleteCompleted | Icon::Scheduler) {
            let sep_x = x + 3.0;
            tp.line_segment(
                [pos2(sep_x, band.top() + 14.0), pos2(sep_x, band.top() + 52.0)],
                egui::Stroke::new(1.0, c.toolbar_split_shadow),
            );
            x += 8.0;
        }
    }

    if show_left {
        let btn_y = band.top() + (band_h - 28.0) / 2.0;
        let btn_rect = Rect::from_min_size(pos2(o.x + 3.0, btn_y), vec2(nav_btn_w, 28.0));
        let resp = ui.interact(winui::rect(btn_rect), egui::Id::unique("tb-nav-left"), winui::pointer_only());
        let (btn_bg, border_c, glyph_c) = if resp.hovered() {
            if crate::theme::dark() {
                (Color32::from_rgb(71, 85, 105), Color32::from_rgb(100, 116, 139), Color32::WHITE)
            } else {
                (Color32::from_rgb(203, 213, 225), Color32::from_rgb(148, 163, 184), Color32::from_rgb(15, 23, 42))
            }
        } else if crate::theme::dark() {
            (Color32::from_rgb(51, 65, 85), Color32::from_rgb(71, 85, 105), Color32::from_rgb(226, 232, 240))
        } else {
            (Color32::from_rgb(226, 232, 240), Color32::from_rgb(203, 213, 225), Color32::from_rgb(71, 85, 105))
        };
        p.rect_filled(winui::rect(btn_rect), 8.0, btn_bg);
        p.rect_stroke(winui::rect(btn_rect), 8.0, Stroke::new(1.0, border_c), egui::StrokeKind::Inside);
        let center = btn_rect.center();
        let stroke = Stroke::new(1.8, glyph_c);
        p.line_segment([pos2(center.x + 1.5, center.y - 4.5), pos2(center.x - 2.5, center.y)], stroke);
        p.line_segment([pos2(center.x - 2.5, center.y), pos2(center.x + 1.5, center.y + 4.5)], stroke);
        if resp.clicked() {
            let next = (scroll_offset - 90.0).max(0.0);
            ui.ctx().data_mut(|d| d.insert_temp(scroll_id, next));
            ui.ctx().request_repaint();
        }
    }
    if show_right {
        let btn_y = band.top() + (band_h - 28.0) / 2.0;
        let btn_rect = Rect::from_min_size(pos2(o.x + width - nav_btn_w - 3.0, btn_y), vec2(nav_btn_w, 28.0));
        let resp = ui.interact(winui::rect(btn_rect), egui::Id::unique("tb-nav-right"), winui::pointer_only());
        let (btn_bg, border_c, glyph_c) = if resp.hovered() {
            if crate::theme::dark() {
                (Color32::from_rgb(71, 85, 105), Color32::from_rgb(100, 116, 139), Color32::WHITE)
            } else {
                (Color32::from_rgb(203, 213, 225), Color32::from_rgb(148, 163, 184), Color32::from_rgb(15, 23, 42))
            }
        } else if crate::theme::dark() {
            (Color32::from_rgb(51, 65, 85), Color32::from_rgb(71, 85, 105), Color32::from_rgb(226, 232, 240))
        } else {
            (Color32::from_rgb(226, 232, 240), Color32::from_rgb(203, 213, 225), Color32::from_rgb(71, 85, 105))
        };
        p.rect_filled(winui::rect(btn_rect), 8.0, btn_bg);
        p.rect_stroke(winui::rect(btn_rect), 8.0, Stroke::new(1.0, border_c), egui::StrokeKind::Inside);
        let center = btn_rect.center();
        let stroke = Stroke::new(1.8, glyph_c);
        p.line_segment([pos2(center.x - 2.5, center.y - 4.5), pos2(center.x + 1.5, center.y)], stroke);
        p.line_segment([pos2(center.x + 1.5, center.y), pos2(center.x - 2.5, center.y + 4.5)], stroke);
        if resp.clicked() {
            let next = (scroll_offset + 90.0).min(max_scroll);
            ui.ctx().data_mut(|d| d.insert_temp(scroll_id, next));
            ui.ctx().request_repaint();
        }
    }
    event
}

/// The widest a toolbar button may grow for its label so that all of them fit in `width`:
/// unlimited when they fit whole; else the largest cap at which they do (never under `base_w`).
/// `buttons`: each label's width and its drop-down arrow's.
fn label_cap(buttons: &[(f32, f32)], base_w: f32, width: f32) -> f32 {
    let total = |cap: f32| -> f32 {
        buttons
            .iter()
            .map(|(ext, drop)| base_w.max((ext + 12.0).min(cap)) + drop)
            .sum()
    };
    if total(f32::INFINITY) <= width {
        return f32::INFINITY;
    }
    let (mut lo, mut hi) = (
        base_w,
        buttons.iter().map(|b| b.0 + 12.0).fold(base_w, f32::max),
    );
    for _ in 0..30 {
        let mid = (lo + hi) / 2.0;
        if total(mid) <= width {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo.floor()
}

#[cfg(test)]
mod label_cap_tests {
    #[test]
    fn labels_give_way_only_when_the_toolbar_is_too_wide() {
        let b = [(40.0, 0.0), (200.0, 15.0), (300.0, 0.0)];
        // Room for all: no cap.
        assert!(super::label_cap(&b, 70.0, 1000.0).is_infinite());
        // Too wide: the cap brings it inside, and the short label keeps its button.
        let cap = super::label_cap(&b, 70.0, 450.0);
        let total: f32 = b
            .iter()
            .map(|(e, d)| 70f32.max((e + 12.0).min(cap)) + d)
            .sum();
        assert!(total <= 450.0 && cap >= 70.0, "{cap} {total}");
    }
}

// ---- Categories pane -----------------------------------------------------------------------

#[derive(Clone)]
pub struct TreeRow<'a> {
    pub level: usize,
    pub label: std::borrow::Cow<'a, str>,
    /// None: leaf; Some(open): has an expander.
    pub expander: Option<bool>,
    pub selected: bool,
    pub icon: IconKey,
    /// A small Silk icon overlaid at the icon's bottom-right (Finished ✓, queue stacks).
    pub badge: Option<&'static str>,
    /// How many downloads the node holds, shown after its label (none when empty).
    pub count: Option<usize>,
}

/// A tree node's count, after its label: "(7)".
fn count_text(n: usize) -> String {
    format!("({})", crate::i18n::integer(n as u64))
}

pub enum TreeEvent {
    Select(usize),
    Toggle(usize),
    Close,
    /// Right-click: the row whose menu opens (the selection does not move) and where.
    Context(usize, Pos2),
    OpenSpeedSettings,
}

/// The Categories pane: frame x 6 .. `right` (IDM: 148, resizable by the splitter), caption, close "x", tree (rows 16 px, indent 19).
pub fn categories(
    ui: &mut Ui,
    icons: &mut ShellIcons,
    o: Pos2,
    bottom: f32,
    right: f32,
    rows: &[TreeRow<'_>],
    // The tree has the keys (its chosen node shows the focus).
    keys: bool,
) -> Option<TreeEvent> {
    let p = ui.painter().clone();
    let y0 = o.y + pane_top() + menu_h(); // frame top
    let c = crate::theme::colors();
    let is_modern = crate::theme::metrics().flyout_radius >= 12.0;
    let card_radius = if is_modern { 14.0 } else { 0.0 };
    let (left_pad, top_pad, bot_pad) = if is_modern {
        (12.0, 8.0, 8.0)
    } else {
        (6.0, 0.0, 5.0)
    };
    // Laid out in English (frame x 6 .. right); every part made where it is on screen — the pane
    // on the right in RTL, its tree indenting from the right.
    let frame = winui::rect(Rect::from_min_max(
        pos2(o.x + left_pad, y0 + top_pad),
        pos2(right, bottom - bot_pad),
    ));
    if is_modern {
        p.rect_filled(frame, card_radius, TREE_BG());
    }
    p.rect_stroke(
        frame,
        card_radius,
        Stroke::new(1.0, CAT_FRAME()),
        egui::StrokeKind::Inside,
    );
    if !is_modern {
        winui::fill_l(
            &p,
            o.x + 6.0,
            y0 + 18.0,
            right - 1.0,
            y0 + 18.0,
            CAT_FRAME(),
        ); // caption underline
    }
    // i18n: title of the side pane that lists the categories
    let caption = if is_modern { "CATEGORIES" } else { crate::i18n::t("Categories") };
    let font = if is_modern { winui::segoe() } else { winui::tahoma() };
    let header_h = if is_modern { 60.0 } else { 18.0 };
    // Short of the close box: in a narrow pane or a long translation, "..." and whole on hover.
    let shown = winui::ellipsize(&p, &font, caption, right - 46.0 - (o.x + left_pad + 16.0));
    let cap_y = if is_modern { y0 + top_pad + header_h / 2.0 + 4.5 } else { y0 + 13.0 };
    let header_color = if is_modern {
        if crate::theme::dark() { egui::Color32::from_rgb(148, 163, 184) } else { egui::Color32::from_rgb(100, 116, 139) }
    } else {
        TEXT()
    };
    let drawn = text_font(&p, font.clone(), o.x + left_pad + if is_modern { 16.0 } else { 8.0 }, cap_y, &shown, header_color);
    if shown != caption {
        winui::full_on_hover(ui, drawn, "categories-caption", caption);
    }
    let mut event = None;
    let close = if is_modern {
        let btn_sz = 28.0;
        let btn_y = y0 + top_pad + (header_h - btn_sz) / 2.0;
        winui::rect(Rect::from_min_size(
            pos2(right - 14.0 - btn_sz, btn_y),
            vec2(btn_sz, btn_sz),
        ))
    } else {
        winui::rect(Rect::from_min_size(
            pos2(right - 20.0, y0 + top_pad + 5.0),
            vec2(16.0, 15.0),
        ))
    };
    let cr = ui
        .interact(close, egui::Id::unique("cat-close"), winui::pointer_only())
        // i18n: tooltip of the × that closes the Categories pane
        .on_hover_text(crate::i18n::t("Close"));
    if is_modern {
        let (btn_bg, border_c, glyph_c) = if cr.hovered() {
            if crate::theme::dark() {
                (Color32::from_rgb(71, 85, 105), Color32::from_rgb(100, 116, 139), Color32::WHITE)
            } else {
                (Color32::from_rgb(203, 213, 225), Color32::from_rgb(148, 163, 184), Color32::from_rgb(15, 23, 42))
            }
        } else if crate::theme::dark() {
            (Color32::from_rgb(51, 65, 85), Color32::from_rgb(71, 85, 105), Color32::from_rgb(226, 232, 240))
        } else {
            (Color32::from_rgb(226, 232, 240), Color32::from_rgb(203, 213, 225), Color32::from_rgb(71, 85, 105))
        };
        p.rect_filled(close, 8.0, btn_bg);
        p.rect_stroke(close, 8.0, Stroke::new(1.0, border_c), egui::StrokeKind::Inside);
        let center = close.center();
        let stroke = Stroke::new(1.8, glyph_c);
        p.line_segment([pos2(center.x - 4.5, center.y - 4.5), pos2(center.x + 4.5, center.y + 4.5)], stroke);
        p.line_segment([pos2(center.x + 4.5, center.y - 4.5), pos2(center.x - 4.5, center.y + 4.5)], stroke);
    } else {
        if let Some(c) = winui::subtle_fill(winui::State::of(&cr, true), crate::theme::colors()) {
            p.rect_filled(close, crate::theme::metrics().part_radius, c);
        }
        text_font(
            &p,
            winui::arial_bold(),
            right - 15.0,
            y0 + 13.0,
            crate::i18n::t("x"),
            header_color,
        );
    }
    if cr.clicked() {
        event = Some(TreeEvent::Close);
    }
    let limiter_h = if is_modern { 72.0 } else { 0.0 };
    let tree_top = if is_modern { y0 + top_pad + header_h } else { y0 + 18.0 };
    let tree = Rect::from_min_max(
        pos2(o.x + left_pad + 4.0, tree_top),
        pos2(right - 4.0, bottom - bot_pad - 4.0 - limiter_h),
    );
    if !is_modern {
        p.rect_filled(winui::rect(tree), 0.0, TREE_BG());
    }

    // Scroll bars (notes/ui-spec/scrollbars.md "Categories tree"): by rows vertically, by
    // pixels horizontally; each shows when the content overflows (Win32 rule).
    let row_h = if is_modern { 38.0 } else { 16.0 };
    let content_h = rows.len() as f32 * row_h;
    let content_w = rows
        .iter()
        .map(|r| {
            let text_x = if is_modern {
                if r.level == 0 { 40.0 } else { 56.0 }
            } else if r.level == 0 { 51.0 } else { 70.0 };
            let count = r.count.map_or(0.0, |n| {
                5.0 + text_width_font(&p, font.clone(), &count_text(n))
            });
            text_x - 8.0
                + text_width_font(&p, font.clone(), crate::i18n::t(&r.label))
                + count
                + 5.0
        })
        .fold(0.0, f32::max)
        .ceil();
    let (mut need_v, mut need_h) = (false, false);
    for _ in 0..2 {
        need_h = content_w > tree.width() - if need_v { scrollbar::THICK } else { 0.0 };
        need_v = content_h > tree.height() - if need_h { scrollbar::THICK } else { 0.0 };
    }
    let area = Rect::from_min_max(
        tree.min,
        pos2(
            tree.right() - if need_v { scrollbar::THICK } else { 0.0 },
            tree.bottom() - if need_h { scrollbar::THICK } else { 0.0 },
        ),
    );
    let scroll_id = egui::Id::unique("tree-scroll");
    let (mut vpos, mut hpos) = ui
        .ctx()
        .data(|d| d.get_temp::<(f32, f32)>(scroll_id))
        .unwrap_or_default();
    let page_rows = (area.height() / row_h).floor().max(1.0);
    if ui.rect_contains_pointer(winui::rect(tree)) {
        vpos += winui::wheel_rows(ui, row_h, page_rows);
        hpos -= ui.input(|i| i.smooth_scroll_delta.x);
    }
    vpos = vpos.clamp(0.0, (rows.len() as f32 - page_rows).max(0.0));
    hpos = hpos.clamp(0.0, (content_w - area.width()).max(0.0)).round();
    let first_row = vpos.round();
    // Everything below is laid out from a horizontally scrolled origin.
    let o = pos2(o.x - hpos, o.y);
    let tp = p.with_clip_rect(winui::rect(area));
    let row_top = |k: usize| area.top() + row_h * (k as f32 - first_row);
    // Blank space (registered first, so the rows sit above it): the selected node's menu, at
    // the pointer (IDM places it beside the node; user decision: at the pointer).
    let blank = ui.interact(
        winui::rect(area),
        egui::Id::unique("tree-blank"),
        winui::pointer_only(),
    );
    if blank.secondary_clicked()
        && let Some(k) = rows.iter().position(|r| r.selected)
        && let Some(at) = blank.interact_pointer_pos()
    {
        event = Some(TreeEvent::Context(k, at));
    }
    // For screen readers and UI Automation (#113): the pane is a tree named as its caption, its
    // rows the tree's items (made in a child whose accessibility parent is the tree).
    winui::expose(
        &blank,
        egui::Role::Tree,
        crate::i18n::t("Categories"),
        true,
        None,
    );
    let rows_ui = ui.new_child(egui::UiBuilder::new().accessibility_parent(blank.id));
    // Where the selected row is, for its menu from the keyboard (Shift+F10, the Menu key).
    let mut selected_at: Option<(usize, Pos2)> = None;

    if !is_modern {
        // Connector dots (#6d6d6d, every other pixel), painted first; expander boxes cover them.
        let roots: Vec<usize> = (0..rows.len()).filter(|&k| rows[k].level == 0).collect();
        if let (Some(&first), Some(&last)) = (roots.first(), roots.last()) {
            let mut y = row_top(first);
            while y <= row_top(last) + 8.0 {
                if (y - tree.top()) as i32 % 2 == 0 {
                    winui::px_l(&tp, o.x + 19.0, y, TREE_LINES());
                }
                y += 1.0;
            }
            for &k in &roots {
                let mut x = o.x + 19.0;
                while x <= o.x + 29.0 {
                    winui::px_l(&tp, x, row_top(k) + 8.0, TREE_LINES());
                    x += 2.0;
                }
            }
        }
        for (k, r) in rows.iter().enumerate() {
            if r.level == 1 {
                let mut x = o.x + 38.0;
                while x <= o.x + 48.0 {
                    winui::px_l(&tp, x, row_top(k) + 8.0, TREE_LINES());
                    x += 2.0;
                }
                let is_last = rows.get(k + 1).is_none_or(|n| n.level == 0);
                let start = row_top(k);
                let end = if is_last {
                    row_top(k) + 8.0
                } else {
                    row_top(k) + 15.0
                };
                let mut y = start;
                while y <= end {
                    if (y - tree.top()) as i32 % 2 == 0 {
                        winui::px_l(&tp, o.x + 38.0, y, TREE_LINES());
                    }
                    y += 1.0;
                }
            }
        }
    }

    for (k, r) in rows.iter().enumerate() {
        let top = row_top(k);
        // Rows outside the viewport (icons are not clipped by `tp`).
        if top < area.top() || top >= area.bottom() {
            continue;
        }
        let icon_x = if is_modern {
            area.left() + if r.level == 0 { 16.0 } else { 32.0 }
        } else {
            o.x + if r.level == 0 { 30.0 } else { 49.0 }
        };
        let text_x = if is_modern {
            icon_x + 24.0
        } else {
            o.x + if r.level == 0 { 51.0 } else { 70.0 }
        };
        let ext = text_width_font(&p, font.clone(), crate::i18n::t(&r.label));
        if is_modern {
            let pill = Rect::from_min_max(
                pos2(area.left() + 6.0, top + 2.0),
                pos2(area.right() - 6.0, top + row_h - 2.0),
            );
            if r.selected {
                let sel_bg = if crate::theme::dark() {
                    Color32::from_rgba_unmultiplied(16, 185, 129, 38)
                } else {
                    Color32::from_rgba_unmultiplied(16, 185, 129, 28)
                };
                tp.rect_filled(winui::rect(pill), 10.0, sel_bg);
            } else if ui.rect_contains_pointer(winui::rect(pill)) {
                let hover_c = if crate::theme::dark() {
                    Color32::from_rgb(51, 65, 85)
                } else {
                    Color32::from_rgb(241, 245, 249)
                };
                tp.rect_filled(winui::rect(pill), 10.0, hover_c);
            }
        } else if r.selected {
            winui::fill_l(
                &tp,
                text_x - 2.0,
                top,
                text_x + ext + 1.0,
                top + 15.0,
                winui::SELECTED_INACTIVE(),
            );
        }
        if !is_modern {
            if let Some(open) = r.expander {
                if c.tree_chevrons {
                    winui::fill_l(&tp, o.x + 11.0, top, o.x + 26.0, top + 15.0, TREE_BG());
                    chevron(&tp, o.x, top, open, r.selected);
                } else {
                    expander_box(&tp, o.x, top, open);
                }
            }
        }
        // Icons placed, not mirrored.
        let icon_y = top + if is_modern { (row_h - 16.0) / 2.0 } else { 0.0 };
        icons.paint_in(ui, &tp, r.icon.clone(), pos2(icon_x, icon_y));
        if let Some(name) = r.badge {
            let rc = winui::rect(Rect::from_min_size(
                pos2(icon_x + 7.0, icon_y + 7.0),
                vec2(9.0, 9.0),
            ));
            // The check badge from the theme's icon set, where it has one.
            let themed =
                name == "tick" && crate::themeicons::paint(ui, &tp, "misc-emblem-ok", rc, true);
            if !themed && let Some(src) = silk(name) {
                paint_image(ui, &tp, src, rc);
            }
        }
        let text_y = top + if is_modern { row_h / 2.0 + 4.5 } else { 11.0 };
        let text_ink = if is_modern && r.selected {
            c.accent
        } else {
            TEXT()
        };
        text_font(
            &tp,
            font.clone(),
            text_x,
            text_y,
            crate::i18n::t(&r.label),
            text_ink,
        );
        if keys && r.selected && !is_modern {
            winui::focus_ring(
                &tp,
                winui::rect(Rect::from_min_max(
                    pos2(text_x - 3.0, top),
                    pos2(text_x + ext + 3.0, top + 16.0),
                )),
            );
        }
        if let Some(n) = r.count {
            let count = count_text(n);
            if is_modern {
                let count_num = format!("{n}");
                let cw = text_width_font(&p, winui::segoe(), &count_num);
                let badge_w = (cw + 12.0).max(20.0);
                let badge_h = 20.0;
                let badge_right = if r.expander.is_some() {
                    area.right() - 38.0
                } else {
                    area.right() - 14.0
                };
                let badge_rect = Rect::from_min_size(
                    pos2(badge_right - badge_w, top + (row_h - badge_h) / 2.0),
                    vec2(badge_w, badge_h),
                );
                let (badge_bg, badge_fg) = if r.selected {
                    (Color32::from_rgb(16, 185, 129), Color32::WHITE)
                } else if crate::theme::dark() {
                    (Color32::from_rgb(30, 41, 59), Color32::from_rgb(148, 163, 184))
                } else {
                    (Color32::from_rgb(241, 245, 249), Color32::from_rgb(100, 116, 139))
                };
                tp.rect_filled(winui::rect(badge_rect), 10.0, badge_bg);
                text_font(
                    &tp,
                    winui::segoe(),
                    badge_rect.left() + (badge_w - cw) / 2.0,
                    badge_rect.top() + 14.5,
                    &count_num,
                    badge_fg,
                );
            } else {
                text_font(
                    &tp,
                    font.clone(),
                    text_x + ext + 5.0,
                    text_y,
                    &count,
                    winui::MUTED(),
                );
            }
        }
        if is_modern && let Some(open) = r.expander {
            let btn_sz = 22.0;
            let cx = area.right() - 20.0;
            let cy = top + row_h / 2.0;
            let btn_rect = Rect::from_center_size(pos2(cx, cy), vec2(btn_sz, btn_sz));
            let chev_hovered = ui.rect_contains_pointer(winui::rect(btn_rect));
            let (btn_bg, border_c, glyph_c) = if chev_hovered {
                if crate::theme::dark() {
                    (Color32::from_rgb(71, 85, 105), Color32::from_rgb(100, 116, 139), Color32::WHITE)
                } else {
                    (Color32::from_rgb(203, 213, 225), Color32::from_rgb(148, 163, 184), Color32::from_rgb(15, 23, 42))
                }
            } else if crate::theme::dark() {
                (Color32::from_rgb(51, 65, 85), Color32::from_rgb(71, 85, 105), Color32::from_rgb(226, 232, 240))
            } else {
                (Color32::from_rgb(226, 232, 240), Color32::from_rgb(203, 213, 225), Color32::from_rgb(71, 85, 105))
            };
            tp.rect_filled(winui::rect(btn_rect), 6.0, btn_bg);
            tp.rect_stroke(winui::rect(btn_rect), 6.0, Stroke::new(1.0, border_c), egui::StrokeKind::Inside);

            let stroke = Stroke::new(1.8, glyph_c);
            if open {
                tp.line_segment([pos2(cx - 3.5, cy - 2.0), pos2(cx, cy + 1.5)], stroke);
                tp.line_segment([pos2(cx, cy + 1.5), pos2(cx + 3.5, cy - 2.0)], stroke);
            } else {
                tp.line_segment([pos2(cx - 2.0, cy - 3.5), pos2(cx + 1.5, cy)], stroke);
                tp.line_segment([pos2(cx + 1.5, cy), pos2(cx - 2.0, cy + 3.5)], stroke);
            }
        }

        let row = winui::rect(
            Rect::from_min_size(pos2(area.left(), top), vec2(area.width(), row_h)).intersect(area),
        );
        let resp = rows_ui.interact(
            row,
            egui::Id::unique(("tree-row", k)),
            winui::pointer_only(),
        );
        // A tree item: its name (with its count), selected, open or shut, its level; Click
        // selects it (egui's own), Expand / Collapse open and shut it, Show context menu opens
        // its menu.
        {
            use egui::accesskit::Action;
            let name = match r.count {
                Some(n) => format!("{} {}", crate::i18n::t(&r.label), count_text(n)),
                None => crate::i18n::t(&r.label).to_owned(),
            };
            winui::expose(&resp, egui::Role::TreeItem, &name, true, None);
            let (expand, collapse, menu) = ui.input(|i| {
                (
                    i.has_accesskit_action_request(resp.id, Action::Expand),
                    i.has_accesskit_action_request(resp.id, Action::Collapse),
                    i.has_accesskit_action_request(resp.id, Action::ShowContextMenu),
                )
            });
            let (level, selected, open) = (r.level, r.selected, r.expander);
            ui.ctx().accesskit_node_builder(resp.id, |n| {
                n.set_selected(selected);
                n.set_level(level + 1);
                n.add_action(Action::ShowContextMenu);
                if let Some(open) = open {
                    n.set_expanded(open);
                    n.add_action(if open {
                        Action::Collapse
                    } else {
                        Action::Expand
                    });
                }
            });
            let under = resp.rect.left_bottom() + vec2(24.0, 0.0);
            if r.selected {
                selected_at = Some((k, under));
            }
            if (expand && r.expander == Some(false)) || (collapse && r.expander == Some(true)) {
                event = Some(TreeEvent::Toggle(k));
            }
            if menu {
                event = Some(TreeEvent::Context(k, under));
            }
            // An assistive action moves no pointer: the frame showing its result is asked for.
            if expand || collapse || menu {
                ui.ctx().request_repaint();
            }
        }
        if resp.clicked() {
            let on_expander = resp.interact_pointer_pos().is_some_and(|q| {
                let x = winui::mx(q.x);
                if is_modern {
                    x >= area.right() - 36.0
                } else {
                    x >= o.x + 11.0 && x <= o.x + 26.0
                }
            });
            event = Some(if on_expander && r.expander.is_some() {
                TreeEvent::Toggle(k)
            } else {
                TreeEvent::Select(k)
            });
        }
        if resp.double_clicked() && r.expander.is_some() {
            event = Some(TreeEvent::Toggle(k));
        }
        if resp.secondary_clicked()
            && let Some(at) = resp.interact_pointer_pos()
        {
            event = Some(TreeEvent::Context(k, at));
        }
    }
    // The selected node's menu from the keyboard, under the node: Shift+F10 or the Menu key.
    if keys
        && let Some((k, at)) = selected_at
        && ui.input_mut(|i| {
            i.consume_key(egui::Modifiers::SHIFT, egui::Key::F10)
                || i.consume_key(egui::Modifiers::NONE, egui::Key::Menu)
        })
    {
        event = Some(TreeEvent::Context(k, at));
    }

    if need_v {
        // At the trailing edge: the left in RTL.
        let track = winui::rect(Rect::from_min_max(
            pos2(area.right(), tree.top()),
            pos2(tree.right(), area.bottom()),
        ));
        let total = rows.len() as f32;
        if scrollbar::bar(
            ui,
            scroll_id.with("v"),
            track,
            true,
            total,
            page_rows,
            1.0,
            &mut vpos,
        ) {
            vpos = vpos.round();
            ui.ctx().request_repaint();
        }
    }
    if need_h {
        let track = winui::rect(Rect::from_min_max(
            pos2(tree.left(), area.bottom()),
            pos2(area.right(), tree.bottom()),
        ));
        if scrollbar::bar(
            ui,
            scroll_id.with("h"),
            track,
            false,
            content_w,
            area.width(),
            15.0,
            &mut hpos,
        ) {
            hpos = hpos.round();
            ui.ctx().request_repaint();
        }
    }
    if need_v && need_h {
        scrollbar::corner(ui, area.right(), area.bottom());
    }
    ui.ctx()
        .data_mut(|d| d.insert_temp(scroll_id, (vpos, hpos)));
    if is_modern {
        let div_c = if crate::theme::dark() {
            egui::Color32::from_rgb(30, 41, 59)
        } else {
            egui::Color32::from_rgb(241, 245, 249)
        };
        p.line_segment(
            [pos2(o.x + left_pad, bottom - limiter_h), pos2(right, bottom - limiter_h)],
            egui::Stroke::new(1.0, div_c),
        );
        let card_rect = Rect::from_min_max(
            pos2(o.x + left_pad + 10.0, bottom - bot_pad - 54.0),
            pos2(right - 10.0, bottom - bot_pad - 10.0),
        );
        let resp = ui.interact(
            winui::rect(card_rect),
            egui::Id::unique("sidebar-speed-limiter"),
            winui::pointer_only(),
        );
        let (card_bg, card_border) = if resp.hovered() {
            (
                if crate::theme::dark() { egui::Color32::from_rgb(30, 41, 59) } else { egui::Color32::from_rgb(248, 250, 252) },
                if crate::theme::dark() { egui::Color32::from_rgb(71, 85, 105) } else { egui::Color32::from_rgb(203, 213, 225) },
            )
        } else {
            (
                if crate::theme::dark() { egui::Color32::from_rgb(15, 23, 42) } else { egui::Color32::WHITE },
                if crate::theme::dark() { egui::Color32::from_rgb(51, 65, 85) } else { egui::Color32::from_rgb(226, 232, 240) },
            )
        };
        p.rect_filled(winui::rect(card_rect), 12.0, card_bg);
        p.rect_stroke(winui::rect(card_rect), 12.0, egui::Stroke::new(1.0, card_border), egui::StrokeKind::Inside);

        // Speedometer icon: emerald arc and needle
        let icon_cx = card_rect.left() + 20.0;
        let icon_cy = card_rect.center().y;
        p.circle_stroke(pos2(icon_cx, icon_cy), 7.0, egui::Stroke::new(1.8, egui::Color32::from_rgb(16, 185, 129)));
        p.line_segment([pos2(icon_cx, icon_cy), pos2(icon_cx + 4.5, icon_cy - 4.5)], egui::Stroke::new(1.8, egui::Color32::from_rgb(16, 185, 129)));

        // Label: "Speed Limiter"
        let font = winui::segoe();
        text_font(&p, font.clone(), card_rect.left() + 36.0, card_rect.center().y + 4.5, "Speed Limiter", TEXT());

        // Pill badge on the right: "Off"
        let badge_w = 36.0;
        let badge_h = 22.0;
        let badge_rect = Rect::from_min_size(
            pos2(card_rect.right() - badge_w - 10.0, card_rect.center().y - (badge_h / 2.0)),
            egui::vec2(badge_w, badge_h),
        );
        let badge_bg = if crate::theme::dark() { egui::Color32::from_rgb(30, 41, 59) } else { egui::Color32::from_rgb(241, 245, 249) };
        let badge_fg = if crate::theme::dark() { egui::Color32::from_rgb(148, 163, 184) } else { egui::Color32::from_rgb(71, 85, 105) };
        p.rect_filled(winui::rect(badge_rect), 6.0, badge_bg);
        text_font(&p, font.clone(), badge_rect.left() + 8.5, badge_rect.center().y + 4.0, "Off", badge_fg);

        if resp.clicked() {
            event = Some(TreeEvent::OpenSpeedSettings);
        }
    }
    event
}

/// Light mode's classic 9×9 +/- box at x 15..23, y top+4..top+12 (light-main.md grids).
fn expander_box(p: &egui::Painter, ox: f32, top: f32, open: bool) {
    const MINUS: [[u32; 9]; 9] = [
        [
            0xbabbbc, 0x919191, 0x919191, 0x919191, 0x919191, 0x919191, 0x919191, 0x919191,
            0xbabbbc,
        ],
        [
            0x919191, 0xfafbfb, 0xfafbfb, 0xfafbfb, 0xfafbfb, 0xfafbfb, 0xfafbfb, 0xfafbfb,
            0x919191,
        ],
        [
            0x919191, 0xfafbfb, 0xfafbfb, 0xfafbfb, 0xfafbfb, 0xfafbfb, 0xfafbfb, 0xfafbfb,
            0x919191,
        ],
        [
            0x919191, 0xfafbfb, 0xfafbfb, 0xfafbfb, 0xfafbfb, 0xfafbfb, 0xfafbfb, 0xfafbfb,
            0x919191,
        ],
        [
            0x919191, 0xededec, 0x4b63a7, 0x4b63a7, 0x4b63a7, 0x4b63a7, 0x4b63a7, 0xededec,
            0x919191,
        ],
        [
            0x919191, 0xededec, 0xededec, 0xededec, 0xededec, 0xededec, 0xededec, 0xededec,
            0x919191,
        ],
        [
            0x919191, 0xe3e3e3, 0xe3e3e3, 0xe3e3e3, 0xe3e3e3, 0xe3e3e3, 0xe3e3e3, 0xe3e3e3,
            0x919191,
        ],
        [
            0x919191, 0xe3e3e3, 0xe3e3e3, 0xe3e3e3, 0xe3e3e3, 0xe3e3e3, 0xe3e3e3, 0xe3e3e3,
            0x919191,
        ],
        [
            0xbabbbc, 0x919191, 0x919191, 0x919191, 0x919191, 0x919191, 0x919191, 0x919191,
            0xbabbbc,
        ],
    ];
    const PLUS: [[u32; 9]; 9] = [
        [
            0xbabbbc, 0x919191, 0x919191, 0x919191, 0x919191, 0x919191, 0x919191, 0x919191,
            0xbabbbc,
        ],
        [
            0x919191, 0xfcfcfc, 0xfcfcfc, 0xfcfcfc, 0xfcfcfc, 0xfcfcfc, 0xfcfcfc, 0xfcfcfc,
            0x919191,
        ],
        [
            0x919191, 0xfcfcfc, 0xfcfcfc, 0xfcfcfc, 0x294272, 0xfcfcfc, 0xfcfcfc, 0xfcfcfc,
            0x919191,
        ],
        [
            0x919191, 0xfafbfb, 0xfafbfb, 0xfafbfb, 0x294272, 0xfafbfb, 0xfafbfb, 0xfafbfb,
            0x919191,
        ],
        [
            0x919191, 0xfafbfb, 0x4b63a7, 0x4b63a7, 0x4b63a7, 0x4b63a7, 0x4b63a7, 0xfafbfb,
            0x919191,
        ],
        [
            0x919191, 0xededec, 0xededec, 0xededec, 0x294272, 0xededec, 0xededec, 0xededec,
            0x919191,
        ],
        [
            0x919191, 0xe3e3e3, 0xe3e3e3, 0xe3e3e3, 0x294272, 0xe3e3e3, 0xe3e3e3, 0xe3e3e3,
            0x919191,
        ],
        [
            0x919191, 0xe3e3e3, 0xe3e3e3, 0xe3e3e3, 0xe3e3e3, 0xe3e3e3, 0xe3e3e3, 0xe3e3e3,
            0x919191,
        ],
        [
            0xbabbbc, 0x919191, 0x919191, 0x919191, 0x919191, 0x919191, 0x919191, 0x919191,
            0xbabbbc,
        ],
    ];
    let grid = if open { &MINUS } else { &PLUS };
    for (dy, row) in grid.iter().enumerate() {
        for (dx, v) in row.iter().enumerate() {
            let c = rgb((v >> 16) as u8, (v >> 8) as u8, *v as u8);
            winui::px_l(p, ox + 15.0 + dx as f32, top + 4.0 + dy as f32, c);
        }
    }
}

/// Explorer dark-theme chevrons, pixel for pixel (spec "Expander chevrons").
fn chevron(p: &egui::Painter, ox: f32, top: f32, open: bool, bright: bool) {
    const COLLAPSED: [[u8; 6]; 9] = [
        [0xc9, 0x95, 0x23, 0x20, 0x20, 0x20],
        [0x8d, 0xd9, 0x97, 0x23, 0x20, 0x20],
        [0x21, 0x8d, 0xd9, 0x97, 0x23, 0x20],
        [0x20, 0x21, 0x8d, 0xd9, 0x97, 0x22],
        [0x20, 0x20, 0x22, 0xbb, 0xd9, 0x4a],
        [0x20, 0x21, 0x8d, 0xd9, 0x8d, 0x21],
        [0x21, 0x8d, 0xd9, 0x8d, 0x21, 0x20],
        [0x8d, 0xd9, 0x8d, 0x21, 0x20, 0x20],
        [0xc9, 0x8d, 0x21, 0x20, 0x20, 0x20],
    ];
    const EXPANDED: [[u8; 9]; 6] = [
        [0xec, 0xad, 0x23, 0x20, 0x20, 0x20, 0x21, 0xa3, 0xec],
        [0xa3, 0xff, 0xb0, 0x23, 0x20, 0x21, 0xa4, 0xff, 0xa3],
        [0x21, 0xa4, 0xff, 0xa8, 0x22, 0xa4, 0xff, 0xa4, 0x21],
        [0x20, 0x21, 0xa4, 0xff, 0xdb, 0xff, 0xa4, 0x21, 0x20],
        [0x20, 0x20, 0x21, 0xa4, 0xff, 0xa4, 0x21, 0x20, 0x20],
        [0x20, 0x20, 0x20, 0x21, 0x53, 0x21, 0x20, 0x20, 0x20],
    ];
    let _ = bright;
    // Measured over the dark tree (#20) in white: coverage, so the theme's tree and ink show.
    let (bg, ink) = (TREE_BG(), TREE_EXPANDER());
    let grey = |v: u8| {
        let a = (v as f32 - 32.0) / 223.0;
        let mix = |b: u8, i: u8| (b as f32 + (i as f32 - b as f32) * a).round() as u8;
        rgb(
            mix(bg.r(), ink.r()),
            mix(bg.g(), ink.g()),
            mix(bg.b(), ink.b()),
        )
    };
    if open {
        for (dy, row) in EXPANDED.iter().enumerate() {
            for (dx, v) in row.iter().enumerate() {
                if *v != 0x20 {
                    winui::px_l(p, ox + 14.0 + dx as f32, top + 5.0 + dy as f32, grey(*v));
                }
            }
        }
    } else {
        for (dy, row) in COLLAPSED.iter().enumerate() {
            for (dx, v) in row.iter().enumerate() {
                if *v != 0x20 {
                    winui::px_l(p, ox + 16.0 + dx as f32, top + 4.0 + dy as f32, grey(*v));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FOCUS_GRACE, FocusVerdict, MenuBarEvent, MenuEntry, MenuFocus, MenuKind, heading_into,
        item_at, menu_viewport, menubar_next,
    };
    use egui::{Rect, pos2};

    /// #277: on X11 the menu bar's menus share one window per level, so going from title to
    /// title moves it (a compositor has no new window to fade in); elsewhere, and for every
    /// other menu, a window of their own as before.
    #[test]
    fn the_bar_menus_share_one_window_on_x11() {
        let bar = |i: usize, depth, shared| {
            menu_viewport(MenuKind::Bar, &format!("menu-{i}"), depth, shared)
        };
        // Tasks, File, Downloads, View, Help: one window (and one for their submenus).
        for i in 1..5 {
            assert_eq!(bar(0, 0, true), bar(i, 0, true));
            assert_eq!(bar(0, 1, true), bar(i, 1, true));
        }
        assert_ne!(bar(0, 0, true), bar(0, 1, true));
        // Without sharing (Windows, macOS, native Wayland): one per menu, as before.
        assert_ne!(bar(0, 0, false), bar(1, 0, false));
        assert_eq!(
            bar(2, 0, false),
            egui::ViewportId::from_hash_of(("menu", "menu-2", 0usize))
        );
        // A context menu and a drop-down list keep their own window, shared or not.
        for shared in [true, false] {
            let ctx = menu_viewport(MenuKind::Context, "popup-1", 0, shared);
            let list = menu_viewport(MenuKind::List, "drop-x", 0, shared);
            assert_ne!(ctx, bar(0, 0, shared));
            assert_ne!(list, bar(0, 0, shared));
            assert_ne!(ctx, list);
            assert_eq!(
                ctx,
                egui::ViewportId::from_hash_of(("menu", "popup-1", 0usize))
            );
        }
    }

    /// Runs `events` from no menu open; the open menu after each.
    fn run(events: &[MenuBarEvent]) -> Vec<Option<usize>> {
        let mut open = None;
        events
            .iter()
            .map(|e| {
                open = menubar_next(open, *e);
                open
            })
            .collect()
    }

    #[test]
    fn a_title_clicked_after_escape_opens_its_menu_again() {
        // Click Tasks, Escape (the pointer still on Tasks: no press), click Tasks.
        let steps = run(&[
            MenuBarEvent::Title {
                i: 0,
                released: true,
            },
            MenuBarEvent::Dismissed {
                press_on_title: false,
            },
            MenuBarEvent::Title {
                i: 0,
                released: true,
            },
        ]);
        assert_eq!(steps, [Some(0), None, Some(0)]);
    }

    #[test]
    fn clicking_the_open_menus_title_shuts_it() {
        // The press shuts the menu's window; its release, on the same title, shuts the menu.
        let steps = run(&[
            MenuBarEvent::Title {
                i: 1,
                released: true,
            },
            MenuBarEvent::Dismissed {
                press_on_title: true,
            },
            MenuBarEvent::Title {
                i: 1,
                released: true,
            },
            MenuBarEvent::Title {
                i: 1,
                released: true,
            },
        ]);
        assert_eq!(steps, [Some(1), Some(1), None, Some(1)]);
    }

    #[test]
    fn another_title_opens_its_menu_from_an_open_one() {
        let steps = run(&[
            MenuBarEvent::Title {
                i: 0,
                released: true,
            },
            // Hovered while Tasks is open: File opens.
            MenuBarEvent::Hover(1),
            // Pressed and released on Downloads.
            MenuBarEvent::Dismissed {
                press_on_title: true,
            },
            MenuBarEvent::Title {
                i: 2,
                released: true,
            },
        ]);
        assert_eq!(steps, [Some(0), Some(1), Some(1), Some(2)]);
    }

    /// The focus over a menu's passes (`(focused, time)` each), as `MenuFocus` judges it, the
    /// system saying nothing of who has it (Linux, macOS).
    fn focus_run(passes: &[(bool, f64)]) -> Vec<FocusVerdict> {
        let mut f = MenuFocus::default();
        passes.iter().map(|&(on, t)| f.pass(on, false, t)).collect()
    }

    /// #859: a click on the desktop (Windows 10's video), the taskbar or another program's window
    /// while a menu is open. The system says another program is in front: the menu closes on
    /// the very pass that learns the focus left, not FOCUS_GRACE later.
    #[test]
    fn a_click_in_another_program_closes_the_menu_at_once() {
        let mut f = MenuFocus::default();
        assert_eq!(f.pass(true, false, 0.0), FocusVerdict::Here);
        assert_eq!(f.pass(true, false, 0.5), FocusVerdict::Here);
        assert_eq!(f.pass(false, true, 1.0), FocusVerdict::Left);
        // Away from our windows without another program in front (focus passing between our
        // own windows): the grace, as on Linux.
        let mut f = MenuFocus::default();
        f.pass(true, false, 0.0);
        assert_eq!(
            f.pass(false, false, 1.0),
            FocusVerdict::Away { wait: FOCUS_GRACE }
        );
        assert_eq!(f.pass(false, true, 1.01), FocusVerdict::Left);
        // A menu opened while another program was in front (a tray menu): nothing to lose.
        let mut f = MenuFocus::default();
        assert_eq!(f.pass(false, true, 0.0), FocusVerdict::Here);
        assert_eq!(f.pass(false, true, 5.0), FocusVerdict::Here);
    }

    /// #250: Tasks open, Import's submenu open with the focus (KWin, sway), the pointer back
    /// onto another item: the submenu's window closes and hands the focus on, and for a few
    /// frames no window of ours has it. The menu stays.
    #[test]
    fn a_submenu_closing_with_the_focus_leaves_its_menu_open() {
        let v = focus_run(&[
            (true, 0.0),   // the main window, then the menu's window
            (true, 0.5),   // the submenu's window took it
            (false, 1.0),  // the submenu closed: nobody has it
            (false, 1.02), // the next frame
            (true, 1.05),  // the main window (or the menu's) got it back
            (true, 1.6),
        ]);
        assert!(v.iter().all(|x| *x != FocusVerdict::Left), "{v:?}");
        assert_eq!(v[2], FocusVerdict::Away { wait: FOCUS_GRACE });
        // Gone again later: counted afresh.
        let v = focus_run(&[
            (true, 0.0),
            (false, 1.0),
            (true, 1.3),
            (false, 5.0),
            (false, 5.3),
        ]);
        assert!(v.iter().all(|x| *x != FocusVerdict::Left), "{v:?}");
    }

    /// Another app takes the focus (a click on its window, Alt+Tab): the menu closes once the
    /// focus has stayed away, and asks to be looked at again until then.
    #[test]
    fn the_focus_gone_to_another_app_closes_the_menu() {
        let v = focus_run(&[(true, 0.0), (false, 2.0), (false, 2.1), (false, 2.5)]);
        assert_eq!(v[0], FocusVerdict::Here);
        assert!(matches!(v[1], FocusVerdict::Away { wait } if (wait - FOCUS_GRACE).abs() < 1e-9));
        assert!(
            matches!(v[2], FocusVerdict::Away { wait } if (wait - (FOCUS_GRACE - 0.1)).abs() < 1e-9)
        );
        assert_eq!(v[3], FocusVerdict::Left);
        // A menu opened while another app was in front (a tray menu): no focus to lose.
        let v = focus_run(&[(false, 0.0), (false, 1.0), (false, 9.0)]);
        assert!(v.iter().all(|x| *x == FocusVerdict::Here), "{v:?}");
    }

    /// #277: Help → About clicked, About closed, then the pointer over the titles: nothing opens
    /// until a click. The bar is as inactive after a command as before the first click.
    #[test]
    fn after_a_command_the_pointer_alone_opens_nothing() {
        use MenuBarEvent::{Chosen, Dismissed, Hover, Title};
        let steps = run(&[
            // Before any click: the pointer over the titles opens nothing.
            Hover(0),
            Hover(4),
            // Help clicked; the pointer on its own title keeps it.
            Title {
                i: 4,
                released: true,
            },
            Hover(4),
            // About chosen: the menu closes.
            Chosen,
            // About opens and takes the focus, is closed, the main window has it again: no
            // event of the bar's (its menu is closed). A stray dismissal changes nothing.
            Dismissed {
                press_on_title: false,
            },
            // The pointer over every title: still nothing.
            Hover(0),
            Hover(1),
            Hover(2),
            Hover(3),
            Hover(4),
            // A click opens a menu again.
            Title {
                i: 1,
                released: true,
            },
        ]);
        assert_eq!(
            steps,
            [
                None,
                None,
                Some(4),
                Some(4),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(1)
            ]
        );
        // The same after a command chosen from the keyboard (Alt+H, A), and after Escape or a
        // click elsewhere.
        for end in [
            Chosen,
            Dismissed {
                press_on_title: false,
            },
        ] {
            assert_eq!(
                run(&[MenuBarEvent::Open(4), end, Hover(0), Hover(3)]),
                [Some(4), None, None, None]
            );
        }
    }

    /// #579: Tasks with Export's submenu open to the right (menu 200 wide, Export's slot at
    /// y 190..212). The pointer going on down or up the menu is not heading into the submenu, so
    /// the next item closes it at once; crossing the item below diagonally towards the submenu
    /// keeps it, as does going there on the left side of a right-to-left menu.
    #[test]
    fn a_submenu_stays_open_only_while_the_pointer_heads_into_it() {
        let sub = Rect::from_min_max(pos2(194.0, 187.0), pos2(330.0, 236.0));
        let left_export = pos2(120.0, 205.0);
        // Down to Import, up to Show drop target: straight, not towards the submenu.
        assert!(!heading_into(left_export, pos2(118.0, 216.0), sub));
        assert!(!heading_into(left_export, pos2(121.0, 170.0), sub));
        assert!(!heading_into(left_export, pos2(60.0, 225.0), sub));
        // Diagonally across Import, towards the submenu.
        assert!(heading_into(left_export, pos2(150.0, 214.0), sub));
        assert!(heading_into(left_export, pos2(185.0, 230.0), sub));
        // Right to left: the submenu on the left.
        let rtl = Rect::from_min_max(pos2(-130.0, 187.0), pos2(6.0, 236.0));
        assert!(heading_into(pos2(80.0, 205.0), pos2(50.0, 214.0), rtl));
        assert!(!heading_into(pos2(80.0, 205.0), pos2(82.0, 216.0), rtl));
    }

    /// #579, the menu itself (egui without a window system: each level drawn in the main
    /// window): Tasks-like, Import's submenu open, the pointer going on up the menu closes it at
    /// once and lights only the item it is on, where before the submenu stayed open (and its
    /// entry lit) until the pointer had rested 0.35 s on one item, so never while it moved on.
    /// Crossing an item on the way into the submenu keeps it.
    #[test]
    fn the_pointer_on_another_item_closes_the_submenu_at_once() {
        // Labels through names, so tools/i18n/extract.py doesn't take them for texts to translate.
        let [rss, show_drop, import, file, exit] = [
            "RSS feeds...",
            "Show drop target",
            "Import",
            "From a file",
            "Exit",
        ];
        let entries: Vec<MenuEntry<u8>> = vec![
            MenuEntry::Item(rss, 0, true, false),
            MenuEntry::Item(show_drop, 1, true, false),
            MenuEntry::Sep,
            MenuEntry::Sub(import, true, vec![MenuEntry::Item(file, 2, true, false)]),
            MenuEntry::Item(exit, 3, true, false),
        ];
        let ctx = egui::Context::default();
        let sub_key = egui::Id::unique(("popup-sub", "t579"));
        let mut t = 0.0;
        let mut frame = |pointer: Option<egui::Pos2>| {
            t += 0.016;
            let mut raw = egui::RawInput {
                time: Some(t),
                screen_rect: Some(Rect::from_min_size(
                    pos2(0.0, 0.0),
                    egui::vec2(1200.0, 800.0),
                )),
                ..Default::default()
            };
            if let Some(p) = pointer {
                raw.events.push(egui::Event::PointerMoved(p));
            }
            let mut out = ctx.run_ui(raw, |ui| {
                let mut result = None;
                super::popup_level(
                    ui.ctx(),
                    MenuKind::Bar,
                    "t579",
                    pos2(100.0, 100.0),
                    None,
                    &entries,
                    sub_key,
                    &mut result,
                    0,
                    &|_| None,
                );
            });
            out.textures_delta.clear();
        };
        frame(None);
        let o = ctx
            .data(|d| d.get_temp::<egui::Pos2>(sub_key.with(("origin", 0usize))))
            .expect("drawn");
        // Slots: 3..25, 25..47, separator, Import 56..78, Exit 78..100.
        let at = |x: f32, y: f32| Some(o + egui::vec2(x, y));
        let open = |ctx: &egui::Context| ctx.data(|d| d.get_temp::<usize>(sub_key));
        for _ in 0..3 {
            frame(at(60.0, 67.0));
        }
        assert_eq!(open(&ctx), Some(3), "Import's submenu opens");
        // Straight up to Show drop target: closed in that pass.
        frame(at(60.0, 40.0));
        frame(at(60.0, 38.0));
        assert_eq!(open(&ctx), None, "closed at once");

        // Again, then diagonally across Exit towards the submenu: kept.
        for _ in 0..3 {
            frame(at(60.0, 67.0));
        }
        assert_eq!(open(&ctx), Some(3));
        let right = ctx
            .data(|d| d.get_temp::<egui::Pos2>(sub_key.with(("origin", 1usize))))
            .expect("submenu drawn")
            .x;
        // Most of the way to the submenu, a little lower: over Exit.
        let x = o.x + (right - o.x) * 0.9 - o.x;
        frame(at(x, 80.0));
        assert_eq!(open(&ctx), Some(3), "kept on the way into it");
    }

    /// #733: a tap on a touch screen chooses the item under the finger. egui-winit makes the
    /// tap a press and a release of the pointer, then lets go of it (PointerGone) as the finger
    /// lifts, in the same pass as the release (in one pass with the press too, for a quick tap):
    /// the item was looked for under a pointer that was gone, and none was chosen.
    #[test]
    fn a_tap_chooses_the_item_under_the_finger() {
        let [a, b, c] = ["a", "b", "c"]; // not texts to translate (tools/i18n/extract.py)
        let entries: Vec<MenuEntry<u8>> = vec![
            MenuEntry::Item(a, 0, true, false),
            MenuEntry::Sep,
            MenuEntry::Item(b, 1, true, false),
            MenuEntry::Item(c, 2, true, false),
        ];
        // Each pass's events; what the menu chose.
        let run = |passes: &[&dyn Fn(egui::Pos2) -> Vec<egui::Event>]| {
            let ctx = egui::Context::default();
            let sub_key = egui::Id::unique(("popup-sub", "t733"));
            let mut t = 0.0;
            let mut chosen = None;
            let mut pass = |events: Vec<egui::Event>| {
                t += 0.016;
                let raw = egui::RawInput {
                    time: Some(t),
                    screen_rect: Some(Rect::from_min_size(
                        pos2(0.0, 0.0),
                        egui::vec2(1200.0, 800.0),
                    )),
                    events,
                    ..Default::default()
                };
                let mut out = ctx.run_ui(raw, |ui| {
                    let mut result = None;
                    super::popup_level(
                        ui.ctx(),
                        MenuKind::Context,
                        "t733",
                        pos2(100.0, 100.0),
                        None,
                        &entries,
                        sub_key,
                        &mut result,
                        0,
                        &|_| None,
                    );
                    if result.is_some() {
                        chosen = result;
                    }
                });
                out.textures_delta.clear();
            };
            pass(Vec::new());
            let o = ctx
                .data(|d| d.get_temp::<egui::Pos2>(sub_key.with(("origin", 0usize))))
                .expect("drawn");
            // "b": below "a" (3..25) and the separator (25..34), at 34..56.
            let on_b = o + egui::vec2(60.0, 45.0);
            for p in passes {
                pass(p(on_b));
            }
            chosen
        };
        let button = |pos, pressed| egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: Default::default(),
        };
        let down = |p: egui::Pos2| vec![egui::Event::PointerMoved(p), button(p, true)];
        let up_gone = |p: egui::Pos2| vec![button(p, false), egui::Event::PointerGone];
        let tap = |p: egui::Pos2| {
            vec![
                egui::Event::PointerMoved(p),
                button(p, true),
                button(p, false),
                egui::Event::PointerGone,
            ]
        };
        let up = |p: egui::Pos2| vec![button(p, false)];
        // A tap: the finger down in one pass, up (and the pointer gone) in the next.
        assert_eq!(run(&[&down, &up_gone]), Some(1));
        // A quick tap, all in one pass.
        assert_eq!(run(&[&tap]), Some(1));
        // The mouse, as before.
        assert_eq!(run(&[&down, &up]), Some(1));
    }

    /// #579: the entry under the pointer, separators and padding none.
    #[test]
    fn the_item_under_the_pointer() {
        let [a, b, c] = ["a", "b", "c"]; // not texts to translate (tools/i18n/extract.py)
        let entries: Vec<MenuEntry<u8>> = vec![
            MenuEntry::Item(a, 0, true, false),
            MenuEntry::Sep,
            MenuEntry::Sub(b, true, vec![]),
            MenuEntry::Item(c, 1, true, false),
        ];
        let o = pos2(0.0, 0.0);
        assert_eq!(item_at(&entries, o, 200.0, pos2(50.0, 10.0)), Some(0));
        // The separator: 25..34.
        assert_eq!(item_at(&entries, o, 200.0, pos2(50.0, 29.0)), None);
        assert_eq!(item_at(&entries, o, 200.0, pos2(50.0, 40.0)), Some(2));
        assert_eq!(item_at(&entries, o, 200.0, pos2(50.0, 60.0)), Some(3));
        // The padding at the sides.
        assert_eq!(item_at(&entries, o, 200.0, pos2(3.0, 40.0)), None);
    }

    /// With a menu open, the pointer moves it from title to title, as in Windows' menu bars
    /// (and IDM's); on its own title it stays.
    #[test]
    fn with_a_menu_open_the_pointer_switches_menus() {
        use MenuBarEvent::{Hover, Title};
        let steps = run(&[
            Title {
                i: 0,
                released: true,
            },
            Hover(1),
            Hover(1),
            Hover(3),
            Hover(0),
        ]);
        assert_eq!(steps, [Some(0), Some(1), Some(1), Some(3), Some(0)]);
    }

    #[test]
    fn a_press_elsewhere_a_choice_or_the_focus_lost_closes_the_menu() {
        for end in [
            MenuBarEvent::Dismissed {
                press_on_title: false,
            },
            MenuBarEvent::Chosen,
        ] {
            assert_eq!(run(&[MenuBarEvent::Open(3), end]), [Some(3), None]);
        }
        // Left / Right from the keyboard move to the neighbour.
        assert_eq!(
            run(&[MenuBarEvent::Open(0), MenuBarEvent::Open(4)]),
            [Some(0), Some(4)]
        );
    }
}
