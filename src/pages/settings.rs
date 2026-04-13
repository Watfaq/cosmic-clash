// SPDX-License-Identifier: AGPL3.0

use crate::app::{AppModel, Message, SettingField};
use crate::fl;
use cosmic::{Element, iced::{Alignment, Length}, widget};

pub fn view_settings(
	app: &AppModel,
	space_s: u16,
) -> Element<'_, Message> {
	let header = widget::row::with_capacity(2)
		.push(widget::text::title1(fl!("settings")))
		.align_y(Alignment::End)
		.spacing(space_s);

	let mut column = widget::column::with_capacity(6)
		.push(header)
		.spacing(space_s)
		.height(Length::Fill);

	// Clash binary path
	let binary_path = app
		.config
		.clash_binary_path
		.as_deref()
		.unwrap_or("auto-detect")
		.to_string();
	column = column.push(
		cosmic::widget::settings::section().add(
			cosmic::widget::settings::item::builder(fl!("clash-binary"))
				.description(binary_path)
				.control(
					widget::button::text(fl!("edit"))
						.on_press(Message::EditSetting(SettingField::BinaryPath)),
				),
		),
	);

	// Config directory
	let config_dir = app
		.config
		.config_dir
		.as_deref()
		.unwrap_or("default")
		.to_string();
	column = column.push(
		cosmic::widget::settings::section().add(
			cosmic::widget::settings::item::builder(fl!("config-directory"))
				.description(config_dir)
				.control(
					widget::button::text(fl!("edit"))
						.on_press(Message::EditSetting(SettingField::ConfigDir)),
				),
		),
	);

	// API Port
	let port_text = app.config.api_port.to_string();
	column = column.push(
		cosmic::widget::settings::section().add(
			cosmic::widget::settings::item::builder(fl!("api-port"))
				.description(port_text)
				.control(
					widget::button::text(fl!("edit"))
						.on_press(Message::EditSetting(SettingField::ApiPort)),
				),
		),
	);

	// API Secret
	let secret_display = app
		.config
		.api_secret
		.as_ref()
		.map(|_| "********".to_string())
		.unwrap_or_else(|| fl!("none").to_string());
	column = column.push(
		cosmic::widget::settings::section().add(
			cosmic::widget::settings::item::builder(fl!("api-secret"))
				.description(secret_display)
				.control(
					widget::button::text(fl!("edit"))
						.on_press(Message::EditSetting(SettingField::ApiSecret)),
				),
		),
	);

	// Inline edit area
	if let Some(field) = &app.editing_setting {
		column = column.push(
			widget::container(view_setting_editor(app, space_s, field))
				.padding(space_s)
				.class(cosmic::style::Container::Card)
		);
	}

	column.into()
}

fn view_setting_editor<'a>(
	app: &'a AppModel,
	space_s: u16,
	field: &'a SettingField,
) -> Element<'a, Message> {
	let placeholder = match field {
		SettingField::BinaryPath => fl!("binary-path-placeholder"),
		SettingField::ConfigDir => fl!("config-dir-placeholder"),
		SettingField::ApiPort => fl!("api-port-placeholder"),
		SettingField::ApiSecret => fl!("api-secret-placeholder"),
	};

	let input = widget::text_input(placeholder, &app.edit_value)
		.on_input(Message::EditValueChanged)
		.on_submit(|_| Message::SaveSetting);

	widget::column::with_capacity(2)
		.push(input)
		.push(
			widget::row::with_capacity(2)
				.push(widget::button::text(fl!("save")).on_press(Message::SaveSetting))
				.push(widget::button::text(fl!("cancel")).on_press(Message::CancelEdit))
				.spacing(space_s),
		)
		.spacing(space_s)
		.into()
}
