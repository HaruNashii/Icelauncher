// ============ IMPORTS ============
use iced::{Alignment, Color, Element, Length, Padding, widget::{Space, column, container, row, stack}};
use iced_layershell::reexport::core::{Border, Shadow};




// ============ CRATES ============
use crate::{AppData, Message};
use crate::helpers::widget::{corner_radius, optional_length_shrink};
use crate::ron::SearchPosition;




// ============ FUNCTIONS ============
pub fn assemble_layout<'a>(app: &'a AppData, search_bar: Element<'a, Message>, results_block: Element<'a, Message>) -> Element<'a, Message>
{
    let window_config = &app.config.window;
    let search_config = &app.config.search;
    let section_gap = window_config.section_spacing;
    let padding = window_config.padding as f32;

    match search_config.position 
    {
        SearchPosition::Top => 
        {
            column![search_bar, results_block].spacing(section_gap).padding(padding).into()
        }
        SearchPosition::Bottom => 
        {
            column![results_block, search_bar].spacing(section_gap).padding(padding).into()
        }
        SearchPosition::Left => 
        {
            row![search_bar, results_block].spacing(section_gap).padding(padding).into()
        }
        SearchPosition::Right => 
        {
            row![results_block, search_bar].spacing(section_gap).padding(padding).into()
        }
        SearchPosition::TopLeft => 
        {
            column![row![search_bar, Space::new().width(Length::Fill)], results_block]
                .spacing(section_gap)
                .padding(padding)
                .into()
        }
        SearchPosition::TopRight => 
        {
            column![row![Space::new().width(Length::Fill), search_bar], results_block]
                .spacing(section_gap)
                .padding(padding)
                .into()
        }
        SearchPosition::BottomLeft => 
        {
            column![results_block, row![search_bar, Space::new().width(Length::Fill)]]
                .spacing(section_gap)
                .padding(padding)
                .into()
        }
        SearchPosition::BottomRight => 
        {
            column![results_block, row![Space::new().width(Length::Fill), search_bar]]
                .spacing(section_gap)
                .padding(padding)
                .into()
        }
    }
}



pub fn build_window_panel<'a>(app: &'a AppData, content: Element<'a, Message>) -> Element<'a, Message>
{
    let window_config = &app.config.window;
    let has_bg_images = app.config.background_images.iter().any(|bg| !bg.path.is_empty());
    let panel_bg = if has_bg_images { Color::TRANSPARENT } else { window_config.background_color.to_iced() };

    let wr = window_config.border_radius;
    let w_bc = window_config.border_color.to_iced();
    let w_bw = window_config.border_width;
    let w_sh = Shadow 
    {
        color: window_config.shadow_color.to_iced(),
        offset: iced::Vector::new(window_config.shadow_offset_x, window_config.shadow_offset_y),
        blur_radius: window_config.shadow_blur,
    };

    let panel: Element<Message> = container(content)
        .width(window_config.width)
        .height(window_config.height)
        .style(move |_| container::Style 
        {
            background: Some(iced::Background::Color(panel_bg)),
            text_color: Some(Color::WHITE),
            snap: false,
            border: Border 
            { 
                color: w_bc,
                width: w_bw,
                radius: corner_radius(wr) 
            },
            shadow: w_sh,
        })
        .into();

    if !has_bg_images 
    {
        return panel;
    }

    build_layered_panel(app, panel)
}



fn build_layered_panel<'a>(app: &'a AppData, panel: Element<'a, Message>) -> Element<'a, Message>
{
    let window_config = &app.config.window;
    let wr = window_config.border_radius;
    let w_bc = window_config.border_color.to_iced();
    let w_bw = window_config.border_width;
    let w_bg = window_config.background_color.to_iced();
    let win_w = window_config.width;
    let win_h = window_config.height;

    let mut layers: Vec<Element<Message>> = vec!
    [
        container(Space::new())
            .width(win_w)
            .height(win_h)
            .style(move |_| container::Style 
            {
                background: Some(iced::Background::Color(w_bg)),
                border: Border 
                { 
                    color: w_bc,
                    width: w_bw,
                    radius: corner_radius(wr) 
                },
                ..Default::default()
            })
            .into(),
    ];

    for bg in app.config.background_images.iter().filter(|bg| !bg.path.is_empty()) 
    {
        let img_w = optional_length_shrink(bg.width);
        let img_h = optional_length_shrink(bg.height);
        let fit = bg.content_fit.clone().to_iced();

        let img: Element<Message> = if bg.path.ends_with(".svg") 
        {
            iced::widget::svg(&bg.path)
                .width(img_w)
                .height(img_h)
                .content_fit(fit)
                .opacity(bg.opacity)
                .into()
        } 
        else 
        {
            iced::widget::image(&bg.path)
                .width(img_w)
                .height(img_h)
                .content_fit(fit)
                .opacity(bg.opacity)
                .into()
        };

        layers.push
        (
            container(img)
                .padding
                (Padding 
                { 
                    top: bg.y,
                    left: bg.x,
                    right: 0.0,
                    bottom: 0.0 
                })
                .width(win_w)
                .height(win_h)
                .align_x(Alignment::Start)
                .align_y(Alignment::Start)
                .into(),
        );
    }

    layers.push(panel);
    stack(layers).width(win_w).height(win_h).into()
}
