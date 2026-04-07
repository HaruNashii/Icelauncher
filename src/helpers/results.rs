// ============ IMPORTS ============
use iced_layershell::reexport::core::Border;
use iced::{Alignment, Color, Element, Length, Padding, widget::{text::Wrapping, Id, Space, button, column, container, mouse_area, row, scrollable, scrollable::{Rail, Scroller}}};




// ============ CRATES ============
use crate::helpers::color::color_or_gradient;
use crate::
{
    AppData, 
    AppEntry, 
    Message, 
    ron::{FooterPosition, LabelPosition},
    helpers::
    {
        footer::build_footer,
        icon::derive_icon_char,
        scroll::row_height,
        style::entry_button_style,
        text::apply_entry_text_rules,
        widget::{corner_radius, horizontal_align, make_font_family},
    }, 
};




// ============ ENUM/STRUCT, ETC ============
struct EntryBadges<'a>
{
    frecency_badge: Option<Element<'a, Message>>,
    shortcut_badge: Option<Element<'a, Message>>,
}




// ============ FUNCTIONS ============
pub fn build_results_block(app: &AppData) -> Element<'_, Message>
{
    let footer  = build_footer(app);
    let results = build_results(app);

    match app.config.footer.position 
    {
        FooterPosition::Top => 
        {
            column![footer, results].spacing(app.config.window.section_spacing).into()
        }
        FooterPosition::Bottom => 
        {
            column![results, footer].spacing(app.config.window.section_spacing).into()
        }
        FooterPosition::Left => 
        {
            row![footer, results].spacing(app.config.window.section_spacing).into()
        }
        FooterPosition::Right => 
        {
            row![results, footer].spacing(app.config.window.section_spacing).into()
        }
    }
}



fn build_results(app: &AppData) -> Element<'_, Message>
{
    if app.loading
    {
        let label = if app.shell_mode { "Loading commands..." } else { "Loading applications..." };
        return status_label(app, label);
    }
    if app.filtered.is_empty()
    {
        return status_label(app, "No results found");
    }

    let entry_buttons = build_entry_buttons(app);
    let grid          = arrange_into_grid(app, entry_buttons);
    build_scrollable(app, grid)
}



fn status_label<'a>(app: &'a AppData, label: &'a str) -> Element<'a, Message>
{
    let color = app.config.search.placeholder_color.to_iced();
    container(iced::widget::text(label).size(14).color(color)).padding(20).into()
}



fn build_entry_buttons(app: &AppData) -> Vec<Element<'_, Message>>
{
    let max = app.config.window.max_results;
    app.filtered
        .iter()
        .take(max)
        .enumerate()
        .map(|(i, entry)| build_entry_button(app, i, entry))
        .collect()
}



fn build_entry_button<'a>(app: &'a AppData, index: usize, entry: &'a AppEntry) -> Element<'a, Message>
{
    let is_selected = index == app.selected;
    let is_hovered  = app.hovered == Some(index);
    let is_calc     = entry.exec.is_empty();
    let icon        = build_icon_element(app, entry, is_selected, is_hovered, is_calc);
    let label       = build_label_element(app, entry, is_selected, is_hovered);
    let badges      = build_badge_elements(app, index, entry, is_calc);
    let content     = arrange_entry_content(app, is_selected, is_calc, icon, label, badges);

    let on_press = if is_calc 
    {
        Message::CopyToClipboard(crate::helpers::update_helpers::calc_display_value(&entry.name))
    } 
    else 
    {
        Message::Launch(entry.exec.clone())
    };

    let padding      = app.config.entry.padding;
    let style_config = app.config.clone();

    let btn = button(content)
        .on_press(on_press)
        .padding(Padding 
        {
            top:    padding[0] as f32,
            right:  padding[1] as f32,
            bottom: padding[2] as f32,
            left:   padding[3] as f32,
        })
        .width(Length::Fill)
        .style(move |_theme, status| entry_button_style(status, is_selected, &style_config));

    // Wrap in mouse_area to detect hover enter/exit and sync keyboard selection.
    let btn: Element<Message> = mouse_area(btn)
        .on_enter(Message::HoverEntry(index))
        .on_exit(Message::HoverClear)
        .into();

    if app.config.entry.show_separator && index > 0
    {
        let sep_color = app.config.entry.separator_color.to_iced();
        let sep_w     = app.config.entry.separator_width;
        let separator: Element<Message> = container(Space::new())
            .width(Length::Fill)
            .height(sep_w)
            .style(move |_| container::Style 
            {
                background: Some(iced::Background::Color(sep_color)),
                ..Default::default()
            })
            .into();
        column![separator, btn].spacing(0).into()
    }
    else
    {
        btn
    }
}



