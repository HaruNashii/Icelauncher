// ============ IMPORTS ============
use iced::{Alignment, Color, Element, Length, Padding, Vector, border::Radius, widget::{Id, Space, button, column, container, row, scrollable, text, text_input}};
use iced::widget::scrollable::{Rail, Scroller};
use iced_layershell::reexport::core::{Shadow, Border};




// ============ CRATES ============
use crate::helpers::{icon::derive_icon_char, style::entry_button_style, text::truncate};
use crate::ron::{SearchOrientation, SearchPosition};
use crate::{AppData, Message};




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
    let search_input = text_input(&sc.placeholder, &app.query).id("search_input").on_input(Message::QueryChanged)
    .on_submit
    ({
        match app.filtered.get(app.selected) 
        {
            Some(e) if e.exec.is_empty() => Message::CopyToClipboard(e.name.trim_start_matches("= ").to_string()),
            Some(e) => Message::Launch(e.exec.clone()),
            None    => Message::Nothing,
        }
    })
    .size(sc.text_size).padding(sc.input_padding as f32)
    .style(move |_theme, _status| iced::widget::text_input::Style
    {
        background:  iced::Background::Color(sc.background_color.to_iced()),
        icon:        sc.placeholder_color.to_iced(),
        placeholder: sc.placeholder_color.to_iced(),
        value:       sc.text_color.to_iced(),
        selection:   sc.selection_color.to_iced(),
        border: Border
        {
            color:  sc.border_color.to_iced(),
            width:  sc.border_width,
            radius: Radius { top_left: sr[0], top_right: sr[1], bottom_left: sr[2], bottom_right: sr[3] },
        },
    });

    // Vertical orientation: render each character of the query on its own line
    let is_vertical  = sc.orientation == SearchOrientation::Vertical;
    let is_left      = sc.position    == SearchPosition::Left;
    let is_right     = sc.position    == SearchPosition::Right;
    let is_side      = is_left || is_right;

    let search_element: Element<Message> = if is_vertical
    {
        let displayed: String = if app.query.is_empty() { sc.placeholder.clone() } else { app.query.clone() };
        let char_els: Vec<Element<Message>> = displayed.chars().map(|c|
        {
            let color = if app.query.is_empty() { sc.placeholder_color.to_iced() } else { sc.text_color.to_iced() };
            container(text(c.to_string()).size(sc.text_size).color(color))
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .into()
        }).collect();

        let char_col = column(char_els).spacing(2).width(Length::Fill);
        let side_w = sc.side_width;

        // Transparent 1px-wide input — narrow enough to be invisible, tall enough
        // for iced to keep it focusable and route all keyboard events through it.
        let transparent_input = text_input("", &app.query)
            .id("search_input")
            .on_input(Message::QueryChanged)
            .on_submit
            ({
                match app.filtered.get(app.selected) 
                {
                    Some(e) if e.exec.is_empty() => Message::CopyToClipboard(e.name.trim_start_matches("= ").to_string()),
                    Some(e) => Message::Launch(e.exec.clone()),
                    None    => Message::Nothing,
                }
            })
            .size(sc.text_size)
            .padding(0)
            .style(|_theme, _status| iced::widget::text_input::Style
            {
                background:  iced::Background::Color(Color::TRANSPARENT),
                icon:        Color::TRANSPARENT,
                placeholder: Color::TRANSPARENT,
                value:       Color::TRANSPARENT,
                selection:   Color::TRANSPARENT,
                border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radius::default() },
            });

        // Row: 1px invisible input on the left for focus, char column fills the rest
        let inner = row![
            container(transparent_input).width(1).height(Length::Fill),
            container(char_col).width(Length::Fill).align_x(Alignment::Center).padding(sc.input_padding as f32)
        ];

        container(inner)
            .width(side_w)
            .height(Length::Fill)
            .style(move |_| container::Style
            {
                background: Some(iced::Background::Color(sc.background_color.to_iced())),
                border: Border
                {
                    color:  sc.border_color.to_iced(),
                    width:  sc.border_width,
                    radius: Radius { top_left: sr[0], top_right: sr[1], bottom_left: sr[2], bottom_right: sr[3] },
                },
                ..Default::default()
            })
            .into()
    }
    else
    {
        search_input.into()
    };

    // Margin direction depends on position
    let search_bar: Element<Message> = if is_side
    {
        let side_margin = sc.bottom_margin; // reuse bottom_margin as side gap
        if is_left
        {
            container(search_element).padding(Padding { top: 0.0, right: side_margin, bottom: 0.0, left: 0.0 }).into()
        }
        else
        {
            container(search_element).padding(Padding { top: 0.0, right: 0.0, bottom: 0.0, left: side_margin }).into()
        }
    }
    else if sc.position == SearchPosition::Bottom
    {
        container(search_element).padding(Padding { top: sc.bottom_margin, right: 0.0, bottom: 0.0, left: 0.0 }).into()
    }
    else
    {
        // Top (default)
        container(search_element).padding(Padding { top: 0.0, right: 0.0, bottom: sc.bottom_margin, left: 0.0 }).into()
    };


    // ── Results ──────────────────────────────────────────────────────────────
    let results: Element<Message> = if app.loading
    {
        container(text("Loading applications...").size(14).color(Color { r: 0.7, g: 0.7, b: 0.7, a: 1.0 })).padding(20).into()
    }
    else if app.filtered.is_empty()
    {
        container(text("No results found").size(14).color(Color { r: 0.7, g: 0.7, b: 0.7, a: 1.0 })).padding(20).into()
    }
    else
    {
        let max = cfg.window.max_results;

        let items: Vec<Element<Message>> = app.filtered.iter().take(max).enumerate().map(|(i, entry)|
        {
            let is_selected  = i == app.selected;
            let is_calc      = entry.exec.is_empty(); // calc entries have no exec
            let exec         = entry.exec.clone();

            // Extract the numeric value from "= 8" for copying
            let calc_value   = if is_calc { entry.name.trim_start_matches("= ").to_string() } else { String::new() };

            let icon_el: Element<Message> = if ic.show
            {
                let ir        = ic.border_radius;
                let ic_border = ic.border_color.to_iced();
                let ic_w      = ic.border_width;
                let icon_bg   = if is_selected { ic.selected_color.to_iced() } else { ic.background_color.to_iced() };
            
                // ── Real icon image ──────────────────────────────────────────────
                let inner: Element<Message> = if is_calc
                {
                    // Always show clipboard emoji for calc results
                    let color = if is_selected { Color::WHITE } else { ic.icon_color.to_iced() };
                    text("📋").size(ic.text_size).color(color).into()
                }
                else if ic.use_real_icons
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
                        let letter = entry.name.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_else(|| "▶".to_string());
                        let color = if is_selected { Color::WHITE } else { ic.icon_color.to_iced() };
                        text(letter).size(ic.text_size).color(color).into()
                    }
                }
                else
                {
                    let icon_char  = derive_icon_char(&entry.name);
                    let icon_color = if is_selected { Color::WHITE } else { ic.icon_color.to_iced() };
                    text(icon_char).size(ic.text_size).color(icon_color).into()
                };
            
                container(inner).width(ic.width).height(ic.height).align_x(Alignment::Center).align_y(Alignment::Center)
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
                }).into()
            }
            else 
            { 
                Space::new().into() 
            };

            let name_el = text(&entry.name).size(ec.name_size).color(ec.name_color.to_iced());
            let comment_el: Element<Message> = if ec.show_comment && !entry.comment.is_empty()
            {
                text(truncate(&entry.comment, ec.comment_max_chars)).size(ec.comment_size).color(ec.comment_color.to_iced()).into()
            }
            else 
            { 
                Space::new().into() 
            };

            let gap = if ic.show { ic.gap } else { 0 };
            let gap_el: Element<Message> = container(Space::new()).width(gap).into();
            let label = column![name_el, comment_el].spacing(2);

            // For calc entries, optionally show the feedback text on the right
            let row_content: Element<Message> = if is_calc && is_selected && app.copy_feedback
            {
                let feedback_el = text(&cfg.behaviour.copy_feedback_text).size(ec.comment_size).color(sc.border_color.to_iced()); // use accent color
                row![icon_el, gap_el, label, Space::new().width(Length::Fill), feedback_el].align_y(Alignment::Center).into()
            }
            else
            {
                row![icon_el, gap_el, label].align_y(Alignment::Center).into()
            };

            let on_press = if is_calc
            {
                Message::CopyToClipboard(calc_value)
            }
            else
            {
                Message::Launch(exec)
            };

            let ep = ec.padding;
            button(row_content).on_press(on_press).padding(Padding { top: ep[0] as f32, right: ep[1] as f32, bottom: ep[0] as f32, left: ep[1] as f32 }).width(Length::Fill).style(move |_theme, status| entry_button_style(status, is_selected, &cfg.clone())).into()
        }).collect();

        let cols = wc.grid_side_items.max(1);
        let ep = ec.padding;
        let base_h = (ec.name_size as f32) + ep[0] as f32 * 2.0 + 8.0;
        let tall_h = base_h + (ec.comment_size as f32) + 6.0;
        let filtered_chunk: Vec<Vec<&_>> = app.filtered.iter().take(max).collect::<Vec<_>>().chunks(cols).map(|c| c.to_vec()).collect();
        let mut items_iter = items.into_iter();
        let mut grid_rows: Vec<Element<Message>> = Vec::new();

        for chunk_entries in &filtered_chunk
        {
            let row_has_comment = chunk_entries.iter().any(|e| ec.show_comment && !e.comment.is_empty());
            let cell_h = if row_has_comment { tall_h } else { base_h };
            let chunk: Vec<Element<Message>> = items_iter.by_ref().take(cols).collect();
            if chunk.is_empty() { break; }
            let mut cells: Vec<Element<Message>> = chunk.into_iter().map(|cell| container(cell).width(Length::Fill).height(cell_h).into()).collect();
            while cells.len() < cols
            {
                cells.push(container(Space::new()).width(Length::Fill).height(cell_h).into());
            }
            grid_rows.push(row(cells).spacing(wc.entry_spacing).into());
        }

        let sbc  = &cfg.scrollbar;
        let sbr  = sbc.border_radius;
        let rail = Rail
        {
            background: Some(iced::Background::Color(sbc.rail_color.to_iced())),
            border: Border
            {
                color:  sbc.rail_border_color.to_iced(),
                width:  sbc.rail_border_width,
                radius: Radius { top_left: sbr[0], top_right: sbr[1], bottom_left: sbr[2], bottom_right: sbr[3] },
            },
            scroller: Scroller
            {
                background: iced::Background::Color(sbc.scroller_color.to_iced()),
                border: Border
                {
                    color:  sbc.scroller_border_color.to_iced(),
                    width:  sbc.scroller_border_width,
                    radius: Radius { top_left: sbr[0], top_right: sbr[1], bottom_left: sbr[2], bottom_right: sbr[3] },
                },
            },
        };

        let (bar_w, bar_margin, scroller_w) = if sbc.show
        {
            (sbc.width, sbc.margin, sbc.scroller_width)
        }
        else
        {
            (0, 0, 0)
        };

        scrollable(column(grid_rows).spacing(wc.entry_spacing))
            .id(Id::new("results_scroll"))
            .on_scroll(|vp| Message::Scrolled(vp.relative_offset().y, vp.bounds().height, vp.content_bounds().height))
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new()
                    .width(bar_w)
                    .margin(bar_margin)
                    .scroller_width(scroller_w)
            ))
            .style(move |_theme, _status| scrollable::Style
            {
                container:       container::Style::default(),
                vertical_rail:   rail,
                horizontal_rail: rail,
                gap:             None,
                auto_scroll:     scrollable::default(_theme, _status).auto_scroll,
            })
            .height(Length::Fill)
            .into()
    };


    // ── Footer ───────────────────────────────────────────────────────────────
    let footer: Element<Message> = if fc.show
    {
        let hint_str = if fc.hint_text.is_empty()
        {
            "↑↓ navigate  •  Enter launch  •  Esc close".to_string()
        }
        else 
        { 
            fc.hint_text.clone() 
        };

        let hint_el: Element<Message> = if fc.show_hint
        {
            text(hint_str).size(fc.text_size).color(fc.text_color.to_iced()).into()
        }
        else 
        { 
            Space::new().into() 
        };

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
        else 
        { 
            Space::new().into() 
        };

        row![hint_el, Space::new().width(Length::Fill), count_el].align_y(Alignment::Center).padding(Padding { top: fc.top_margin, right: 0.0, bottom: 0.0, left: 0.0 }).into()
    }
    else 
    { 
        Space::new().into() 
    };


    // ── Outer panel ──────────────────────────────────────────────────────────
    let content: Element<Message> = match sc.position
    {
        SearchPosition::Bottom => column![results, footer, search_bar].spacing(wc.section_spacing).padding(wc.padding as f32).into(),
        SearchPosition::Left => column![row![search_bar, results].spacing(wc.section_spacing), footer].padding(wc.padding as f32).into(),
        SearchPosition::Right => column![row![results, search_bar].spacing(wc.section_spacing), footer].padding(wc.padding as f32).into(),
        SearchPosition::Top => column![search_bar, results, footer].spacing(wc.section_spacing).padding(wc.padding as f32).into(),
    };
    let wr = wc.border_radius;
    let w_border = wc.border_color.to_iced();
    let w_bg     = wc.background_color.to_iced();
    let w_sh_col = wc.shadow_color.to_iced();
    let w_sh_x   = wc.shadow_offset_x;
    let w_sh_y   = wc.shadow_offset_y;
    let w_sh_bl  = wc.shadow_blur;
    let w_bw     = wc.border_width;

    container(content).width(wc.width)
    .style(move |_| container::Style
    {
        background: Some(iced::Background::Color(w_bg)),
        text_color: Some(Color::WHITE),
        snap: false,
        border: Border
        {
            color:  w_border,
            width:  w_bw,
            radius: Radius { top_left: wr[0], top_right: wr[1], bottom_left: wr[2], bottom_right: wr[3] },
        },
        shadow: Shadow 
        { 
            color: w_sh_col, 
            offset: Vector::new(w_sh_x, w_sh_y),
            blur_radius: w_sh_bl 
        },
    }).into()
}
