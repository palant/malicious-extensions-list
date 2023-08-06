#![deny(elided_lifetimes_in_paths)]
#![deny(explicit_outlives_requirements)]
#![deny(keyword_idents)]
#![deny(meta_variable_misuse)]
#![deny(missing_debug_implementations)]
#![deny(non_ascii_idents)]
#![warn(noop_method_call)]
#![deny(pointer_structural_match)]
#![deny(single_use_lifetimes)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![warn(unused_crate_dependencies)]
#![deny(unused_import_braces)]
#![deny(unused_lifetimes)]
#![warn(unused_macro_rules)]
#![warn(unused_tuple_struct_fields)]
#![deny(variant_size_differences)]

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use enum_iterator::Sequence;

#[derive(Debug, Sequence)]
enum Browser {
    Chrome,
    ChromeBeta,
    ChromeCanary,
    Chromium,
    Opera,
    OperaBeta,
    OperaDev,
    Vivaldi,
    VivaldiSnapshot,
    Edge,
    EdgeBeta,
    EdgeDev,
    EdgeCanary,
    Brave,
    BraveBeta,
    BraveDev,
    BraveNightly
}

impl Browser {
    #[cfg(target_os = "linux")]
    pub fn get_profile_root(self) -> Option<PathBuf> {
        let mut dir = app_dirs2::get_data_root(app_dirs2::AppDataType::UserConfig).ok()?;
        match self {
            // https://chromium.googlesource.com/chromium/src/+/master/docs/user_data_dir.md#Default-Location
            Self::Chrome => dir.push("google-chrome"),
            Self::ChromeBeta => dir.push("google-chrome-beta"),
            Self::ChromeCanary => dir.push("google-chrome-unstable"),
            Self::Chromium => dir.push("chromium"),
            Self::Opera => dir.push("opera"),
            Self::OperaBeta => dir.push("opera-beta"),
            Self::OperaDev => dir.push("opera-developer"),
            // https://help.vivaldi.com/desktop/privacy/preventing-vivaldi-profiles-from-being-uploaded-to-git-repositories/
            Self::Vivaldi => dir.push("vivaldi"),
            Self::VivaldiSnapshot => dir.push("vivaldi-snapshot"),
            Self::Edge => dir.push("microsoft-edge"),
            Self::EdgeBeta => dir.push("microsoft-edge-beta"),
            Self::EdgeDev => dir.push("microsoft-edge-dev"),
            Self::EdgeCanary => dir.push("microsoft-edge-unstable"),
            // https://github.com/brave/brave-core/blob/master/build/commands/lib/start.js#L68
            // https://github.com/brave/brave-core/blob/master/common/brave_channel_info_unittest.cc#L51
            Self::Brave => {
                dir.push("BraveSoftware");
                dir.push("Brave-Browser");
            }
            Self::BraveBeta => {
                dir.push("BraveSoftware");
                dir.push("Brave-Browser-Beta");
            }
            Self::BraveDev => {
                dir.push("BraveSoftware");
                dir.push("Brave-Browser-Dev");
            }
            Self::BraveNightly => {
                dir.push("BraveSoftware");
                dir.push("Brave-Browser-Nightly");
            }
        }
        Some(dir)
    }

    #[cfg(target_os = "windows")]
    pub fn get_profile_root(self) -> Option<PathBuf> {
        let mut dir = app_dirs2::get_data_root(app_dirs2::AppDataType::UserData).ok()?;
        match self {
            Self::Chrome => {
                dir.push("Google");
                dir.push("Chrome");
            }
            Self::ChromeBeta => {
                dir.push("Google");
                dir.push("Chrome Beta");
            }
            Self::ChromeCanary => {
                dir.push("Google");
                dir.push("Chrome SxS");
            }
            Self::Chromium => dir.push("Chromium"),
            Self::Opera => dir.push("Opera"),
            Self::OperaBeta => dir.push("Opera Beta"),
            Self::OperaDev => dir.push("Opera Developer"),
            Self::Vivaldi => dir.push("Vivaldi"),
            Self::VivaldiSnapshot => dir.push("Vivaldi Snapshot"),
            Self::Edge => {
                dir.push("Microsoft");
                dir.push("Edge");
            }
            Self::EdgeBeta => {
                dir.push("Microsoft");
                dir.push("Edge Beta");
            }
            Self::EdgeDev => {
                dir.push("Microsoft");
                dir.push("Edge Dev");
            }
            Self::EdgeCanary => {
                dir.push("Microsoft");
                dir.push("Edge SxS");
            }
            Self::Brave => {
                dir.push("BraveSoftware");
                dir.push("Brave-Browser");
            }
            Self::BraveBeta => {
                dir.push("BraveSoftware");
                dir.push("Brave-Browser-Beta");
            }
            Self::BraveDev => {
                dir.push("BraveSoftware");
                dir.push("Brave-Browser-Dev");
            }
            Self::BraveNightly => {
                dir.push("BraveSoftware");
                dir.push("Brave-Browser-Nightly");
            }
        }
        dir.push("User Data");
        Some(dir)
    }

