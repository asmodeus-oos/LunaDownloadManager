//! Add or edit a category (D-021): the user's own or a built-in one by file type (#93: name,
//! file types, folder), General's and Torrents' folder only. Opened from Options → Save to
//! (New, Edit...) and the Categories right-click menu (Add category, Properties); Delete
//! removes a category (General and Torrents stay), and Options → Save to restores the
//! built-in ones as shipped.

use std::path::Path;

use egui::{Color32, Rect, Stroke, pos2};

use crate::dialogs::{Outcome, window};
use crate::model::{BuiltinEdit, Category, CustomCategory};
use crate::settings::Settings;
use crate::winui::{self, ButtonKind, LIGHT_BG, LIGHT_DISABLED, LIGHT_TEXT, r};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Target {
    New,
    /// A category of the user's or a built-in one (General and Torrents: only their folder).
    Edit(Category),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CategoryEdit {
    pub target: Target,
    pub name: String,
    pub types: String,
    /// The folder typed (empty: the default one).
    pub folder: String,
}

pub struct CategoryDialog {
    target: Target,
    name: String,
    types: String,
    folder: String,
    /// Name has been given the focus (once, when the dialog opens).
    focused: bool,
}

impl CategoryDialog {
    pub fn new() -> Self {
        CategoryDialog {
            target: Target::New,
            name: String::new(),
            types: String::new(),
            folder: String::new(),
            focused: false,
        }
    }

    /// Edit `cat`, whose folder is `folder` now.
    pub fn edit(cat: Category, folder: &Path) -> Self {
        CategoryDialog {
            target: Target::Edit(cat),
            // A built-in category's name as the interface writes it (a name the user gave stays
            // as typed).
            name: cat.shown(),
            types: match cat {
                Category::Custom(id) => crate::model::custom(id)
                    .map(|c| c.types)
                    .unwrap_or_default(),
                c => c.types_text(),
            },
            folder: folder.display().to_string(),
            focused: false,
        }
    }

    /// `names`: every category's name now (a new name must be another).
    pub fn show(&mut self, ctx: &egui::Context, names: &[String]) -> Outcome<CategoryEdit> {
        // General and Torrents: only the folder changes.
        let built_in = matches!(self.target, Target::Edit(c) if !c.editable());
        let title = match self.target {
            Target::New => crate::i18n::mark("Add category"),
            Target::Edit(_) => crate::i18n::mark("Category properties"),
        };
        let own = match self.target {
            Target::Edit(c) => c.shown(),
            Target::New => String::new(),
        };
        let is_modern = crate::theme::metrics().flyout_radius >= 12.0;
        let dlg_size = if is_modern { [480.0, 240.0] } else { [420.0, 196.0] };
        let out = window(
            ctx,
            "category-edit",
            title,
            dlg_size,
            false,
            LIGHT_BG(),
            |ui, o| {
                let p = ui.painter().clone();
                let text_color = if is_modern { winui::TEXT() } else { LIGHT_TEXT() };
                if is_modern {
                    let dark = crate::theme::dark();
                    let card_bg = if dark { Color32::from_rgb(30, 41, 59) } else { Color32::WHITE };
                    let card_border = if dark { Color32::from_rgb(51, 65, 85) } else { Color32::from_rgb(226, 232, 240) };
                    let card_rect = Rect::from_min_max(pos2(o.x + 8.0, o.y + 6.0), pos2(o.x + 472.0, o.y + 234.0));
                    p.rect_filled(winui::rect(card_rect), 16.0, card_bg);
                    p.rect_stroke(winui::rect(card_rect), 16.0, Stroke::new(1.0, card_border), egui::StrokeKind::Inside);
                }

                let label_x = if is_modern { o.x + 100.0 } else { o.x + 90.0 };
                let label = |y: f32, s: &str| {
                    winui::text_right(&p, label_x, o.y + y, crate::i18n::t(s), text_color)
                };
                let input_x = if is_modern { 108.0 } else { 98.0 };
                let input_w = if is_modern { 348.0 } else { 310.0 };
                let input_h = if is_modern { 30.0 } else { 23.0 };

                label(26.0, crate::i18n::mark("Name:"));
                let name_box = if is_modern {
                    winui::edit(ui, r(o, input_x, 12.0, input_w, input_h), &mut self.name, !built_in, false)
                } else {
                    winui::edit_light(ui, r(o, input_x, 12.0, input_w, input_h), &mut self.name, !built_in, false)
                };
                // Opened, the keys are in Name (a new category is named first).
                if !self.focused && !built_in {
                    name_box.request_focus();
                    self.focused = true;
                }
                label(if is_modern { 64.0 } else { 57.0 }, crate::i18n::mark("File types:"));
                let _types_box = if is_modern {
                    winui::edit(ui, r(o, input_x, 48.0, input_w, input_h), &mut self.types, !built_in, false)
                } else {
                    winui::edit_light(ui, r(o, input_x, 43.0, input_w, input_h), &mut self.types, !built_in, false)
                };
                winui::fit(
                    ui,
                    &p,
                    o.x + input_x,
                    o.y + if is_modern { 88.0 } else { 80.0 },
                    crate::i18n::t("Extensions separated by spaces, for example MP4 MKV R0*"),
                    LIGHT_DISABLED(),
                    o.x + input_x + input_w,
                    "category-types-hint",
                );
                label(if is_modern { 122.0 } else { 112.0 }, crate::i18n::mark("Folder:"));
                // Browse as wide as its label in this language (#192); the box ends 8 px before.
                let right_bound = if is_modern { input_x + input_w } else { 408.0 };
                let (browse_x, browse_w) =
                    winui::buttons_to_right(&p, right_bound, 8.0, &[("Browse...", 73.0)], 160.0)[0];
                let folder_y = if is_modern { 108.0 } else { 98.0 };
                if is_modern {
                    winui::edit(ui, r(o, input_x, folder_y, browse_x - 8.0 - input_x, input_h), &mut self.folder, true, false);
                } else {
                    winui::edit_light(ui, r(o, input_x, folder_y, browse_x - 8.0 - input_x, input_h), &mut self.folder, true, false);
                }
                let browse_clicked = if is_modern {
                    winui::button(ui, r(o, browse_x, folder_y, browse_w, input_h), "Browse...", ButtonKind::Normal, false, None).clicked()
                } else {
                    winui::button_light(ui, r(o, browse_x, folder_y + 1.0, browse_w, 21.0), "Browse...", ButtonKind::Normal, false).clicked()
                };
                if browse_clicked {
                    // i18n: title of the folder picker of the category dialog
                    let title = crate::i18n::t("Category folder").to_owned();
                    // From the folder typed, when there is one.
                    let typed = Path::new(self.folder.trim()).to_path_buf();
                    winui::start_pick(ui.ctx(), "category-folder", move || {
                        let mut pick = crate::winui::file_dialog(&title);
                        if typed.is_dir() {
                            pick = pick.set_directory(&typed);
                        }
                        pick.pick_folder()
                    });
                }
                if let Some(dir) = winui::picked(ui.ctx(), "category-folder") {
                    self.folder = dir.display().to_string();
                }
                let name = self.name.trim();
                // An empty name only greys OK; a name in use says so.
                // Compared in any script's case (Книги is книги), the translated names too.
                let same = |a: &str, b: &str| a.to_lowercase() == b.to_lowercase();
                let taken = !built_in
                    && names.iter().any(|n| {
                        (same(n, name) || same(crate::i18n::t(n), name))
                            && !same(n, &own)
                            && !same(crate::i18n::t(n), &own)
                    });
                if taken {
                    let e = crate::i18n::t("There is already a category with this name");
                    winui::fit(
                        ui,
                        &p,
                        o.x + input_x,
                        o.y + if is_modern { 152.0 } else { 142.0 },
                        e,
                        winui::ERROR(),
                        o.x + right_bound,
                        "category-taken",
                    );
                }
                let valid = built_in || (!name.is_empty() && !taken);
                let btn_right = if is_modern { right_bound } else { 408.0 };
                let [ok_at, cancel_at] = winui::buttons_to_right(
                    &p,
                    btn_right,
                    8.0,
                    &[("OK", 73.0), ("Cancel", 73.0)],
                    140.0,
                )[..] else {
                    unreachable!("two buttons")
                };
                let btn_y = if is_modern { 190.0 } else { 164.0 };
                let btn_h = if is_modern { 34.0 } else { 21.0 };
                let ok = if is_modern {
                    winui::button(ui, r(o, ok_at.0, btn_y, ok_at.1, btn_h), "OK", if valid { ButtonKind::Default } else { ButtonKind::Disabled }, false, None)
                } else {
                    winui::button_light(ui, r(o, ok_at.0, btn_y, ok_at.1, btn_h), "OK", if valid { ButtonKind::Default } else { ButtonKind::Disabled }, false)
                };
                let ok = winui::why_disabled(
                    ok,
                    name.is_empty() && !built_in,
                    "Type the category's name",
                )
                .clicked();
                let cancel = if is_modern {
                    winui::button(ui, r(o, cancel_at.0, btn_y, cancel_at.1, btn_h), "Cancel", ButtonKind::Normal, false, None).clicked()
                } else {
                    winui::button_light(ui, r(o, cancel_at.0, btn_y, cancel_at.1, btn_h), "Cancel", ButtonKind::Normal, false).clicked()
                };
                if cancel || winui::escape_pressed(ui) {
                    Outcome::Closed
                } else if valid && (ok || winui::enter_pressed(ui)) {
                    Outcome::Done(())
                } else {
                    Outcome::Open
                }
            },
        );
        match out {
            Outcome::Open => Outcome::Open,
            Outcome::Closed => Outcome::Closed,
            Outcome::Done(()) => Outcome::Done(CategoryEdit {
                target: self.target,
                name: self.name.trim().to_owned(),
                types: normalize_types(&self.types),
                folder: self.folder.trim().to_owned(),
            }),
        }
    }
}

/// File types as the matching reads them: one extension per word, without the "." or "*."
/// people type (".mp4", "*.mp4"), commas and semicolons as spaces ("mp4, mkv"); a pattern's
/// own "*" (R0*) stays.
pub fn normalize_types(typed: &str) -> String {
    typed
        .split(|c: char| c.is_whitespace() || c == ',' || c == ';')
        .map(|t| t.trim_start_matches("*.").trim_start_matches('.'))
        .filter(|t| !t.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod types_tests {
    #[test]
    fn typed_extensions_are_made_plain() {
        assert_eq!(
            super::normalize_types(".mp4, *.mkv;avi  R0*"),
            "mp4 mkv avi R0*"
        );
    }
}

/// Every category's name now, as shown (for the dialog's "already exists" check).
pub fn names() -> Vec<String> {
    Category::all().iter().map(|c| c.shown()).collect()
}

/// The edit `settings` hold for built-in `cat` (made when there is none).
fn builtin_entry(settings: &mut Settings, cat: Category) -> &mut BuiltinEdit {
    let at = match settings
        .builtin_categories
        .iter()
        .position(|e| e.category == cat)
    {
        Some(i) => i,
        None => {
            settings.builtin_categories.push(BuiltinEdit {
                category: cat,
                name: None,
                types: None,
                deleted: false,
            });
            settings.builtin_categories.len() - 1
        }
    };
    &mut settings.builtin_categories[at]
}

/// Edits that say nothing any more (the shipped name and types, not deleted) go.
fn tidy_builtin(settings: &mut Settings) {
    settings
        .builtin_categories
        .retain(|e| e.name.is_some() || e.types.is_some() || e.deleted);
}

/// The five built-in categories by file type, as Restore lists them.
fn by_type() -> impl Iterator<Item = Category> {
    Category::BUILT_IN.into_iter().filter(|c| c.editable())
}

/// Apply an edit to `settings` (the app's, or Options' draft) and to the registry; returns the
/// category. A folder equal to the default one is stored as "none" (it follows Downloads).
pub fn apply(settings: &mut Settings, e: CategoryEdit, downloads: &Path) -> Category {
    let cat = match e.target {
        Target::Edit(c) if !c.editable() => c,
        Target::Edit(Category::Custom(id)) => {
            if let Some(c) = settings.custom_categories.iter_mut().find(|c| c.id == id) {
                c.name = e.name.clone();
                c.types = e.types.clone();
            }
            Category::Custom(id)
        }
        // A built-in one by file type (#93): what equals the shipped one stays unsaid, so the
        // name keeps following the interface language.
        Target::Edit(c) => {
            let name = e.name.trim();
            let shipped_name = name == c.shipped_shown() || name == c.key().as_ref();
            let same = |a: &str, b: &str| a.eq_ignore_ascii_case(b);
            let shipped_types = same(
                &normalize_types(&e.types),
                &normalize_types(&c.shipped_types_text()),
            );
            let entry = builtin_entry(settings, c);
            entry.name = (!shipped_name && !name.is_empty()).then(|| name.to_owned());
            entry.types = (!shipped_types).then(|| normalize_types(&e.types));
            tidy_builtin(settings);
            settings.publish_categories();
            c
        }
        _ => {
            let id = settings
                .custom_categories
                .iter()
                .map(|c| c.id + 1)
                .max()
                .unwrap_or(1);
            settings.custom_categories.push(CustomCategory {
                id,
                name: e.name.clone(),
                types: e.types.clone(),
                folder: None,
            });
            Category::Custom(id)
        }
    };
    crate::model::set_custom(&settings.custom_categories);
    let typed = e.folder.trim_end_matches(['\\', '/']);
    let base = if cat == Category::General {
        std::borrow::Cow::Borrowed(downloads)
    } else {
        crate::model::default_downloads(downloads, &settings.category_dirs)
    };
    let default = cat.default_folder(&base).display().to_string();
    let own = (!typed.is_empty() && typed != default.trim_end_matches(['\\', '/']))
        .then(|| typed.to_owned());
    settings.set_category_dir(cat, own);
    crate::model::set_custom(&settings.custom_categories);
    cat
}

/// Delete a category: a user's goes, a built-in one by file type is marked deleted (#93); the
/// downloads still in it become General (`items`), files stay put. General and Torrents stay.
pub fn delete(settings: &mut Settings, cat: Category, items: &mut [crate::model::Item]) {
    match cat {
        Category::Custom(id) => settings.custom_categories.retain(|c| c.id != id),
        c if c.editable() => builtin_entry(settings, c).deleted = true,
        _ => return,
    }
    settings.publish_categories();
    for i in items {
        if i.category == cat {
            i.category = Category::General;
        }
    }
}

/// Options → Save to → Restore the built-in categories: the five by file type come back as
/// shipped (name, file types, folder); General, Torrents and the user's own stay as they are.
pub fn restore(settings: &mut Settings) {
    let keys: Vec<String> = by_type().map(|c| c.key().into_owned()).collect();
    settings
        .builtin_categories
        .retain(|e| !keys.iter().any(|k| *k == *e.category.key()));
    settings.category_dirs.retain(|(c, _)| !keys.contains(c));
    settings.publish_categories();
}

/// "Compressed, Documents, Music, Programs, Video" in the interface language, for Restore's
/// question.
pub fn restored_names() -> String {
    by_type()
        .map(|c| c.shipped_shown())
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    // Whole paths on the system the tests run on.
    const SONGS: &str = if cfg!(windows) {
        r"E:\Songs"
    } else {
        "/mnt/Songs"
    };
    const ALL: &str = if cfg!(windows) { r"F:\All" } else { "/mnt/All" };

    use super::*;

    /// The category registry is the process's: its tests take turns.
    static REGISTRY: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn registry() -> std::sync::MutexGuard<'static, ()> {
        REGISTRY.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn edit(cat: Category, name: &str, types: &str, folder: &str) -> CategoryEdit {
        CategoryEdit {
            target: Target::Edit(cat),
            name: name.into(),
            types: types.into(),
            folder: folder.into(),
        }
    }

    #[test]
    fn a_built_in_one_takes_a_name_types_and_folder() {
        let _turn = registry();
        let mut s = Settings::default();
        s.publish_categories();
        let downloads = Path::new(r"C:\Users\u\Downloads");
        apply(
            &mut s,
            edit(Category::Music, "Songs", "MP3 FLAC", SONGS),
            downloads,
        );
        assert_eq!(Category::Music.label(), "Songs");
        assert_eq!(Category::Music.shown(), "Songs", "kept as typed");
        assert_eq!(
            Category::Music.key(),
            "Music",
            "settings still know it as Music"
        );
        assert_eq!(Category::of("a.flac"), Category::Music);
        assert_eq!(
            Category::of("a.wav"),
            Category::General,
            "WAV was taken off"
        );
        assert_eq!(s.category_dir(Category::Music, downloads), Path::new(SONGS));
        assert_eq!(
            Category::Music.default_folder(downloads),
            downloads.join("Music"),
            "renaming moves no folder"
        );
        // Typing the shipped name and types back is as shipped: nothing kept.
        let shipped = Category::Music.shipped_types_text();
        apply(
            &mut s,
            edit(Category::Music, "Music", &shipped, ""),
            downloads,
        );
        assert!(
            s.builtin_categories.is_empty(),
            "{:?}",
            s.builtin_categories
        );
        assert_eq!(Category::of("a.wav"), Category::Music);
        s = Settings::default();
        s.publish_categories();
    }

    #[test]
    fn a_deleted_built_in_one_takes_no_files_and_restore_brings_it_back() {
        let _turn = registry();
        let mut s = Settings::default();
        s.publish_categories();
        let downloads = Path::new(r"C:\d");
        apply(
            &mut s,
            edit(Category::Video, "Films", "MKV", r"E:\Films"),
            downloads,
        );
        let mut items: Vec<crate::model::Item> = Vec::new();
        delete(&mut s, Category::Video, &mut items);
        assert!(Category::Video.deleted());
        assert!(!Category::all().contains(&Category::Video));
        assert_eq!(Category::of("film.mkv"), Category::General);
        assert_eq!(
            Category::of("film.mp4"),
            Category::General,
            "its shipped types go too"
        );
        // General and Torrents can't be deleted.
        delete(&mut s, Category::General, &mut items);
        delete(&mut s, Category::Torrents, &mut items);
        assert!(!Category::General.deleted() && !Category::Torrents.deleted());
        assert!(!Category::General.editable() && !Category::Torrents.editable());
        // A user's own and General's folder survive Restore.
        let books = apply(
            &mut s,
            CategoryEdit {
                target: Target::New,
                name: "Books".into(),
                types: "EPUB".into(),
                folder: String::new(),
            },
            downloads,
        );
        s.set_category_dir(Category::General, Some(ALL.into()));
        restore(&mut s);
        assert!(!Category::Video.deleted());
        assert_eq!(Category::Video.label(), "Video");
        assert_eq!(Category::of("film.mp4"), Category::Video);
        assert_eq!(
            s.category_dir(Category::Video, downloads),
            Path::new(ALL).join("Video")
        );
        assert_eq!(s.category_dir(Category::General, downloads), Path::new(ALL));
        assert!(Category::all().contains(&books));
        s = Settings::default();
        s.publish_categories();
    }

    #[test]
    fn old_settings_without_edits_still_load() {
        let s: Settings = serde_json::from_str(r#"{"category_dirs":[["Music","E:\\M"]]}"#).unwrap();
        assert!(s.builtin_categories.is_empty());
        let e: BuiltinEdit =
            serde_json::from_str(r#"{"category":"Music","deleted":true}"#).unwrap();
        assert!(e.deleted && e.name.is_none() && e.types.is_none());
    }

    #[test]
    fn a_new_category_takes_its_files_and_folder() {
        let _turn = registry();
        let mut s = Settings::default();
        let downloads = Path::new(r"C:\Users\u\Downloads");
        let e = CategoryEdit {
            target: Target::New,
            name: "Books".into(),
            types: "EPUB MOBI".into(),
            folder: String::new(),
        };
        let cat = apply(&mut s, e, downloads);
        assert!(cat.is_custom());
        assert_eq!(Category::of("novel.epub"), cat, "its file types come first");
        assert_eq!(Category::of("a.zip"), Category::Compressed);
        assert_eq!(cat.label(), "Books");
        assert_eq!(cat.folder(downloads), downloads.join("Books"));
        // A folder of its own.
        let e = CategoryEdit {
            target: Target::Edit(cat),
            name: "Books".into(),
            types: "EPUB".into(),
            folder: r"D:\Library".into(),
        };
        apply(&mut s, e, downloads);
        assert_eq!(cat.folder(downloads), Path::new(r"D:\Library"));
        assert_eq!(
            Category::of("x.mobi"),
            Category::General,
            "MOBI was taken off"
        );
        let mut items: Vec<crate::model::Item> = Vec::new();
        delete(&mut s, cat, &mut items);
        assert_eq!(Category::of("novel.epub"), Category::General);
        crate::model::set_custom(&[]);
    }

    #[test]
    fn torrents_have_a_folder_but_no_file_types() {
        let _turn = registry();
        let downloads = Path::new(r"C:\d");
        assert_eq!(
            Category::Torrents.folder(downloads),
            downloads.join("Torrents")
        );
        assert_eq!(Category::of("ubuntu.iso.torrent"), Category::General);
        assert!(Category::all().contains(&Category::Torrents));
    }
}
