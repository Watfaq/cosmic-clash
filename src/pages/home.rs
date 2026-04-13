// SPDX-License-Identifier: AGPL3.0

use crate::app::{AppModel, Message};
use crate::fl;
use cosmic::{Element, iced::{Alignment, Length}, widget};

pub fn view_home(
	app: &AppModel,
	space_s: u16,
) -> Element<'_, Message> {
	let header = widget::row::with_capacity(2)
		.push(widget::text::title1(fl!("home")))
		.align_y(Alignment::End)
		.spacing(space_s);

	// VPN Status with visual indicator
	let vpn_status = if app.vpn_is_active { "Running" } else { "Stopped" };

	let vpn_icon = if app.vpn_is_active { "network-wireless-symbolic" } else { "network-wireless-offline-symbolic" };
	
	let status_text = if app.vpn_is_active { "●" } else { "○" };
	let status_indicator = widget::text::body(status_text);
	
	let status_row = widget::row::with_capacity(4)
		.push(widget::icon::from_name(vpn_icon).size(24))
		.push(widget::text::title3("VPN Status"))
		.push(status_indicator)
		.push(widget::text::body(vpn_status))
		.spacing(space_s)
		.align_y(Alignment::Center);
	
	let control_button = widget::button::text(if app.vpn_is_active { "Stop" } else { "Start" })
		.on_press(Message::ToggleVPN)
		.width(Length::Fixed(100.0));

	// Show version info if available
	let mut info_column = widget::column::with_capacity(2).spacing(space_s);
	
	if let Some(version) = &app.clash_version {
		let ver_text = version.as_str();
		info_column = info_column.push(
			widget::row::with_capacity(2)
				.push(widget::icon::from_name("dialog-information-symbolic").size(16))
				.push(widget::text::body(format!("{}: {}", fl!("clash-version"), ver_text)))
				.spacing(space_s / 2)
				.align_y(Alignment::Center)
		);
	}

	// Show traffic if available
	if let Some(traffic) = &app.traffic {
		let up_mb = traffic.up / 1_048_576;
		let down_mb = traffic.down / 1_048_576;
		
		info_column = info_column.push(
			widget::row::with_capacity(2)
				.push(widget::icon::from_name("go-up-symbolic").size(16))
				.push(widget::text::body(format!("Upload: {} MB", up_mb)))
				.spacing(space_s / 2)
				.align_y(Alignment::Center)
		);
		
		info_column = info_column.push(
			widget::row::with_capacity(2)
				.push(widget::icon::from_name("go-down-symbolic").size(16))
				.push(widget::text::body(format!("Download: {} MB", down_mb)))
				.spacing(space_s / 2)
				.align_y(Alignment::Center)
		);
	}

	let main_card = widget::container(
		widget::column::with_capacity(3)
			.push(status_row)
			.push(control_button)
			.push(info_column)
			.spacing(space_s)
	)
	.padding(space_s)
	.class(cosmic::style::Container::Card);

	widget::column::with_capacity(2)
		.push(header)
		.push(main_card)
		.spacing(space_s)
		.height(Length::Fill)
		.into()
}