fn build_icon_element<'a>(app: &'a AppData, entry: &'a AppEntry, is_selected: bool, is_hovered: bool, is_calc: bool) -> Element<'a, Message>
{
    let icon_config = &app.config.icon;
    if !icon_config.show 
    {
        return Space::new().into();
    }

    let radius       = icon_config.border_radius;
    let (bg_color, bg_gradient) = if is_selected
    {
        (icon_config.selected_color,    icon_config.selected_gradient.as_ref())
    }
    else if is_hovered
    {
        (icon_config.hovered_color,     icon_config.hovered_gradient.as_ref())
    }
    else
    {
        (icon_config.background_color,  icon_config.background_gradient.as_ref())
    };
    let border_color = if is_selected { icon_config.selected_border_color.to_iced() } else if is_hovered { icon_config.hovered_border_color.to_iced() } else { icon_config.border_color.to_iced()       };
    let icon_color   = if is_selected { icon_config.selected_icon_color.to_iced()   } else if is_hovered { icon_config.hovered_icon_color.to_iced()   } else { icon_config.icon_color.to_iced()         };
    let opacity      = if is_selected { icon_config.selected_opacity                } else if is_hovered { icon_config.hovered_opacity                } else { icon_config.opacity                      };
    let border_width = icon_config.border_width;

    let inner: Element<Message> = if is_calc 
    {
        iced::widget::text("📋").size(icon_config.text_size).color(icon_color).into()
    } 
    else if icon_config.use_real_icons 
    {
        build_real_icon(entry, icon_config, icon_color, opacity)
    } 
    else 
    {
        let glyph = derive_icon_char(&entry.name);
        iced::widget::text(glyph).size(icon_config.text_size).color(icon_color).into()
    };

    let ip = icon_config.padding;
    container(inner)
        .width(icon_config.width)
        .height(icon_config.height)
        .padding(Padding 
        {
            top:    ip[0] as f32,
            right:  ip[1] as f32,
            bottom: ip[0] as f32,
            left:   ip[1] as f32,
        })
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(move |_| container::Style 
        {
            background: Some(color_or_gradient(bg_gradient, bg_color)),
            border: Border { color: border_color, width: border_width, radius: corner_radius(radius) },
            ..Default::default()
        })
        .into()
}



fn build_real_icon<'a>(entry: &'a AppEntry, icon_config: &crate::ron::IconConfig, icon_color: iced::Color, opacity: f32) -> Element<'a, Message>
{
    let icon_w = icon_config.width.saturating_sub(icon_config.padding[1] * 2);
    let icon_h = icon_config.height.saturating_sub(icon_config.padding[0] * 2);

    if let Some(ref path) = entry.icon_path 
    {
        return if path.ends_with(".svg") 
        {
            iced::widget::svg(path).width(icon_w).height(icon_h).opacity(opacity).into()
        }
        else 
        {
            iced::widget::image(path).width(icon_w).height(icon_h).opacity(opacity).into()
        };
    }

    let fallback_letter = entry
        .name
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "▶".to_string());

    iced::widget::text(fallback_letter).size(icon_config.text_size).color(icon_color).into()
}



fn build_label_element<'a>(app: &'a AppData, entry: &'a AppEntry, is_selected: bool, is_hovered: bool) -> Element<'a, Message> 
{
    let entry_config = &app.config.entry;
    let wrapping     = if entry_config.wrap_word { Wrapping::Word } else { Wrapping::None };

    let name_text    = apply_entry_text_rules(&entry.name,    entry_config.name_max_chars,    entry_config.ellipsize_instead_of_wrapping, entry_config.wrap_word, &entry_config.ellipsis);
    let comment_text = apply_entry_text_rules(&entry.comment, entry_config.comment_max_chars, entry_config.ellipsize_instead_of_wrapping, entry_config.wrap_word, &entry_config.ellipsis);

    let name_color    = if is_selected { entry_config.selected_name_color.to_iced()    } else if is_hovered { entry_config.hovered_name_color.to_iced()    } else { entry_config.name_color.to_iced()    };
    let comment_color = if is_selected { entry_config.selected_comment_color.to_iced() } else if is_hovered { entry_config.hovered_comment_color.to_iced() } else { entry_config.comment_color.to_iced() };

    let name_font    = make_font_family(&entry_config.name_font_weight,    &entry_config.name_font_style,    &entry_config.name_font_family);
    let comment_font = make_font_family(&entry_config.comment_font_weight, &entry_config.comment_font_style, &entry_config.comment_font_family);

    let name_element = iced::widget::text(name_text)
        .size(entry_config.name_size)
        .color(name_color)
        .font(name_font)
        .align_x(horizontal_align(&entry_config.name_align))
        .wrapping(wrapping)
        .width(Length::Fill);

    let comment_element: Element<Message> = if entry_config.show_comment && !entry.comment.is_empty() 
    {
        iced::widget::text(comment_text)
            .size(entry_config.comment_size)
            .color(comment_color)
            .font(comment_font)
            .align_x(horizontal_align(&entry_config.comment_align))
            .wrapping(wrapping)
            .width(Length::Fill)
            .into()
    } 
    else 
    {
        Space::new().height(0).into()
    };

    column![name_element, comment_element]
        .spacing(entry_config.name_comment_spacing as u32)
        .width(Length::Fill)
        .into()
}



