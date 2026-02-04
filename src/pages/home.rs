use cosmic::{Element, fl, iced::{Alignment, Length}, widget};

use crate::app::{AppModel, Message};

pub type HomePage = AppModel;
impl HomePage {
	pub fn new(self: &AppModel, space_s: u16) -> Element<'static, Message> {
		let header = widget::row::with_capacity(2)
			.push(widget::text::title1(fl!("home")))
			.align_y(Alignment::End)
			.spacing(space_s);

		let vpn_status_label = ["VPN: ", if self.vpn_is_active { "Running" } else { "Stopped" }].concat();
		let section = cosmic::widget::settings::section().add(
			cosmic::widget::settings::item::builder(vpn_status_label)
				.control(widget::button::text(if self.vpn_is_active { "Stop" } else { "Start" }).on_press(Message::ToggleVPN)),
		);

		widget::column::with_capacity(2)
			.push(header)
			.push(section)
			.spacing(space_s)
			.height(Length::Fill)
			.into()
	}
}
