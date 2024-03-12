use std::{env::var, path::Display};

use adw::{
    prelude::*, Application, ApplicationWindow, Bin, HeaderBar, NavigationPage, NavigationView,
    ToolbarView,
};
use gtk::{
    cairo::Path,
    ffi::{gtk_icon_theme_lookup_icon, gtk_widget_get_display},
    gdk::ffi::GdkDisplay,
    gio::{self, resources_lookup_data, resources_register_include, Icon},
    glib::{property::PropertyGet, ExitCode, GString},
    pango::{AttrList, AttrSize},
    Box, FlowBox, Grid, IconLookupFlags, IconPaintable, IconTheme, Image, Label, Orientation,
    ScrolledWindow, Widget,
};

#[derive(Clone)]
struct CardData {
    title: String,
    url: String,
    score: u32,
    kids: u32,
    by: String,
    time: String,
}

fn build_card(card_data: CardData) -> Widget {
    let top_label: Label = Label::builder()
        .wrap(true)
        .natural_wrap_mode(gtk::NaturalWrapMode::None)
        .build();

    top_label.set_markup(
        format!(
            "<span size=\"115%\">{}</span> <span foreground=\"grey\">({})</span>",
            card_data.title.clone(),
            &card_data.url.clone()
        )
        .as_str(),
    );
    let top_label_widget: Widget = <gtk::Label as Into<Widget>>::into(top_label);

    let bottom_label: Label = Label::builder()
        .wrap(true)
        .natural_wrap_mode(gtk::NaturalWrapMode::None)
        .build();

    let gtk_icons: IconTheme = IconTheme::builder().build();
    let chat_icon: IconPaintable = gtk_icons.lookup_icon(
        "chat-bubbles-text-symbolic",
        &[
            &"chat-bubble-emtpy-symbolic".to_string(),
            &"comment-symbolic".to_string(),
        ],
        14,
        1,
        gtk::TextDirection::Ltr,
        IconLookupFlags::all(),
    );

    let icon_name: String = chat_icon
        .icon_name()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string();

    let chat_icon_image: Widget = Image::builder().icon_name(icon_name).build().into();

    let bottom_label_widget: Widget = <gtk::Label as Into<Widget>>::into(bottom_label);

    let card_box: Box = Box::builder().orientation(Orientation::Horizontal).build();
    card_box.append(&top_label_widget);
    card_box.append(&chat_icon_image);
    card_box.into()

    // let flowbox: FlowBox = FlowBox::builder()
    //     .orientation(Orientation::Horizontal)
    //     .column_spacing(20)
    //     .homogeneous(true)
    //     .overflow(gtk::Overflow::Hidden)
    //     .build();

    // flowbox.insert(&title_label, 1);
    // flowbox.insert(&url_label, 2);
    // flowbox.into()

    // let grid: Grid = Grid::builder().build();
    // grid.attach(&title_label, 0, 0, 100, 100);
    // grid.attach_next_to(
    //     &url_label,
    //     grid.child_at(0, 0).as_ref(),
    //     gtk::PositionType::Right,
    //     100,
    //     100,
    // );
    // grid.into()
}

fn main() {
    gio::resources_register_include!("compiled.gresource").expect("Failed to register resources.");

    let card: CardData = CardData {
        title: String::from("A peculiarity of the X Window System: Windows all the way down"),
        url: String::from("utcc.utoronto.ca"),
        score: 43,
        kids: 17,
        by: String::from("ingve"),
        time: String::from("yesterday"),
    };

    let application = Application::builder()
        .application_id("com.example.FirstAdwaitaApp")
        .build();

    application.connect_activate(move |app| {
        let header_bar: HeaderBar = HeaderBar::builder()
            .decoration_layout(String::new())
            .halign(gtk::Align::Start)
            .build();

        let container: Box = Box::builder()
            .homogeneous(true)
            .orientation(Orientation::Vertical)
            .build();

        let news_feed: ScrolledWindow = ScrolledWindow::builder()
            .margin_bottom(0)
            .margin_top(0)
            .has_frame(true)
            .propagate_natural_height(true)
            .max_content_height(300)
            .child(&container)
            .build();

        (0..20).for_each(|_i: i32| {
            let item: Bin = Bin::builder()
                .margin_top(6)
                .margin_bottom(6)
                .margin_start(6)
                .margin_end(6)
                .child(&build_card(card.clone()))
                .build();
            container.append(&item);
        });

        let toolbar_view: ToolbarView = ToolbarView::builder().build();
        toolbar_view.add_top_bar(&header_bar);
        toolbar_view.set_content(Some(&news_feed));

        let story_page: NavigationPage = NavigationPage::builder()
            .title("Top Stories".to_string())
            .child(&toolbar_view)
            .build();

        let nav_view: NavigationView = NavigationView::builder().build();
        nav_view.add(&story_page);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("First App")
            // add content to window
            .content(&nav_view)
            .build();
        window.present();
    });

    application.run();
}
