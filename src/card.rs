use gtk::{Box, Image, Label, Orientation, Widget};
use adw::prelude::*;

use crate::transform::CardData;

pub fn build_card(card_data: CardData) -> Widget {
    let top_widget = build_card_top_wiget(card_data.clone());
    let bottom_widget = build_card_bottom_widget(card_data.clone());
    top_widget.set_margin_bottom(10);

    let card_box: Box = Box::builder().orientation(Orientation::Vertical).build();
    card_box.append(&top_widget);
    card_box.append(&bottom_widget);
    card_box.into()
}

fn build_card_top_wiget(card_data: CardData) -> Widget {
    let top_label: Label = Label::builder()
        .xalign(0.0)
        .wrap(true)
        .natural_wrap_mode(gtk::NaturalWrapMode::None)
        .wrap_mode(gtk::pango::WrapMode::Word)
        .lines(2)
        .ellipsize(gtk::pango::EllipsizeMode::End)
        .build();

    top_label.set_markup(
        format!(
            "<span size=\"115%\">{}</span> <span foreground=\"grey\">({})</span>",
            card_data.title.clone(),
            &card_data.url.clone()
        )
        .as_str(),
    );
    let top_label_widget: Widget = top_label.into();

    let top_box: Box = Box::builder().orientation(Orientation::Horizontal).build();
    top_box.append(&top_label_widget);

    top_box.into()
}

fn build_card_bottom_widget(card_data: CardData) -> Widget {
    let arrow_icon_image: Image = Image::from_resource(
        "/org/gtk/gtk-glider-clone/icons/scalable/actions/arrow2-up-symbolic.svg",
    );
    arrow_icon_image.set_pixel_size(12);
    let arrow_icon_widget: Widget = arrow_icon_image.into();
    arrow_icon_widget.set_margin_end(4);

    let score_text: Label = Label::new(Some(&card_data.score.to_string()));
    score_text.set_size_request(12, 12);
    let score_text_widget: Widget = score_text.into();

    let chat_icon_image: Image = Image::from_resource(
        "/org/gtk/gtk-glider-clone/icons/scalable/actions/chat-bubble-emtpy-symbolic.svg",
    );
    chat_icon_image.set_pixel_size(12);
    let chat_icon_widget: Widget = chat_icon_image.into();
    chat_icon_widget.set_margin_start(10);
    chat_icon_widget.set_margin_end(4);

    let comments_text: Label = Label::new(Some(&card_data.descendants.to_string()));
    comments_text.set_size_request(12, 12);
    let comments_text_widget: Widget = comments_text.into();

    let by_text: Label = Label::new(Some(&card_data.by));
    by_text.set_size_request(12, 12);
    let by_text_widget: Widget = by_text.into();
    by_text_widget.set_margin_start(10);

    let time_text: Label = Label::builder().build();
    time_text.set_markup(
        format!(
            "<span foreground=\"grey\">{}</span>",
            card_data.time.clone(),
        )
        .as_str(),
    );
    time_text.set_size_request(12, 12);
    let time_text_widget: Widget = time_text.into();
    time_text_widget.set_halign(gtk::Align::End);
    time_text_widget.set_hexpand(true);

    let bottom_box: Box = Box::builder().orientation(Orientation::Horizontal).build();
    bottom_box.append(&arrow_icon_widget);
    bottom_box.append(&score_text_widget);
    bottom_box.append(&chat_icon_widget);
    bottom_box.append(&comments_text_widget);
    bottom_box.append(&by_text_widget);
    bottom_box.append(&time_text_widget);
    bottom_box.into()
}