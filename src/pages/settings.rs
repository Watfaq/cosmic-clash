// SPDX-License-Identifier: AGPL3.0

use cosmic::{
	Element,
	iced::{Alignment, Length},
	widget,
};

use crate::{
	app::{AppModel, Message, SettingField},
	fl,
};

pub fn view_settings(app: &AppModel, space_s: u16) -> Element<'_, Message> {
	let header = widget::row::with_capacity(2)
		.push(widget::text::title1(fl!("settings")))
		.align_y(Alignment::End)
		.spacing(space_s);

	let mut main_column = widget::column::with_capacity(6)
		.push(header)
		.spacing(space_s * 2)
		.height(Length::Fill);

	// Application Settings Card
	let app_card = widget::container(
		widget::column::with_capacity(3)
			.push(
				widget::row::with_capacity(2)
					.push(widget::icon::from_name("settings").size(24))
					.push(widget::text::title3("Application Settings"))
					.spacing(space_s)
					.align_y(Alignment::Center),
			)
			.push(create_setting_row(
				"Clash Binary Path",
				app.config.clash_binary_path.as_deref().unwrap_or("Auto-detect"),
				Message::EditSetting(SettingField::BinaryPath),
				space_s,
			))
			.push(create_setting_row(
				"Config Directory",
				app.config.config_dir.as_deref().unwrap_or("Default directory"),
				Message::EditSetting(SettingField::ConfigDir),
				space_s,
			))
			.spacing(space_s + space_s / 2),
	)
	.padding(space_s * 2)
	.class(cosmic::style::Container::Card);

	main_column = main_column.push(app_card);

	// API Settings Card
	let api_card = widget::container(
		widget::column::with_capacity(3)
			.push(
				widget::row::with_capacity(2)
					.push(widget::icon::from_name("api").size(24))
					.push(widget::text::title3("API Settings"))
					.spacing(space_s)
					.align_y(Alignment::Center),
			)
			.push(create_setting_row(
				"API Port",
				&app.config.api_port.to_string(),
				Message::EditSetting(SettingField::ApiPort),
				space_s,
			))
			.push(create_setting_row(
				"API Secret",
				if app.config.api_secret.is_some() {
					"••••••••"
				} else {
					"Not set"
				},
				Message::EditSetting(SettingField::ApiSecret),
				space_s,
			))
			.spacing(space_s + space_s / 2),
	)
	.padding(space_s * 2)
	.class(cosmic::style::Container::Card);

	main_column = main_column.push(api_card);

	// Inline edit area (if editing)
	if let Some(field) = &app.editing_setting {
		let edit_card = widget::container(view_setting_editor(app, space_s, field))
			.padding(space_s * 2)
			.class(cosmic::style::Container::Card);

		main_column = main_column.push(edit_card);
	}

	main_column.into()
}

fn create_setting_row(label: &str, value: &str, action: Message, space_s: u16) -> Element<'static, Message> {
	let label_text = label.to_string();
	let value_text = value.to_string();

	widget::row::with_capacity(2)
		.push(
			widget::column::with_capacity(2)
				.push(widget::text::body(label_text))
				.push(widget::text::body(value_text))
				.spacing(space_s / 2)
				.width(Length::Fill),
		)
		.push(widget::button::text("EDIT").on_press(action).padding([4, 12]))
		.spacing(space_s)
		.width(Length::Fill)
		.align_y(Alignment::Center)
		.into()
}

fn view_setting_editor<'a>(app: &'a AppModel, space_s: u16, field: &'a SettingField) -> Element<'a, Message> {
	let (title, placeholder) = match field {
		SettingField::BinaryPath => ("Edit Binary Path", fl!("binary-path-placeholder")),
		SettingField::ConfigDir => ("Edit Config Directory", fl!("config-dir-placeholder")),
		SettingField::ApiPort => ("Edit API Port", fl!("api-port-placeholder")),
		SettingField::ApiSecret => ("Edit API Secret", fl!("api-secret-placeholder")),
	};

	let input = widget::text_input(placeholder, &app.edit_value)
		.on_input(Message::EditValueChanged)
		.on_submit(|_| Message::SaveSetting)
		.padding(space_s);

	widget::column::with_capacity(3)
		.push(
			widget::row::with_capacity(2)
				.push(widget::icon::from_name("edit").size(24))
				.push(widget::text::title3(title))
				.spacing(space_s)
				.align_y(Alignment::Center),
		)
		.push(input)
		.push(
			widget::row::with_capacity(2)
				.push(
					widget::button::text("SAVE")
						.on_press(Message::SaveSetting)
						.padding([space_s, space_s * 2]),
				)
				.push(
					widget::button::text("CANCEL")
						.on_press(Message::CancelEdit)
						.padding([space_s, space_s * 2]),
				)
				.spacing(space_s)
				.width(Length::Fill),
		)
		.spacing(space_s * 2)
		.into()
}
