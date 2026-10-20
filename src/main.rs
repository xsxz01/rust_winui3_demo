#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use windows_reactor::*;

const THEMES: [&str; 3] = ["Mica", "云母 Alt", "亚克力"];

/// Segoe MDL2 Assets 字形码点。
/// https://learn.microsoft.com/windows/apps/design/style/segoe-ui-symbol-font
mod mdl2 {
    pub const HOME: &str = "\u{E80F}";
    pub const CALCULATOR: &str = "\u{E1D0}";
    pub const REPAIR: &str = "\u{E90F}";
}

fn mdl2_icon(glyph: &str) -> FontIcon {
    FontIcon::new().glyph(glyph)
}

fn main() {
    App::run_component::<DemoApp>(()).unwrap();
}

struct DemoApp {
    history: Vec<Page>,
    page: Page,
    pane_open: bool,
    search: String,
    count: i32,
    theme: WindowTheme,
    dark_mode: bool,
    volume: f64,
    material: Option<usize>,
    accepted: bool,
    info_open: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Home,
    Counter,
    Controls,
}

impl Page {
    fn from_tag(tag: &str) -> Option<Self> {
        match tag {
            "home" => Some(Self::Home),
            "counter" => Some(Self::Counter),
            "controls" => Some(Self::Controls),
            _ => None,
        }
    }

    fn tag(self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::Counter => "counter",
            Self::Controls => "controls",
        }
    }

    fn title(self) -> &'static str {
        match self {
            Self::Home => "首页",
            Self::Counter => "计数器",
            Self::Controls => "控件",
        }
    }
}

#[derive(Clone)]
enum Message {
    Back,
    Navigate(Option<String>),
    PaneOpen(bool),
    SearchChanged(String),
    TogglePane,
    CycleTheme,
    Increment,
    Decrement,
    Reset,
    DarkMode(bool),
    Volume(f64),
    Material(Option<usize>),
    Accepted(bool),
    DismissInfo,
}

impl Component for DemoApp {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            history: Vec::new(),
            page: Page::Home,
            pane_open: true,
            search: String::new(),
            count: 0,
            theme: WindowTheme::System,
            dark_mode: false,
            volume: 48.0,
            material: Some(0),
            accepted: false,
            info_open: true,
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Back => {
                if let Some(page) = self.history.pop() {
                    self.page = page;
                }
            }
            Message::Navigate(Some(tag)) => {
                if let Some(page) = Page::from_tag(&tag) {
                    self.navigate(page);
                }
            }
            Message::Navigate(None) => {}
            Message::PaneOpen(open) => self.pane_open = open,
            Message::SearchChanged(value) => self.search = value,
            Message::TogglePane => self.pane_open = !self.pane_open,
            Message::CycleTheme => {
                self.theme = match self.theme {
                    WindowTheme::System => WindowTheme::Light,
                    WindowTheme::Light => WindowTheme::Dark,
                    WindowTheme::Dark => WindowTheme::System,
                };
            }
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
            Message::Reset => self.count = 0,
            Message::DarkMode(value) => self.dark_mode = value,
            Message::Volume(value) => self.volume = value,
            Message::Material(index) => self.material = index,
            Message::Accepted(value) => self.accepted = value,
            Message::DismissInfo => self.info_open = false,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title(format!("Rust WinUI 3 Demo - {}", self.page.title()));
        context.window_visuals(
            WindowVisuals::new()
                .theme(self.theme)
                .backdrop(WindowBackdrop::Mica)
                .client_size(1400.0, 900.0),
        );

