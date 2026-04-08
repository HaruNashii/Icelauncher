// ============ IMPORTS ============
use iced::{Alignment, Element, Font, Length, Padding, widget::{Space, column, container, row, text}}; 
use iced_layershell::reexport::core::Border;




// ============ CRATES ============
use crate::{AppData, Message};
use crate::helpers::color::color_or_gradient;
use crate::helpers::widget::{corner_radius, optional_length, optional_length_shrink};
use crate::ron::{FooterOrientation, FooterPosition};




// ============ FUNCTIONS ============
pub fn build_footer(app: &AppData) -> Element<'_, Message>
{
    let footer_config = &app.config.footer;
    if !footer_config.show 
    {
        return Space::new().into();
    }

    let hint_text  = resolve_hint_text(footer_config, app.shell_mode);
    let count_text = resolve_count_text(app);
    let is_vertical = footer_config.text_orientation == FooterOrientation::Vertical;

    let hint_element = if footer_config.show_hint 
    {
        make_footer_text(hint_text, footer_config.text_size, footer_config.hint_color.to_iced(), app.footer_font, is_vertical)
    } 
    else 
    {
        Space::new().into()
    };

    let count_element = if footer_config.show_count && !app.loading 
    {
        make_footer_text(count_text, footer_config.text_size, footer_config.count_color.to_iced(), app.footer_font, is_vertical)
    }
    else 
    {
        Space::new().into()
    };

    let fp    = footer_config.padding;
    let fbr   = footer_config.border_radius;
    let fc_bg = color_or_gradient(footer_config.background_gradient.as_ref(), footer_config.background_color);
    let fc_bc = footer_config.border_color.to_iced();
    let fc_bw = footer_config.border_width;
    let inner_pad = Padding 
    {
        top:    fp[0] as f32 + footer_config.top_margin,
        right:  fp[1] as f32,
        bottom: fp[2] as f32,
        left:   fp[3] as f32,
    };

    let is_sidebar = matches!(footer_config.position, FooterPosition::Left | FooterPosition::Right);

    let inner: Element<Message> = if is_vertical || is_sidebar 
    {
        column![hint_element, Space::new().height(Length::Fill), count_element]
            .align_x(Alignment::Center)
            .padding(inner_pad)
            .into()
    } 
    else 
    {
        row![hint_element, Space::new().width(Length::Fill), count_element]
            .align_y(Alignment::Center)
            .padding(inner_pad)
            .into()
    };

    let fc_w = optional_length(footer_config.width);
    let fc_h = optional_length_shrink(footer_config.height);

    container(inner)
        .width(fc_w)
        .height(fc_h)
        .style(move |_| container::Style 
        {
            background: Some(fc_bg),
            border: Border { color: fc_bc, width: fc_bw, radius: corner_radius(fbr) },
            ..Default::default()
        })
        .into()
}



fn resolve_hint_text(footer_config: &crate::ron::FooterConfig, shell_mode: bool) -> String
{
    if !footer_config.hint_text.is_empty()
    {
        return footer_config.hint_text.clone();
    }
    if shell_mode
    {
        "↑↓ history  •  Enter run  •  Alt+1-9 quick run  •  Esc close  •  [shell mode]".to_string()
    }
    else
    {
        "↑↓ navigate  •  Enter launch  •  Alt+1-9 quick launch  •  Alt+L relaunch  •  Esc close".to_string()
    }
}



fn resolve_count_text(app: &AppData) -> String
{
    let shown = app.filtered.len().min(app.config.window.max_results);
    let total = app.filtered.len();

    if total > app.config.window.max_results
    {
        app.config.footer.count_format
            .replace("{shown}", &shown.to_string())
            .replace("{total}", &total.to_string())
    }
    else if total == 1
    {
        app.config.footer.single_format
            .replace("{total}", &total.to_string())
    }
    else
    {
        app.config.footer.count_format
            .replace("{shown}", &total.to_string())
            .replace("{total}", &total.to_string())
    }
}



fn make_footer_text<'a>(content: String, size: u32, color: iced::Color, font: Font, is_vertical: bool) -> Element<'a, Message>
{
    if is_vertical 
    {
        let chars: Vec<Element<Message>> = content
            .chars()
            .map(|c| 
            {
                container(text(c.to_string()).size(size).color(color).font(font))
                    .width(Length::Fill)
                    .align_x(Alignment::Center)
                    .into()
            })
            .collect();
        column(chars).spacing(1).width(Length::Fill).into()
    } 
    else 
    {
        text(content).size(size).color(color).font(font).into()
    }
}
