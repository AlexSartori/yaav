use iced::{Element, widget::{button, container, row}};
use iced_font_awesome::fa_icon_solid;

use crate::alignment_viewer::Message;


pub fn toolbar<'a>() -> Element<'a, Message> {
    container(
        row![
            button(
                fa_icon_solid("folder-open")
            ).on_press(Message::OpenFasta),
            button(
                fa_icon_solid("magnifying-glass-minus")
            ).on_press(Message::ZoomMinus),
            button(
                fa_icon_solid("magnifying-glass-plus")
            ).on_press(Message::ZoomPlus)
        ].spacing(10)
    )
    .width(iced::Length::Fill)
    .padding(6)
    .into()
}