        let title_bar = TitleBar::new()
            .preferred_height(WindowTitleBarHeight::Tall)
            .title("Rust WinUI 3 Demo")
            .subtitle(self.page.title())
            .is_back_button_visible(true)
            .is_back_button_enabled(!self.history.is_empty())
            .is_pane_toggle_button_visible(true)
            .on_back_requested(context.message(Message::Back))
            .on_pane_toggle_requested(context.message(Message::TogglePane))
            .grid_row(0)
            .slots([
                SlotView::new(
                    TitleBarSlot::Content,
                    TextBox::new()
                        .text(self.search.clone())
                        .placeholder_text("Search controls and samples...")
                        .on_text_changed(context.callback(Message::SearchChanged)),
                ),
                SlotView::new(
                    TitleBarSlot::RightHeader,
                    Button::new()
                        .on_click(context.message(Message::CycleTheme))
                        .content(format!("Theme: {}", theme_name(self.theme))),
                ),
            ]);

        let navigation = NavigationView::new()
            .pane_display_mode(NavigationViewPaneDisplayMode::Left)
            .is_pane_toggle_button_visible(false)
            .is_back_button_visible(NavigationViewBackButtonVisible::Collapsed)
            .is_settings_visible(false)
            .always_show_header(false)
            .pane_title("WinUI 3")
            .is_pane_open(self.pane_open)
            .on_is_pane_open_changed(context.callback(Message::PaneOpen))
            .on_selected_tag_changed(context.callback(Message::Navigate))
            .grid_row(1)
            .slots([
                SlotView::collection(
                    NavigationViewSlot::MenuItems,
                    [
                        nav_item("home", "首页", mdl2::HOME, self.page == Page::Home),
                        nav_item(
                            "counter",
                            "计数器",
                            mdl2::CALCULATOR,
                            self.page == Page::Counter,
                        ),
                        nav_item(
                            "controls",
                            "控件",
                            mdl2::REPAIR,
                            self.page == Page::Controls,
                        ),
                    ],
                ),
                SlotView::new(NavigationViewSlot::Content, self.page_content(context)),
            ]);

        Grid::new()
            .rows([GridLength::Auto, GridLength::Star(1.0)])
            .children((title_bar, navigation))
    }
}

fn theme_name(theme: WindowTheme) -> &'static str {
    match theme {
        WindowTheme::System => "System",
        WindowTheme::Light => "Light",
        WindowTheme::Dark => "Dark",
    }
}

impl DemoApp {
    fn navigate(&mut self, page: Page) {
        if page != self.page {
            self.history
                .push(std::mem::replace(&mut self.page, page));
        }
    }

    fn page_content(&self, context: &mut ViewContext<Self>) -> View {
        match self.page {
            Page::Home => home_page(self, context),
            Page::Counter => counter_page(self, context),
            Page::Controls => controls_page(self, context),
        }
    }
}

fn nav_item(tag: &str, label: &str, glyph: &str, selected: bool) -> KeyedView {
    KeyedView::new(
        tag,
        NavigationViewItem::new()
            .tag(tag)
            .is_selected(selected)
            .selects_on_invoked(true)
            .slots([
                SlotView::new(NavigationViewItemSlot::Content, label),
                SlotView::new(NavigationViewItemSlot::Icon, mdl2_icon(glyph)),
            ]),
    )
}

fn page_shell(children: impl Into<View>) -> View {
    Border::new()
        .padding(24.0)
        .content(ScrollViewer::new().content(children))
}

fn heading(title: &str, subtitle: &str) -> View {
    StackPanel::new().spacing(6.0).children((
        TextBlock::new().text(title).font_size(28.0),
        TextBlock::new()
            .text(subtitle)
            .font_size(14.0)
            .opacity(0.72)
            .text_wrapping(TextWrapping::Wrap),
    ))
}

