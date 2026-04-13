// SPDX-License-Identifier: AGPL3.0

use crate::app::{AppModel, Message};
use crate::fl;
use cosmic::{Element, iced::{Alignment, Length}, widget};

pub fn view_profile(
	app: &AppModel,
	space_s: u16,
) -> Element<'_, Message> {
	let header = widget::row::with_capacity(2)
		.push(widget::text::title1(fl!("profile")))
		.align_y(Alignment::End)
		.spacing(space_s);

	let mut column = widget::column::with_capacity(4)
		.push(header)
		.spacing(space_s)
		.height(Length::Fill);

	// Active profile display
	let active = app.config.active_profile.as_deref().unwrap_or("no-profile");
	
	let active_profile_row = widget::row::with_capacity(3)
		.push(widget::icon::from_name("emblem-ok-symbolic").size(16))
		.push(widget::text::body(fl!("active-profile")))
		.push(widget::text::body(active))
		.spacing(space_s)
		.align_y(Alignment::Center);
	
	column = column.push(
		widget::container(active_profile_row)
			.padding(space_s)
			.class(cosmic::style::Container::Card)
	);

	// Reload button
	column = column.push(
		widget::button::text(fl!("reload-config"))
			.on_press(Message::ReloadConfig)
			.width(Length::Fill)
	);

	// Profile list
	if app.profiles.is_empty() {
		column = column.push(
			widget::container(
				widget::column::with_capacity(2)
					.push(widget::icon::from_name("dialog-warning-symbolic").size(32))
					.push(widget::text::body(fl!("no-profiles-found")))
					.spacing(space_s)
					.align_x(Alignment::Center)
			)
			.align_x(Alignment::Center)
			.align_y(Alignment::Center)
			.padding(space_s * 2)
		);
	} else {
		let mut profiles_list = widget::column::with_capacity(app.profiles.len())
			.spacing(space_s / 2);
		
		for profile in &app.profiles {
			let is_active = match &app.config.active_profile {
				Some(active) => active == profile,
				None => false,
			};
			let _icon_name = if is_active { "emblem-ok-symbolic" } else { "text-x-generic-symbolic" };
			

			
			let msg = Message::SelectProfile(profile.clone());
			profiles_list = profiles_list.push(
				widget::button::text(profile)
					.on_press(msg)
					.width(Length::Fill)
					.padding(space_s)
			);
		}
		
		column = column.push(
			widget::container(
				widget::column::with_capacity(2)
					.push(widget::text::heading(fl!("available-profiles")))
					.push(profiles_list)
					.spacing(space_s)
			)
			.padding(space_s)
			.class(cosmic::style::Container::Card)
		);
	}

	// Scan button
	column = column.push(
		widget::button::text(fl!("scan-profiles"))
			.on_press(Message::Nop)
			.width(Length::Fill)
	);

	column.into()
}
