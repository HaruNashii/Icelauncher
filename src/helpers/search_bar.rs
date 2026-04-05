use iced_layershell::reexport::core::Border;
use iced::{Alignment, Color, Element, Length, Padding, border::Radius, widget::{column, container, row, text, text_input}};




use crate::
{
    AppData, Message,
    helpers::widget::{corner_radius, make_font, optional_length, optional_length_shrink},
    ron::{SearchOrientation, SearchPosition},
};




pub fn build_search_bar(app: &AppData) -> Element<'_, Message>
{
    let search_config = &app.config.search;
    let is_focused = !app.query.is_empty();

    let background = if is_focused 
    {
        search_config.focused_background_color.to_iced()
    } 
    else 
    {
        search_config.background_color.to_iced()
    };
    let border_color = if is_focused 
    {
        search_config.focused_border_color.to_iced()
    } 
    else 
    {
        search_config.border_color.to_iced()
    };

    let font = make_font(&search_config.font_weight, &search_config.font_style);
    let radius = search_config.border_radius;
    let submit_message = resolve_submit_message(app);

    let input = text_input(&search_config.placeholder, &app.query)
        .id("search_input")
        .on_input(Message::QueryChanged)
        .on_submit(submit_message.clone())
        .size(search_config.text_size)
        .padding(search_config.input_padding as f32)
        .font(font)
        .style(move |_theme, _status| iced::widget::text_input::Style 
        {
            background: iced::Background::Color(background),
            icon: search_config.placeholder_color.to_iced(),
            placeholder: search_config.placeholder_color.to_iced(),
            value: search_config.text_color.to_iced(),
            selection: search_config.selection_color.to_iced(),
            border: Border 
            {
                color: border_color,
                width: search_config.border_width,
                radius: corner_radius(radius),
            },
        });

    let search_config = &app.config.search;
    let element: Element<Message> = match search_config.orientation 
    {
        SearchOrientation::Vertical => 
        {
            build_vertical_search(app, background, border_color, submit_message)
        }
        SearchOrientation::Horizontal if !search_config.icon.is_empty() => 
        {
            build_horizontal_search_with_icon(app, input, background, border_color)
        }
        SearchOrientation::Horizontal => input.into(),
    };

    wrap_search_bar_margin(app, element)
}



fn resolve_submit_message(app: &AppData) -> Message
{
    match app.filtered.get(app.selected) 
    {
        Some(e) if e.exec.is_empty() => 
        {
            Message::CopyToClipboard(crate::helpers::update_helpers::calc_display_value(&e.name))
        }
        Some(e) => Message::Launch(e.exec.clone()),
        None => Message::Nothing,
    }
}



fn build_vertical_search<'a>(app: &'a AppData, background: iced::Color, border_color: iced::Color, submit_msg: Message) -> Element<'a, Message>
{
    let search_config = &app.config.search;
    let font = make_font(&search_config.font_weight, &search_config.font_style);
    let radius = search_config.border_radius;
    let display_text = if app.query.is_empty() { search_config.placeholder.clone() } else { app.query.clone() };

    let char_column: Element<Message> = 
    {
        let char_color = if app.query.is_empty() 
        {
            search_config.placeholder_color.to_iced()
        }
        else 
        {
            search_config.text_color.to_iced()
        };

        let chars: Vec<Element<Message>> = display_text
            .chars()
            .map(|c| 
            {
                container(text(c.to_string()).size(search_config.text_size).color(char_color).font(font))
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .into()
            })
            .collect();

        column(chars).spacing(2).width(Length::Fill).into()
    };

    let invisible_input = text_input("", &app.query)
        .id("search_input")
        .on_input(Message::QueryChanged)
        .on_submit(submit_msg)
        .size(search_config.text_size)
        .padding(0)
        .style(|_theme, _status| iced::widget::text_input::Style 
        {
            background: iced::Background::Color(Color::TRANSPARENT),
            icon: Color::TRANSPARENT,
            placeholder: Color::TRANSPARENT,
            value: Color::TRANSPARENT,
            selection: Color::TRANSPARENT,
            border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radius::default() },
        });

    let bar_width = if search_config.width > 0 { search_config.width as f32 } else { 48.0 };
    let inner = row!
    [
        container(invisible_input).width(1).height(Length::Fill),
        container(char_column)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .padding(search_config.input_padding as f32),
    ];

    container(inner)
        .width(bar_width)
        .height(Length::Fill)
        .style(move |_| container::Style 
        {
            background: Some(iced::Background::Color(background)),
            border: Border 
            {
                color: border_color,
                width: search_config.border_width,
                radius: corner_radius(radius),
            },
            ..Default::default()
        })
        .into()
}



fn build_horizontal_search_with_icon<'a>(app: &'a AppData, input: iced::widget::TextInput<'a, Message>, background: iced::Color, border_color: iced::Color) -> Element<'a, Message>
{
    let search_config = &app.config.search;
    let radius = search_config.border_radius;
    let icon_element: Element<Message> = text(search_config.icon.clone()).size(search_config.text_size).color(search_config.icon_color.to_iced()).into();
    let bar_w = optional_length(search_config.width);
    let bar_h = optional_length_shrink(search_config.height);

    container
    (
        row!
        [
            container(icon_element)
                .padding(Padding 
                {
                    top: 0.0,
                    right: 6.0,
                    bottom: 0.0,
                    left: search_config.input_padding as f32
                })
                .align_y(Alignment::Center),
            input,
        ]
        .align_y(Alignment::Center),
    )
    .width(bar_w)
    .height(bar_h)
    .style(move |_| container::Style 
    {
        background: Some(iced::Background::Color(background)),
        border: Border 
        {
            color: border_color,
            width: search_config.border_width,
            radius: corner_radius(radius),
        },
        ..Default::default()
    })
    .into()
}



fn wrap_search_bar_margin<'a>(app: &'a AppData, element: Element<'a, Message>) -> Element<'a, Message>
{
    let search_config = &app.config.search;
    let is_left = matches!(search_config.position, SearchPosition::Left | SearchPosition::TopLeft | SearchPosition::BottomLeft);
    let is_right = matches!(search_config.position, SearchPosition::Right | SearchPosition::TopRight | SearchPosition::BottomRight);
    let is_side = is_left || is_right;
    let margin = search_config.bottom_margin;
    let bar_w = optional_length(search_config.width);
    let bar_h = optional_length_shrink(search_config.height);

    let padding = if is_side 
    {
        if is_left 
        {
            Padding { top: 0.0, right: margin, bottom: 0.0, left: 0.0 }
        }
        else 
        {
            Padding { top: 0.0, right: 0.0, bottom: 0.0, left: margin }
        }
    } 
    else if search_config.position == SearchPosition::Bottom 
    {
        Padding { top: margin, right: 0.0, bottom: 0.0, left: 0.0 }
    } 
    else 
    {
        Padding { top: 0.0, right: 0.0, bottom: margin, left: 0.0 }
    };

    let bar: Element<Message> = container(element).padding(padding).width(bar_w).height(bar_h).into();
    let offset_x = search_config.fixed_x.unwrap_or(0.0);
    let offset_y = search_config.fixed_y.unwrap_or(0.0);

    if offset_x != 0.0 || offset_y != 0.0 
    {
        container(bar)
            .padding(Padding { top: offset_y, right: 0.0, bottom: 0.0, left: offset_x })
            .into()
    } 
    else 
    {
        bar
    }
}