fn build_badge_elements<'a>(app: &'a AppData, index: usize, entry: &'a AppEntry, is_calc: bool) -> EntryBadges<'a>
{
    let comment_size = app.config.entry.comment_size;
    let frecency_badge = if !is_calc && app.config.entry.show_hot_apps && app.frecency.launch_count(&entry.exec) >= app.config.entry.hot_apps_threshold
    {
        let badge_color = app.config.entry.hot_apps_color.to_iced();
        Some(iced::widget::text(&app.config.entry.hot_apps_icon).size(comment_size).color(badge_color).into())
    } 
    else 
    {
        None
    };

    let shortcut_badge = if app.config.entry.show_shortcut_hint && index < 9
    {
        let hint_color = Color { r: 0.5, g: 0.5, b: 0.5, a: 0.6 };
        let prefix = &app.config.keybinds.launch_alt_prefix;
        Some(iced::widget::text(format!("{}+{}", prefix, index + 1)).size(comment_size.saturating_sub(1)).color(hint_color).into())
    } 
    else 
    {
        None
    };

    EntryBadges { frecency_badge, shortcut_badge }
}



fn arrange_entry_content<'a>(app: &'a AppData, is_selected: bool, is_calc: bool, icon: Element<'a, Message>, label: Element<'a, Message>, badges: EntryBadges<'a>) -> Element<'a, Message>
{
    let icon_config  = &app.config.icon;
    let entry_config = &app.config.entry;
    let gap          = if icon_config.show { icon_config.gap } else { 0 };

    match entry_config.label_position 
    {
        LabelPosition::Right => arrange_label_right(app, is_selected, is_calc, icon, label, badges, gap),

        LabelPosition::Left =>
        {
            row![label, Space::new().width(gap), icon]
                .align_y(Alignment::Center)
                .into()
        }

        LabelPosition::Below => 
        {
            column!
                [
                    container(icon)
                        .align_x(Alignment::Center)
                        .width(Length::Fill),
                    Space::new().height(gap),
                    container(label)
                        .align_x(Alignment::Center)
                        .width(Length::Fill),
                ]
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .into()
        }
        LabelPosition::Above => 
        {
            column!
                [
                    container(label)
                        .align_x(Alignment::Center)
                        .width(Length::Fill),
                    Space::new().height(gap),
                    container(icon)
                        .align_x(Alignment::Center)
                        .width(Length::Fill),
                ]
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .into()
        }
    }
}



fn arrange_label_right<'a>(app: &'a AppData, is_selected: bool, is_calc: bool, icon: Element<'a, Message>, label: Element<'a, Message>, badges: EntryBadges<'a>, gap: u32) -> Element<'a, Message> 
{
    if is_calc && is_selected && app.copy_feedback 
    {
        return build_calc_copy_feedback(app, icon, label, gap);
    }

    let right_badges = collect_right_badges(badges);

    if right_badges.is_empty() 
    {
        row![icon, Space::new().width(gap), label]
            .align_y(Alignment::Center)
            .into()
    }
    else 
    {
        let badge_row: Element<Message> = container(row(right_badges).align_y(Alignment::Center))
            .padding(Padding { top: 0.0, right: 6.0, bottom: 0.0, left: 4.0 })
            .into();

        row![icon, Space::new().width(gap), label, Space::new().width(Length::Fill), badge_row]
            .align_y(Alignment::Center)
            .into()
    }
}



fn build_calc_copy_feedback<'a>(app: &'a AppData, icon: Element<'a, Message>, label: Element<'a, Message>, gap: u32) -> Element<'a, Message>
{
    let feedback_text = if app.wl_copy_available 
    {
        app.config.behaviour.copy_feedback_text.clone()
    } 
    else 
    {
        "wl-copy not installed".to_string()
    };

    let feedback: Element<Message> = iced::widget::text(feedback_text)
        .size(app.config.entry.comment_size)
        .color(app.config.search.border_color.to_iced())
        .into();

    row![icon, Space::new().width(gap), label, Space::new().width(Length::Fill), feedback]
        .align_y(Alignment::Center)
        .into()
}