    #[cfg(target_os = "macos")]
    pub fn get_profile_root(self) -> Option<PathBuf> {
        let mut dir = app_dirs2::get_data_root(app_dirs2::AppDataType::UserData).ok()?;
        match self {
            Self::Chrome => {
                dir.push("Google");
                dir.push("Chrome");
            }
            Self::ChromeBeta => {
                dir.push("Google");
                dir.push("Chrome Beta");
            }
            Self::ChromeCanary => {
                dir.push("Google");
                dir.push("Chrome Canary");
            }
            Self::Chromium => dir.push("Chromium"),
            Self::Opera => dir.push("Opera"),
            Self::OperaBeta => dir.push("Opera Beta"),
            Self::OperaDev => dir.push("Opera Developer"),
            Self::Vivaldi => dir.push("Vivaldi"),
            Self::VivaldiSnapshot => dir.push("Vivaldi Snapshot"),
            Self::Edge => dir.push("Microsoft Edge"),
            Self::EdgeBeta => dir.push("Microsoft Edge Beta"),
            Self::EdgeDev => dir.push("Microsoft Edge Dev"),
            Self::EdgeCanary => dir.push("Microsoft Edge Canary"),
            Self::Brave => {
                dir.push("BraveSoftware");
                dir.push("Brave-Browser");
            }
            Self::BraveBeta => {
                dir.push("BraveSoftware");
                dir.push("Brave-Browser-Beta");
            }
            Self::BraveDev => {
                dir.push("BraveSoftware");
                dir.push("Brave-Browser-Dev");
            }
            Self::BraveNightly => {
                dir.push("BraveSoftware");
                dir.push("Brave-Browser-Nightly");
            }
        }
        Some(dir)
    }
}

fn get_ids() -> HashMap<String, String> {
    const DATA: &str = include_str!("../list.txt");
    let mut current_url = String::new();
    let mut result = HashMap::new();
    for line in DATA.split(['\r', '\n']) {
        if line.is_empty() {
            continue;
        }

        if let Some(url) = line.strip_prefix('#') {
            current_url = url.trim().to_string();
        } else {
            result.insert(line.trim().to_string(), current_url.clone());
        }
    }
    result
}

fn is_profile(path: &Path) -> bool {
    if path.is_dir() {
        let mut preferences = path.to_path_buf();
        preferences.push("Preferences");
        preferences.is_file()
    } else {
        false
    }
}

fn get_profiles() -> Vec<PathBuf> {
    let mut result = Vec::new();
    for root in enum_iterator::all().filter_map(Browser::get_profile_root) {
        let entries = match root.read_dir() {
            Ok(entries) => entries,
            Err(_) => continue,
        };

        result.extend(
            entries
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .filter(|path| is_profile(path)),
        );
        if is_profile(&root) {
            result.push(root);
        }
    }
    result
}

fn main() {
    let args = std::env::args_os().collect::<Vec<_>>();

    let profiles = if args.len() > 1 {
        let mut paths = Vec::new();
        for arg in &args[1..] {
            let path = PathBuf::from(arg);
            if is_profile(&path) {
                paths.push(path);
            } else {
                eprintln!("{} does not seem to be a browser profile, is it the directory containing Preferences file?", arg.to_string_lossy());
            }
        }

        if paths.is_empty() {
            eprintln!("No valid browser profiles given.");
            return;
        }
        paths
    } else {
        get_profiles()
    };

    if profiles.is_empty() {
        eprintln!("No browser profiles found, try passing a directory on command line explicitly.");
        return;
    }

    let ids = get_ids();
    for path in profiles {
        eprintln!("Checking profile {}", path.as_os_str().to_string_lossy());

        let mut extensions = path.clone();
        extensions.push("Extensions");

        let mut seen_extension = false;
        let mut seen_malicious = false;
        if let Ok(entries) = extensions.read_dir() {
            for entry in entries.filter_map(|entry| entry.ok()) {
                seen_extension = true;
                let name = entry.file_name();
                let id = name.to_string_lossy();
                if let Some(url) = ids.get(id.as_ref()) {
                    eprintln!("Extension {id} is potentially malicious, see {url}");
                    seen_malicious = true;
                }
            }
        }

        if !seen_extension {
            eprintln!("No installed extensions found.");
        } else if !seen_malicious {
            eprintln!("No known extensions found.");
        }

        eprintln!();
    }
}
