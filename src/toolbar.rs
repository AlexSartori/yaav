use iced::{Element, widget::{button, combo_box, container, row}};
use iced_font_awesome::fa_icon_solid;

use crate::alignment_viewer::{AlignmentEvent, AlignmentState, Message};


pub fn toolbar(state: &AlignmentState) -> Element<'_, Message> {
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
            ).on_press(Message::ZoomPlus),
            combo_box(
                &state.display_combobox, "", Some(&state.display_encoding), |a| Message::AlignmentStateChanged(AlignmentEvent::EncodingChanged(a))
            ).width(200)
        ].spacing(10)
    )
    .width(iced::Length::Fill)
    .padding(6)
    .into()
}