fn collect_right_badges(badges: EntryBadges<'_>) -> Vec<Element<'_, Message>>
{
    let mut collected: Vec<Element<Message>> = Vec::new();

    if let Some(frecency) = badges.frecency_badge 
    {
        collected.push(frecency);
    }

    if let Some(shortcut) = badges.shortcut_badge 
    {
        if !collected.is_empty() 
        {
            collected.push(container(Space::new()).width(4).into());
        }
        collected.push(shortcut);
    }

    collected
}



fn arrange_into_grid<'a>(app: &'a AppData, buttons: Vec<Element<'a, Message>>) -> Vec<Element<'a, Message>> 
{
    let window_config = &app.config.window;
    let entry_config  = &app.config.entry;
    let icon_config   = &app.config.icon;
    let cols          = window_config.grid_side_items.max(1);
    let max           = app.filtered.len().min(window_config.max_results);
    let col_spacing   = window_config.grid_column_spacing as f32;
    let mut button_iter = buttons.into_iter();
    let mut grid_rows: Vec<Element<Message>> = Vec::new();
    let mut row_start = 0;

    while row_start < max 
    {
        let row_end   = (row_start + cols).min(max);
        let row_slice = &app.filtered[row_start .. row_end];
        let row_has_comment = row_slice.iter().any(|e| entry_config.show_comment && !e.comment.is_empty());
        let cell_height     = row_height(entry_config, icon_config, row_has_comment);

        let mut cells: Vec<Element<Message>> = button_iter
            .by_ref()
            .take(cols)
            .map(|cell| container(cell).clip(true).width(Length::Fill).height(cell_height).into())
            .collect();

        if cells.is_empty() 
        {
            break;
        }

        while cells.len() < cols 
        {
            cells.push(container(Space::new()).width(Length::Fill).height(cell_height).into());
        }

        grid_rows.push(row(cells).spacing(col_spacing).into());
        row_start = row_end;
    }

    grid_rows
}



fn build_scrollable<'a>(app: &'a AppData, grid_rows: Vec<Element<'a, Message>>) -> Element<'a, Message>
{
    let scrollbar_config = &app.config.scrollbar;
    let window_config    = &app.config.window;
    let entry_config     = &app.config.entry;
    let radius           = scrollbar_config.border_radius;

    let make_rail = |scroller_color: iced::Color| Rail 
    {
        background: Some(iced::Background::Color(scrollbar_config.rail_color.to_iced())),
        border: Border 
        {
            color:  scrollbar_config.rail_border_color.to_iced(),
            width:  scrollbar_config.rail_border_width,
            radius: corner_radius(radius),
        },
        scroller: Scroller 
        {
            background: iced::Background::Color(scroller_color),
            border: Border 
            {
                color:  scrollbar_config.scroller_border_color.to_iced(),
                width:  scrollbar_config.scroller_border_width,
                radius: corner_radius(radius),
            },
        },
    };

    let rail_idle    = make_rail(scrollbar_config.scroller_color.to_iced());
    let rail_hovered = make_rail(scrollbar_config.scroller_hovered_color.to_iced());
    let rail_dragged = make_rail(scrollbar_config.scroller_dragging_color.to_iced());

    let (bar_width, bar_margin, scroller_width) = if scrollbar_config.show 
    {
        (scrollbar_config.width, scrollbar_config.margin, scrollbar_config.scroller_width)
    } 
    else 
    {
        (0, 0, 0)
    };

    let entry_width = if entry_config.width > 0 
    {
        Length::Fixed(entry_config.width as f32)
    } 
    else 
    {
        Length::Fill
    };

    scrollable(column(grid_rows).spacing(window_config.entry_spacing))
        .id(Id::new("results_scroll"))
        .on_scroll(|vp| 
        {
            Message::Scrolled(vp.relative_offset().y, vp.bounds().height, vp.content_bounds().height)
        })
        .direction(scrollable::Direction::Vertical
        (
            scrollable::Scrollbar::new()
                .width(bar_width)
                .margin(bar_margin)
                .scroller_width(scroller_width),
        ))
        .style(move |_theme, status| 
        {
            let rail = match status 
            {
                scrollable::Status::Dragged { .. } => rail_dragged,
                scrollable::Status::Hovered { .. } => rail_hovered,
                _ => rail_idle,
            };
            scrollable::Style 
            {
                container: container::Style::default(),
                vertical_rail: rail,
                horizontal_rail: rail_idle,
                gap: None,
                auto_scroll: scrollable::default(_theme, status).auto_scroll,
            }
        })
        .width(entry_width)
        .height(Length::Fill)
        .into()
}
