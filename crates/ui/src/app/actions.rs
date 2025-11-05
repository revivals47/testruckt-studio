use gtk4::{gio, glib, prelude::*};

pub fn register_global_actions(app: &gtk4::Application) {
    add_action(app, "quit", |application| {
        application.quit();
    });

    let actions: [(&str, Option<&str>); 7] = [
        ("new", Some("<Primary>n")),
        ("open", Some("<Primary>o")),
        ("save", Some("<Primary>s")),
        ("undo", Some("<Primary>z")),
        ("redo", Some("<Primary><Shift>z")),
        ("toggle-guides", Some("F7")),
        ("toggle-grid", Some("F8")),
    ];

    for (name, accel) in actions {
        let name_owned = name.to_string();
        add_action(app, name, move |_| tracing::info!(action = %name_owned, "triggered"));
        if let Some(accel) = accel {
            app.set_accels_for_action(&format!("app.{name}"), &[accel]);
        }
    }

    glib::set_application_name("Testruct Studio");
}

fn add_action<F>(app: &gtk4::Application, name: &str, callback: F)
where
    F: Fn(&gtk4::Application) + 'static,
{
    let action = gio::SimpleAction::new(name, None);
    let application_ref = app.clone();
    action.connect_activate(move |_, _| callback(&application_ref));
    app.add_action(&action);
}
