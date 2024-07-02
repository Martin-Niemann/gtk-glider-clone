use adw::{HeaderBar, NavigationPage, ToolbarView};

pub struct Details {
    pub details_page: NavigationPage
}

impl Details {
    pub fn new() -> Details {
        let header_bar_normal: HeaderBar = HeaderBar::builder().decoration_layout("").show_back_button(true).build();

        let item_page_toolbar_view: ToolbarView = ToolbarView::builder().build();
        item_page_toolbar_view.add_top_bar(&header_bar_normal);
        item_page_toolbar_view.set_top_bar_style(adw::ToolbarStyle::Flat);

        let details_page: NavigationPage = NavigationPage::builder()
            .child(&item_page_toolbar_view)
            .build();
        let details: Details = Details {details_page};
        details
    }
}