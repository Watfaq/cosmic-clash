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

pub fn view_home(app: &AppModel, _space_s: u16) -> Element<'_, Message> {
	let cosmic_theme::Spacing {
		space_xxxs,
		space_xxs,
		space_xs,
		space_s,
		space_m,
		space_l,
		..
	} = theme::active().cosmic().spacing;

	if !app.vpn_is_active {
		return empty_state(space_xxs, space_xs, space_s, space_m, space_l);
	}

	let active_card = active_status_card(app, space_xxxs, space_xxs, space_xs, space_s, space_m, space_l);

	let mut layout = widget::column::with_capacity(2)
		.push(active_card)
		.spacing(space_m)
		.width(Length::Fill);

	if let Some(traffic) = &app.traffic {
		layout = layout.push(stats_card(traffic, space_xxs, space_xs, space_s, space_m, space_l));
	}

	layout.into()
}

fn empty_state(_space_xxs: u16, _space_xs: u16, space_s: u16, space_m: u16, _space_l: u16) -> Element<'static, Message> {
	let icon = widget::container(
		widget::icon::from_name("network-wireless-disabled-symbolic")
			.size(64),
	)
	.padding(space_m)
	.class(theme::Container::custom(|t| {
		let cosmic = t.cosmic();
		widget::container::Style {
			background: Some(Background::Color(cosmic.bg_component_color().into())),
			text_color: Some(cosmic.on_bg_component_color().into()),
			icon_color: Some(cosmic.accent_color().into()),
			border: Border { radius: 96.0.into(), ..Default::default() },
			..Default::default()
		}
	}));

	let start_button = widget::button::suggested(fl!("start-vpn")).on_press(Message::ToggleVPN);

	let column = widget::column::with_capacity(5)
		.align_x(Alignment::Center)
		.spacing(space_m)
		.width(Length::Fill)
		.height(Length::Fill)
		.push(widget::space::vertical().height(Length::Fill))
		.push(icon)
		.push(
			widget::column::with_capacity(2)
				.align_x(Alignment::Center)
				.spacing(space_s)
				.push(widget::text::title2(fl!("no-vpn-running")))
				.push(widget::text::body(fl!("no-vpn-running-description"))),
		)
		.push(start_button)
		.push(widget::space::vertical().height(Length::Fill));

	widget::container(column)
		.width(Length::Fill)
		.height(Length::Fill)
		.into()
}

fn active_status_card(
	app: &AppModel,
	_space_xxxs: u16,
	space_xxs: u16,
	space_xs: u16,
	space_s: u16,
	_space_m: u16,
	space_l: u16,
) -> Element<'_, Message> {
	let dot = widget::container(widget::text(""))
		.width(Length::Fixed(10.0))
		.height(Length::Fixed(10.0))
		.class(theme::Container::custom(|t| {
			let cosmic = t.cosmic();
			widget::container::Style {
				background: Some(Background::Color(cosmic.success_color().into())),
				border: Border { radius: 5.0.into(), ..Default::default() },
				..Default::default()
			}
		}));

	let header = widget::row::with_capacity(4)
		.push(widget::icon::from_name("network-wireless-symbolic").size(28))
		.push(widget::text::title3(fl!("vpn-status")))
		.push(widget::space::horizontal().width(Length::Fill))
		.push(
			widget::row::with_capacity(2)
				.push(dot)
				.push(widget::text::body(fl!("vpn-running")))
				.spacing(space_xs)
				.align_y(Alignment::Center),
		)
		.spacing(space_s)
		.align_y(Alignment::Center)
		.width(Length::Fill);

	let mut details = widget::column::with_capacity(2).spacing(space_xs).width(Length::Fill);

	if let Some(version) = &app.clash_version {
		details = details.push(meta_row(fl!("clash-version"), version.clone(), space_xxs));
	}

	let footer = widget::row::with_capacity(2)
		.push(widget::space::horizontal().width(Length::Fill))
		.push(widget::button::destructive(fl!("stop-vpn")).on_press(Message::ToggleVPN))
		.width(Length::Fill);

	widget::container(
		widget::column::with_capacity(3)
			.push(header)
			.push(widget::divider::horizontal::default())
			.push(details)
			.push(footer)
			.spacing(space_s)
			.width(Length::Fill),
	)
	.padding(space_l)
	.width(Length::Fill)
	.class(theme::Container::Card)
	.into()
}

fn meta_row(label: String, value: String, _spacing: u16) -> Element<'static, Message> {
	widget::row::with_capacity(3)
		.push(widget::text::caption(label))
		.push(widget::space::horizontal().width(Length::Fill))
		.push(widget::text::body(value))
		.align_y(Alignment::Center)
		.width(Length::Fill)
		.into()
}

fn stats_card(
	traffic: &crate::api::Traffic,
	_space_xxs: u16,
	space_xs: u16,
	space_s: u16,
	space_m: u16,
	space_l: u16,
) -> Element<'static, Message> {
	let format_bytes = |bytes: u64| -> String {
		let kb = bytes as f64 / 1024.0;
		if kb < 1024.0 {
			format!("{:.1} KB/s", kb)
		} else {
			format!("{:.2} MB/s", kb / 1024.0)
		}
	};

	let header = widget::row::with_capacity(2)
		.push(widget::icon::from_name("utilities-system-monitor-symbolic").size(20))
		.push(widget::text::heading(fl!("statistics")))
		.spacing(space_s)
		.align_y(Alignment::Center);

	let stat_tile = |icon: &'static str, label: String, value: String| -> Element<'static, Message> {
		widget::container(
			widget::column::with_capacity(3)
				.push(
					widget::row::with_capacity(2)
						.push(widget::icon::from_name(icon).size(16))
						.push(widget::text::caption(label))
						.spacing(space_xs)
						.align_y(Alignment::Center),
				)
				.push(widget::text::title3(value))
				.spacing(space_xs)
				.width(Length::Fill),
		)
		.padding(space_m)
		.width(Length::Fill)
		.class(theme::Container::custom(|t| {
			let cosmic = t.cosmic();
			widget::container::Style {
				background: Some(Background::Color(cosmic.bg_component_color().into())),
				text_color: Some(cosmic.on_bg_component_color().into()),
				icon_color: Some(cosmic.accent_color().into()),
				border: Border {
					radius: 12.0.into(),
					..Default::default()
				},
				..Default::default()
			}
		}))
		.into()
	};

	let tiles = widget::row::with_capacity(2)
		.push(stat_tile(
			"go-up-symbolic",
			fl!("upload"),
			format_bytes(traffic.up),
		))
		.push(stat_tile(
			"go-down-symbolic",
			fl!("download"),
			format_bytes(traffic.down),
		))
		.spacing(space_s)
		.width(Length::Fill);

	widget::container(
		widget::column::with_capacity(2)
			.push(header)
			.push(tiles)
			.spacing(space_s)
			.width(Length::Fill),
	)
	.padding(space_l)
	.width(Length::Fill)
	.class(theme::Container::Card)
	.into()
}
