// SPDX-License-Identifier: AGPL3.0

use cosmic::{
	Element,
	cosmic_theme,
	iced::{Alignment, Background, Border, Length},
	theme, widget,
};

use crate::{
	app::{AppModel, Message},
	fl,
};

pub fn view_profile(app: &AppModel, _space_s: u16) -> Element<'_, Message> {
	let cosmic_theme::Spacing {
		space_xxs,
		space_xs,
		space_s,
		space_m,
		space_l,
		..
	} = theme::active().cosmic().spacing;

	let header = widget::row::with_capacity(3)
		.push(
			widget::column::with_capacity(2)
				.push(widget::text::title2(fl!("profile")))
				.spacing(space_xxs / 2),
		)
		.push(widget::space::horizontal().width(Length::Fill))
		.push(widget::button::standard(fl!("reload-config")).on_press(Message::ReloadConfig))
		.align_y(Alignment::Center)
		.spacing(space_s)
		.width(Length::Fill);

	let active_card = active_profile_card(app, space_xxs, space_xs, space_s, space_l);

	let mut layout = widget::column::with_capacity(3)
		.push(header)
		.push(active_card)
		.spacing(space_m)
		.width(Length::Fill);

	if app.profiles.is_empty() {
		layout = layout.push(empty_profiles(space_s, space_m, space_l));
	} else {
		layout = layout.push(profile_list(app, space_xxs, space_xs, space_s, space_l));
	}

	layout.into()
}

fn active_profile_card(
	app: &AppModel,
	_space_xxs: u16,
	_space_xs: u16,
	space_s: u16,
	space_l: u16,
) -> Element<'_, Message> {
	let active = app.config.active_profile.as_deref();
	let label = active
		.map(|s| s.to_string())
		.unwrap_or_else(|| fl!("no-profile"));

	let icon_name = if active.is_some() {
		"emblem-default-symbolic"
	} else {
		"dialog-question-symbolic"
	};

	let icon_disc = widget::container(widget::icon::from_name(icon_name).size(28))
		.padding(space_s)
		.class(theme::Container::custom(move |t| {
			let cosmic = t.cosmic();
			let bg = cosmic.accent_color();
			widget::container::Style {
				background: Some(Background::Color(bg.into())),
				icon_color: Some(cosmic.on_accent_color().into()),
				text_color: Some(cosmic.on_accent_color().into()),
				border: Border { radius: 32.0.into(), ..Default::default() },
				..Default::default()
			}
		}));

	let text_col = widget::column::with_capacity(2)
		.push(widget::text::caption(fl!("active-profile")))
		.push(widget::text::title3(label))
		.spacing(2);

	widget::container(
		widget::row::with_capacity(2)
			.push(icon_disc)
			.push(text_col)
			.spacing(space_s)
			.align_y(Alignment::Center)
			.width(Length::Fill),
	)
	.padding(space_l)
	.width(Length::Fill)
	.class(theme::Container::Card)
	.into()
}

fn empty_profiles(space_s: u16, space_m: u16, space_l: u16) -> Element<'static, Message> {
	let icon = widget::container(widget::icon::from_name("folder-symbolic").size(48))
		.padding(space_m)
		.class(theme::Container::custom(|t| {
			let cosmic = t.cosmic();
			widget::container::Style {
				background: Some(Background::Color(cosmic.bg_component_color().into())),
				icon_color: Some(cosmic.on_bg_component_color().into()),
				border: Border { radius: 64.0.into(), ..Default::default() },
				..Default::default()
			}
		}));

	let column = widget::column::with_capacity(4)
		.align_x(Alignment::Center)
		.spacing(space_s)
		.width(Length::Fill)
		.push(icon)
		.push(widget::text::title4(fl!("no-profiles-found")))
		.push(widget::text::body(fl!("no-profiles-description")));

	widget::container(column)
		.padding(space_l + space_m)
		.width(Length::Fill)
		.class(theme::Container::Card)
		.into()
}

fn profile_list<'a>(
	app: &'a AppModel,
	_space_xxs: u16,
	_space_xs: u16,
	space_s: u16,
	space_l: u16,
) -> Element<'a, Message> {
	let header = widget::row::with_capacity(3)
		.push(widget::icon::from_name("folder-open-symbolic").size(20))
		.push(widget::text::heading(fl!("available-profiles")))
		.push(widget::space::horizontal().width(Length::Fill))
		.push(widget::text::caption(format!("{}", app.profiles.len())))
		.spacing(space_s)
		.align_y(Alignment::Center)
		.width(Length::Fill);

	let mut section = widget::settings::section();

	for profile in &app.profiles {
		let is_active = app.config.active_profile.as_deref() == Some(profile.as_str());

		let indicator = widget::icon::from_name(if is_active {
			"emblem-default-symbolic"
		} else {
			"folder-symbolic"
		})
		.size(20);

		let item = widget::settings::item::builder(profile.clone()).icon(indicator);

		let row = if is_active {
			item.control(widget::text::caption(fl!("selected")))
		} else {
			item.control(
				widget::button::text(fl!("select"))
					.on_press(Message::SelectProfile(profile.clone())),
			)
		};

		section = section.add(row);
	}

	widget::container(
		widget::column::with_capacity(2)
			.push(header)
			.push(Element::from(section))
			.spacing(space_s)
			.width(Length::Fill),
	)
	.padding(space_l)
	.width(Length::Fill)
	.class(theme::Container::Card)
	.into()
}
