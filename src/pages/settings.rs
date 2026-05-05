// SPDX-License-Identifier: AGPL3.0

use cosmic::{
	Element,
	cosmic_theme,
	iced::{Alignment, Length},
	theme, widget,
};

use crate::{
	app::{AppModel, Message, SettingField},
	fl,
};

pub fn view_settings(app: &AppModel, _space_s: u16) -> Element<'_, Message> {
	let cosmic_theme::Spacing {
		space_xs,
		space_s,
		space_m,
		..
	} = theme::active().cosmic().spacing;

	let header = widget::row::with_capacity(2)
		.push(widget::text::title2(fl!("settings")))
		.align_y(Alignment::Center)
		.spacing(space_s);

	let app_section = widget::settings::section()
		.title(fl!("application-settings"))
		.add(setting_item(
			app,
			SettingField::BinaryPath,
			fl!("clash-binary"),
			app.config
				.clash_binary_path
				.clone()
				.unwrap_or_else(|| fl!("auto-detect")),
			space_xs,
			space_s,
		))
		.add(setting_item(
			app,
			SettingField::ConfigDir,
			fl!("config-directory"),
			app.config
				.config_dir
				.clone()
				.unwrap_or_else(|| fl!("default")),
			space_xs,
			space_s,
		));

	let api_section = widget::settings::section()
		.title(fl!("api-settings"))
		.add(setting_item(
			app,
			SettingField::ApiPort,
			fl!("api-port"),
			app.config.api_port.to_string(),
			space_xs,
			space_s,
		))
		.add(setting_item(
			app,
			SettingField::ApiSecret,
			fl!("api-secret"),
			if app.config.api_secret.is_some() {
				"••••••••".to_string()
			} else {
				fl!("none")
			},
			space_xs,
			space_s,
		));

	widget::settings::view_column(vec![
		header.into(),
		app_section.into(),
		api_section.into(),
	])
	.spacing(space_m)
	.width(Length::Fill)
	.into()
}

fn setting_item<'a>(
	app: &'a AppModel,
	field: SettingField,
	label: String,
	display_value: String,
	space_xs: u16,
	space_s: u16,
) -> Element<'a, Message> {
	let editing = matches!(app.editing_setting, Some(f) if discriminant_eq(f, field));

	if editing {
		let placeholder = match field {
			SettingField::BinaryPath => fl!("binary-path-placeholder"),
			SettingField::ConfigDir => fl!("config-dir-placeholder"),
			SettingField::ApiPort => fl!("api-port-placeholder"),
			SettingField::ApiSecret => fl!("api-secret-placeholder"),
		};

		let mut input = widget::text_input(placeholder, &app.edit_value)
			.on_input(Message::EditValueChanged)
			.on_submit(|_| Message::SaveSetting)
			.padding([space_xs, space_s])
			.width(Length::Fill);

		if matches!(field, SettingField::ApiSecret) {
			input = input.password();
		}

		let actions = widget::row::with_capacity(2)
			.push(widget::button::standard(fl!("cancel")).on_press(Message::CancelEdit))
			.push(widget::button::suggested(fl!("save")).on_press(Message::SaveSetting))
			.spacing(space_xs);

		let edit_row = widget::row::with_capacity(2)
			.push(input)
			.push(actions)
			.spacing(space_s)
			.align_y(Alignment::Center)
			.width(Length::Fill);

		return widget::settings::item::builder(label)
			.flex_control(edit_row)
			.into();
	}

	let edit_button = widget::button::text(fl!("edit")).on_press(Message::EditSetting(field));

	widget::settings::item::builder(label)
		.description(display_value)
		.control(edit_button)
		.into()
}

fn discriminant_eq(a: SettingField, b: SettingField) -> bool {
	std::mem::discriminant(&a) == std::mem::discriminant(&b)
}
