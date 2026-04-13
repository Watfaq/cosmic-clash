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

	let mut main_column = widget::column::with_capacity(4)
		.push(header)
		.spacing(space_s * 2)
		.height(Length::Fill);

	// Active Profile Card
	let active_profile = app.config.active_profile.as_deref().unwrap_or("No active profile");
	
	let active_card = widget::container(
		widget::column::with_capacity(3)
			.push(
				widget::row::with_capacity(3)
					.push(widget::icon::from_name("check-circle").size(24))
					.push(widget::text::title3("Active Profile"))
					.push(
						widget::container(
							widget::text::caption("ACTIVE")
						)
						.padding([2, 8])
					)
					.spacing(space_s)
					.align_y(Alignment::Center)
			)
			.push(
				widget::container(
					widget::text::title2(active_profile)
				)
				.padding([space_s, 0])
			)
			.push(
				widget::row::with_capacity(1)
					.push(
						widget::button::text("RELOAD CONFIGS")
							.on_press(Message::ReloadConfig)
							.padding([space_s, space_s * 2])
					)
					.width(Length::Fill)
			)
			.spacing(space_s * 2)
	)
	.padding(space_s * 2)
	.class(cosmic::style::Container::Card);

	main_column = main_column.push(active_card);

	// Profile List Card
	let profile_list_card = if app.profiles.is_empty() {
		// Empty state
		widget::container(
			widget::column::with_capacity(3)
				.push(widget::icon::from_name("folder-open").size(48))
				.push(widget::text::title3("No Profiles Found"))
				.push(
					widget::text::body("Add YAML configuration files to your config directory")
				)
				.spacing(space_s)
		)
		.padding(space_s * 3)
		.class(cosmic::style::Container::Card)
	} else {
		// Profile list
		let mut profiles_list = widget::column::with_capacity(app.profiles.len() + 1)
			.spacing(space_s);
		
		profiles_list = profiles_list.push(
			widget::text::title3("Available Profiles")
		);
		
		for profile in &app.profiles {
			let is_active = app.config.active_profile.as_deref() == Some(profile.as_str());
			
			let profile_item = widget::container(
				widget::row::with_capacity(3)
					.push(
						widget::icon::from_name(
							if is_active { "radio-button-checked" } else { "radio-button-unchecked" }
						).size(20)
					)
					.push(
						widget::text::body(profile)
					)
					.push(
						widget::button::text("SELECT")
							.on_press(Message::SelectProfile(profile.clone()))
							.padding([4, 12])
					)
					.spacing(space_s)
					.align_y(Alignment::Center)
					.width(Length::Fill)
			)
			.padding(space_s);
			
			profiles_list = profiles_list.push(profile_item);
		}
		
		widget::container(profiles_list)
			.padding(space_s * 2)
			.class(cosmic::style::Container::Card)
	};

	main_column = main_column.push(profile_list_card);

	main_column.into()
}