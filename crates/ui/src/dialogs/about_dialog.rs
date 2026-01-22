//! About dialog for the application
//!
//! Displays application information, credits, and license details.

use gtk4::prelude::*;
use gtk4::{AboutDialog, Window};

/// Application version (should match Cargo.toml)
pub const APP_VERSION: &str = "0.1.0";

/// Application name
pub const APP_NAME: &str = "Testruct Studio";

/// Show the About dialog
pub fn show_about_dialog(parent: &Window) {
    let dialog = AboutDialog::new();

    dialog.set_transient_for(Some(parent));
    dialog.set_modal(true);

    // Application identity
    dialog.set_program_name(Some(APP_NAME));
    dialog.set_version(Some(APP_VERSION));

    // Description
    dialog.set_comments(Some(
        "プロフェッショナルなドキュメントとテスト問題を作成するための\n\
         強力なデザイン・テスト作成ツール\n\n\
         A powerful design and test creation tool for creating\n\
         professional documents and test items."
    ));

    // Copyright and license
    dialog.set_copyright(Some("© 2025 Testruct Team. All rights reserved."));
    dialog.set_license_type(gtk4::License::MitX11);

    // Custom license text for more details
    dialog.set_license(Some(
        "MIT License\n\n\
         Permission is hereby granted, free of charge, to any person obtaining a copy\n\
         of this software and associated documentation files (the \"Software\"), to deal\n\
         in the Software without restriction, including without limitation the rights\n\
         to use, copy, modify, merge, publish, distribute, sublicense, and/or sell\n\
         copies of the Software, and to permit persons to whom the Software is\n\
         furnished to do so, subject to the following conditions:\n\n\
         The above copyright notice and this permission notice shall be included in all\n\
         copies or substantial portions of the Software.\n\n\
         THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\n\
         IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\n\
         FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT."
    ));
    dialog.set_wrap_license(true);

    // Website and repository
    dialog.set_website(Some("https://github.com/testruct/testruct-studio"));
    dialog.set_website_label("GitHub Repository");

    // Credits - Authors
    let authors = [
        "Testruct Team <team@testruct.dev>",
        "",
        "Contributors:",
        "  Core Development Team",
    ];
    dialog.set_authors(&authors);

    // Credits - Documenters
    let documenters = [
        "Testruct Documentation Team",
    ];
    dialog.set_documenters(&documenters);

    // Credits - Artists (UI/UX)
    let artists = [
        "Testruct Design Team",
    ];
    dialog.set_artists(&artists);

    // Translator credits (for i18n)
    dialog.set_translator_credits(Some(
        "Japanese: Testruct Team\n\
         English: Testruct Team"
    ));

    // System information in comments
    let system_info = format!(
        "Build Information:\n\
         • Rust Edition: 2021\n\
         • GTK Version: 4.x\n\
         • Platform: {}\n\
         • Architecture: {}",
        std::env::consts::OS,
        std::env::consts::ARCH
    );
    dialog.add_credit_section("System", &[&system_info]);

    // Third-party acknowledgments
    dialog.add_credit_section(
        "Powered By",
        &[
            "GTK4 - GNOME UI Toolkit",
            "Cairo - 2D Graphics Library",
            "Pango - Text Rendering",
            "Rust Programming Language",
        ],
    );

    // Show the dialog
    dialog.present();
}

/// Get the application version string
pub fn get_version() -> &'static str {
    APP_VERSION
}

/// Get the application name
pub fn get_app_name() -> &'static str {
    APP_NAME
}
