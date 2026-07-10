use super::*;

#[test]
fn horizontal_data_list_exposes_root_item_label_and_value_geometry() {
    let rect = Rect::from_min_size(egui::pos2(10.0, 20.0), Vec2::new(320.0, 44.0));
    let items = [
        DataListItem::new("Status", "Blocked")
            .tone(DataListValueTone::Warning)
            .icon(RadixIcon::ExclamationTriangle),
        DataListItem::new("Evidence", "7 passed").tone(DataListValueTone::Positive),
    ];
    let output = primitive_data_list_root_output(
        rect,
        &items,
        DataListRootOptions::default()
            .size(DataListSize::One)
            .label_width(64.0)
            .show_dividers(true),
    );

    assert_eq!(output.role, "list");
    assert_eq!(output.data_orientation, "horizontal");
    assert_eq!(output.data_size, "1");
    assert_eq!(output.data_variant, "plain");
    assert_eq!(output.items.len(), 2);
    assert_eq!(output.items[0].role, "listitem");
    assert_eq!(output.items[0].label.rect.width(), 64.0);
    assert_eq!(output.items[0].value.data_tone, "warning");
    assert_eq!(
        output.items[0].value.icon.map(|icon| icon.icon),
        Some(RadixIcon::ExclamationTriangle)
    );
    assert!(output.items[0].divider_rect.is_none());
    assert!(output.items[1].divider_rect.is_some());
    assert_eq!(output.items[1].rect.bottom(), rect.bottom());
}

#[test]
fn data_list_visuals_support_surface_and_dark_light_value_tones() {
    let light = data_list_visual(DataListVariant::Surface, PrimitiveTheme::light());
    let dark = data_list_visual(DataListVariant::Surface, PrimitiveTheme::dark());
    assert_ne!(light.fill, dark.fill);
    assert_ne!(light.stroke, dark.stroke);
    assert_ne!(light.label, dark.label);
    assert_eq!(
        data_list_value_color(DataListValueTone::Neutral, PrimitiveTheme::light()),
        PrimitiveTheme::light().text
    );
    assert_ne!(
        data_list_value_color(DataListValueTone::Warning, PrimitiveTheme::light()),
        data_list_value_color(DataListValueTone::Warning, PrimitiveTheme::dark())
    );
}

#[test]
fn vertical_data_list_stacks_label_above_value() {
    let rect = Rect::from_min_size(egui::Pos2::ZERO, Vec2::new(240.0, 44.0));
    let output = primitive_data_list_root_output(
        rect,
        &[DataListItem::new("Case", "ITF26-B1")],
        DataListRootOptions::default()
            .orientation(DataListOrientation::Vertical)
            .size(DataListSize::Two),
    );
    let item = &output.items[0];

    assert_eq!(output.data_orientation, "vertical");
    assert!(item.label.rect.bottom() <= item.value.rect.top());
    assert_eq!(
        primitive_data_list_height(1, DataListOrientation::Vertical, DataListSize::Two),
        44.0
    );
}
