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

	// VPN Status Card
	let vpn_status_text = if app.vpn_is_active { "Running" } else { "Stopped" };
	let vpn_icon = if app.vpn_is_active { 
		"network-wireless"
	} else { 
		"network-wireless-off"
	};
	
	let status_header = widget::row::with_capacity(3)
		.push(widget::icon::from_name(vpn_icon).size(32))
		.push(widget::text::title2("VPN Status"))
		.push(widget::text::body(if app.vpn_is_active { "●" } else { "○" }))
		.spacing(space_s)
		.align_y(Alignment::Center);
	
	let status_details = widget::column::with_capacity(2)
		.push(
			widget::row::with_capacity(2)
				.push(widget::text::body("Status:"))
				.push(widget::text::body(vpn_status_text))
				.spacing(space_s / 2)
		)
		.spacing(space_s / 2);
	
	// Control button
	let control_button = widget::button::text(
		if app.vpn_is_active { "STOP VPN" } else { "START VPN" }
	)
	.on_press(Message::ToggleVPN)
	.padding([space_s, space_s * 2])
	.width(Length::Shrink);

	// Stats Card
	let mut stats_content = widget::column::with_capacity(3).spacing(space_s);
	
	// Version info
	if let Some(version) = &app.clash_version {
		stats_content = stats_content.push(
			widget::row::with_capacity(3)
				.push(widget::icon::from_name("info").size(20))
				.push(widget::text::body("Version:"))
				.push(widget::text::body(version))
				.spacing(space_s / 2)
				.align_y(Alignment::Center)
		);
	}

	// Traffic stats
	if let Some(traffic) = &app.traffic {
		let up_mb = traffic.up as f64 / 1_048_576.0;
		let down_mb = traffic.down as f64 / 1_048_576.0;
		
		stats_content = stats_content.push(
			widget::row::with_capacity(3)
				.push(widget::icon::from_name("arrow-upward").size(20))
				.push(widget::text::body("Upload:"))
				.push(widget::text::body(format!("{:.2} MB", up_mb)))
				.spacing(space_s / 2)
				.align_y(Alignment::Center)
		);
		
		stats_content = stats_content.push(
			widget::row::with_capacity(3)
				.push(widget::icon::from_name("arrow-downward").size(20))
				.push(widget::text::body("Download:"))
				.push(widget::text::body(format!("{:.2} MB", down_mb)))
				.spacing(space_s / 2)
				.align_y(Alignment::Center)
		);
	}

	let stats_card = if app.clash_version.is_some() || app.traffic.is_some() {
		Some(
			widget::container(
				widget::column::with_capacity(2)
					.push(widget::text::title3("Statistics"))
					.push(stats_content)
					.spacing(space_s)
			)
			.padding(space_s)
			.class(cosmic::style::Container::Card)
		)
	} else {
		None
	};

	// Main VPN card
	let vpn_card = widget::container(
		widget::column::with_capacity(3)
			.push(status_header)
			.push(status_details)
			.push(
				widget::row::with_capacity(1)
					.push(control_button)
					.width(Length::Fill)
			)
			.spacing(space_s * 2)
	)
	.padding(space_s * 2)
	.class(cosmic::style::Container::Card);

	// Main layout
	let mut main_column = widget::column::with_capacity(3)
		.push(header)
		.push(vpn_card)
		.spacing(space_s * 2);
	
	if let Some(stats_card) = stats_card {
		main_column = main_column.push(stats_card);
	}

	main_column
		.height(Length::Fill)
		.into()
}