fn home_page(app: &DemoApp, context: &mut ViewContext<DemoApp>) -> View {
    page_shell(
        StackPanel::new()
            .spacing(16.0)
            .max_width(720.0)
            .children((
                heading(
                    "Rust + WinUI 3",
                    "使用 windows-rs 的 windows-reactor 封装，在原生 WinUI 3 上构建桌面界面。",
                ),
                InfoBar::new()
                    .title("运行环境")
                    .message("当前应用以 framework-dependent 方式启动，会加载本机已安装的 Windows App Runtime。")
                    .severity(InfoBarSeverity::Success)
                    .is_open(app.info_open)
                    .on_closed(context.message(Message::DismissInfo)),
                TextBlock::new()
                    .text("这个 demo 覆盖了导航、状态更新和一组常用 WinUI 控件。从左侧选择页面开始体验。")
                    .text_wrapping(TextWrapping::Wrap),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .children((
                        Button::new()
                            .on_click(context.message(Message::Navigate(Some(
                                Page::Counter.tag().into(),
                            ))))
                            .content("打开计数器"),
                        Button::new()
                            .on_click(context.message(Message::Navigate(Some(
                                Page::Controls.tag().into(),
                            ))))
                            .content("浏览控件"),
                    )),
            )),
    )
}

fn counter_page(app: &DemoApp, context: &mut ViewContext<DemoApp>) -> View {
    let progress = app.count.clamp(0, 20) as f64 * 5.0;

    page_shell(
        StackPanel::new()
            .spacing(16.0)
            .max_width(520.0)
            .children((
                heading("计数器", "点击按钮会发送组件消息并触发重新渲染。"),
                TextBlock::new()
                    .text(app.count.to_string())
                    .font_size(56.0),
                ProgressBar::new()
                    .minimum(0.0)
                    .maximum(100.0)
                    .value(progress),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .children((
                        Button::new()
                            .on_click(context.message(Message::Decrement))
                            .content("− 减少"),
                        Button::new()
                            .on_click(context.message(Message::Increment))
                            .content("+ 增加"),
                        Button::new()
                            .on_click(context.message(Message::Reset))
                            .content("重置"),
                    )),
                TextBlock::new()
                    .text("进度条按 0–20 的计数范围映射；超出范围后计数仍会继续变化。")
                    .opacity(0.72)
                    .text_wrapping(TextWrapping::Wrap),
            )),
    )
}

fn controls_page(app: &DemoApp, context: &mut ViewContext<DemoApp>) -> View {
    let theme_label = app
        .material
        .and_then(|index| THEMES.get(index).copied())
        .unwrap_or("未选择");

    page_shell(
        StackPanel::new()
            .spacing(16.0)
            .max_width(520.0)
            .children((
                heading("常用控件", "ToggleSwitch、Slider、ComboBox 和 CheckBox 都通过受控属性回写状态。"),
                ToggleSwitch::new()
                    .is_on(app.dark_mode)
                    .on_toggled(context.callback(Message::DarkMode))
                    .slots([
                        SlotView::new(ToggleSwitchSlot::Header, "深色模式"),
                        SlotView::new(ToggleSwitchSlot::OnContent, "开"),
                        SlotView::new(ToggleSwitchSlot::OffContent, "关"),
                    ]),
                TextBlock::new().text(format!("音量：{:.0}%", app.volume)),
                Slider::new()
                    .minimum(0.0)
                    .maximum(100.0)
                    .step_frequency(1.0)
                    .value(app.volume)
                    .on_value_changed(context.callback(Message::Volume)),
                ComboBox::new()
                    .items_source(THEMES)
                    .placeholder_text("选择窗口材质")
                    .selected_index(app.material)
                    .on_selection_changed(context.callback(Message::Material))
                    .slot(ComboBoxSlot::Header, "主题材质"),
                CheckBox::new()
                    .is_checked(app.accepted)
                    .on_is_checked_changed(context.callback(Message::Accepted))
                    .content("我已阅读 WinUI 3 / windows-reactor 示例说明"),
                TextBlock::new()
                    .text(format!(
                        "深色模式：{}　材质：{}　条款：{}",
                        if app.dark_mode { "开" } else { "关" },
                        theme_label,
                        if app.accepted { "已同意" } else { "未同意" },
                    ))
                    .opacity(0.72)
                    .text_wrapping(TextWrapping::Wrap),
            )),
    )
}
