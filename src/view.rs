// ============ IMPORTS ============
use iced::{
    Alignment, Color, Element, Length, Padding, Vector,
    border::Radius,
    widget::{Space, button, column, container, row, scrollable, text, text_input},
};
use iced_layershell::reexport::core::{Shadow, Border};




// ============ CRATES ============
use crate::{AppData, Message};
use crate::helpers::{derive_icon_char, entry_button_style, truncate};




// ============ VIEW ============
pub fn view(app: &AppData) -> Element<'_, Message>
{
    let cfg = &app.config;
    let wc  = &cfg.window;
    let sc  = &cfg.search;
    let ec  = &cfg.entry;
    let ic  = &cfg.icon;
    let fc  = &cfg.footer;

    // ── Search bar ───────────────────────────────────────────────────────────
    let sr = sc.border_radius;
    let search_input = text_input(&sc.placeholder, &app.query)
        .id("search_input")
        .on_input(Message::QueryChanged)
        .on_submit(app.filtered.get(app.selected).map(|e| Message::Launch(e.exec.clone())).unwrap_or(Message::Nothing))
        .size(sc.text_size)
        .padding(sc.input_padding as f32)
        .style(move |_theme, _status| iced::widget::text_input::Style
        {
            background:  iced::Background::Color(sc.background_color.to_iced()),
            border: Border
            {
                color:  sc.border_color.to_iced(),
                width:  sc.border_width,
                radius: Radius { top_left: sr[0], top_right: sr[1], bottom_left: sr[2], bottom_right: sr[3] },
            },
            icon:        sc.placeholder_color.to_iced(),
            placeholder: sc.placeholder_color.to_iced(),
            value:       sc.text_color.to_iced(),
            selection:   sc.selection_color.to_iced(),
        });

    let search_bar = container(search_input)
        .padding(Padding { top: 0.0, right: 0.0, bottom: sc.bottom_margin, left: 0.0 });


    // ── Results ──────────────────────────────────────────────────────────────
    let results: Element<Message> = if app.loading
    {
        container(
            text("Loading applications...")
                .size(14)
                .color(Color { r: 0.7, g: 0.7, b: 0.7, a: 1.0 })
        )
        .padding(20)
        .into()
    }
    else if app.filtered.is_empty()
    {
        container(
            text("No results found")
                .size(14)
                .color(Color { r: 0.7, g: 0.7, b: 0.7, a: 1.0 })
        )
        .padding(20)
        .into()
    }
    else
    {
        let max = cfg.window.max_results;

        let items: Vec<Element<Message>> = app.filtered.iter().take(max).enumerate()
            .map(|(i, entry)|
            {
                let is_selected = i == app.selected;
                let exec        = entry.exec.clone();

                // Icon badge
                let icon_el: Element<Message> = if ic.show
                {
                    let ir       = ic.border_radius;
                    let ic_border = ic.border_color.to_iced();
                    let ic_w      = ic.border_width;
                    let icon_bg   = if is_selected { ic.selected_color.to_iced() } else { ic.background_color.to_iced() };
                
                    // ── Real icon image ──────────────────────────────────────────────
                    let inner: Element<Message> = if ic.use_real_icons
                    {
                        if let Some(ref path) = entry.icon_path
                        {
                                if path.ends_with(".svg")
                                {
                                    iced::widget::svg(path).width(ic.width  - 8).height(ic.height - 8).into()
                                }
                                else
                                {
                                    iced::widget::image(path).width(ic.width  - 8).height(ic.height - 8).into()
                                }
                        }
                        else
                        {
                            // No icon found — fall back to first letter of name
                            let letter = entry.name.chars().next()
                                .map(|c| c.to_uppercase().to_string())
                                .unwrap_or_else(|| "▶".to_string());
                            let color = if is_selected { Color::WHITE } else { ic.icon_color.to_iced() };
                            text(letter).size(ic.text_size).color(color).into()
                        }
                    }
                    else
                    {
                        // Abstract emoji badge (original behaviour)
                        let icon_char  = derive_icon_char(&entry.name);
                        let icon_color = if is_selected { Color::WHITE } else { ic.icon_color.to_iced() };
                        text(icon_char).size(ic.text_size).color(icon_color).into()
                    };
                
                    container(inner)
                        .width(ic.width)
                        .height(ic.height)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .style(move |_| container::Style
                        {
                            background: Some(iced::Background::Color(icon_bg)),
                            border: Border
                            {
                                color:  ic_border,
                                width:  ic_w,
                                radius: Radius { top_left: ir[0], top_right: ir[1], bottom_left: ir[2], bottom_right: ir[3] },
                            },
                            ..Default::default()
                        })
                        .into()
                }
                else { Space::new().into() };

                // Text label
                let name_el = text(&entry.name).size(ec.name_size).color(ec.name_color.to_iced());

                let comment_el: Element<Message> = if ec.show_comment && !entry.comment.is_empty()
                {
                    text(truncate(&entry.comment, ec.comment_max_chars))
                        .size(ec.comment_size)
                        .color(ec.comment_color.to_iced())
                        .into()
                }
                else { Space::new().into() };

                let gap = if ic.show { ic.gap } else { 0 };
                let gap_el: Element<Message> = container(Space::new()).width(gap).into();
                let label = column![name_el, comment_el].spacing(2);
                let row_content = row![icon_el, gap_el, label]
                    .align_y(Alignment::Center);

                let ep = ec.padding;
                button(row_content)
                    .on_press(Message::Launch(exec))
                    .padding(Padding { top: ep[0] as f32, right: ep[1] as f32, bottom: ep[0] as f32, left: ep[1] as f32 })
                    .width(Length::Fill)
                    .style(move |_theme, status| entry_button_style(status, is_selected, &cfg.clone()))
                    .into()
            })
            .collect();

        let cols = wc.grid_side_items.max(1);
        let mut items_iter = items.into_iter();
        let mut grid_rows: Vec<Element<Message>> = Vec::new();
        
        loop 
        {
            let chunk: Vec<Element<Message>> = items_iter.by_ref().take(cols).collect();
            if chunk.is_empty() { break; }
            let mut cells: Vec<Element<Message>> = chunk.into_iter().map(|cell| container(cell).width(Length::Fill).into()).collect();
            while cells.len() < cols 
            {
                cells.push(Space::new().width(Length::Fill).into());
            }
            grid_rows.push(row(cells).spacing(wc.entry_spacing).into());
        }

        scrollable(column(grid_rows).spacing(wc.entry_spacing)).height(Length::Fill).into()
    };


    // ── Footer ───────────────────────────────────────────────────────────────
    let footer: Element<Message> = if fc.show
    {
        let hint_str = if fc.hint_text.is_empty()
        {
            "↑↓ navigate  •  Enter launch  •  Esc close".to_string()
        }
        else { fc.hint_text.clone() };

        let hint_el: Element<Message> = if fc.show_hint
        {
            text(hint_str).size(fc.text_size).color(fc.text_color.to_iced()).into()
        }
        else { Space::new().into() };

        let count_el: Element<Message> = if fc.show_count && !app.loading
        {
            let shown = app.filtered.len().min(cfg.window.max_results);
            let total = app.filtered.len();
            let label = if total > cfg.window.max_results
            {
                format!("{} / {} results", shown, total)
            }
            else
            {
                format!("{} result{}", total, if total == 1 { "" } else { "s" })
            };
            text(label).size(fc.text_size).color(fc.text_color.to_iced()).into()
        }
        else { Space::new().into() };

        row![hint_el, Space::new().width(Length::Fill), count_el]
            .align_y(Alignment::Center)
            .padding(Padding { top: fc.top_margin, right: 0.0, bottom: 0.0, left: 0.0 })
            .into()
    }
    else { Space::new().into() };


    // ── Outer panel ──────────────────────────────────────────────────────────
    let content = column![search_bar, results, footer]
        .spacing(wc.section_spacing)
        .padding(wc.padding as f32);

    let wr = wc.border_radius;
    let w_border = wc.border_color.to_iced();
    let w_bg     = wc.background_color.to_iced();
    let w_sh_col = wc.shadow_color.to_iced();
    let w_sh_x   = wc.shadow_offset_x;
    let w_sh_y   = wc.shadow_offset_y;
    let w_sh_bl  = wc.shadow_blur;
    let w_bw     = wc.border_width;

    container(content)
        .width(wc.width)
        .style(move |_| container::Style
        {
            background: Some(iced::Background::Color(w_bg)),
            border: Border
            {
                color:  w_border,
                width:  w_bw,
                radius: Radius { top_left: wr[0], top_right: wr[1], bottom_left: wr[2], bottom_right: wr[3] },
            },
            shadow: Shadow { color: w_sh_col, offset: Vector::new(w_sh_x, w_sh_y), blur_radius: w_sh_bl },
            text_color: Some(Color::WHITE),
            snap: false,
        })
        .into()
}
