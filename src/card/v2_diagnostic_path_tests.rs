use super::*;

fn plain_text() -> Text {
    Text::plain("text")
}

fn callback() -> Behavior {
    Behavior::callback(JsonValue::from(serde_json::json!("ok")))
}

fn all_element_kinds() -> Vec<Element> {
    vec![
        Element::Div(Div::new(plain_text())),
        Element::Markdown(Markdown::new("markdown")),
        Element::Img(Image::new("img", plain_text())),
        Element::ImgCombination(ImageCombination::new(ImageCombinationMode::Double)),
        Element::Person(Person::new("user")),
        Element::PersonList(PersonList::new()),
        Element::Chart(Chart::new(serde_json::json!({}).into())),
        Element::Table(Table::new()),
        Element::Hr(Hr::new()),
        Element::ColumnSet(ColumnSet::new()),
        Element::CollapsiblePanel(CollapsiblePanel::new(CollapsiblePanelHeader::new(
            plain_text(),
        ))),
        Element::Form(Form::new("form")),
        Element::InteractiveContainer(InteractiveContainer::new(callback())),
        Element::Button(Button::new(plain_text())),
        Element::Input(Input::new("input")),
        Element::Overflow(Overflow::new()),
        Element::SelectStatic(StaticSelect::new()),
        Element::MultiSelectStatic(MultiStaticSelect::new()),
        Element::SelectPerson(PersonSelect::new()),
        Element::MultiSelectPerson(MultiPersonSelect::new()),
        Element::DatePicker(DatePicker::new()),
        Element::PickerTime(TimePicker::new()),
        Element::PickerDatetime(DatetimePicker::new()),
        Element::SelectImg(ImageSelect::new(vec![], callback())),
        Element::Checker(Checker::new()),
    ]
}

#[test]
fn diagnostic_metadata_covers_all_element_discriminators() {
    let tags = [
        "div",
        "markdown",
        "img",
        "img_combination",
        "person",
        "person_list",
        "chart",
        "table",
        "hr",
        "column_set",
        "collapsible_panel",
        "form",
        "interactive_container",
        "button",
        "input",
        "overflow",
        "select_static",
        "multi_select_static",
        "select_person",
        "multi_select_person",
        "date_picker",
        "picker_time",
        "picker_datetime",
        "select_img",
        "checker",
    ];
    let elements = all_element_kinds();
    assert_eq!(elements.len(), tags.len());
    for (element, tag) in elements.iter().zip(tags) {
        assert_eq!(element_wire_tag(element), tag);
    }

    for tag in ["multi_select_static", "multi_select_person"] {
        assert_eq!(initial_option_field(tag), "selected_values");
    }
    assert_eq!(initial_option_field("select_static"), "initial_option");
    assert_eq!(initial_picker_field("date_picker"), "initial_date");
    assert_eq!(initial_picker_field("picker_time"), "initial_time");
    assert_eq!(initial_picker_field("picker_datetime"), "initial_datetime");
    assert_eq!(initial_picker_field("unknown"), "value");

    let option_tags = [
        "overflow",
        "select_static",
        "multi_select_static",
        "select_person",
        "multi_select_person",
        "select_img",
    ];
    let behavior_tags = [
        "button",
        "input",
        "overflow",
        "select_static",
        "multi_select_static",
        "select_person",
        "multi_select_person",
        "date_picker",
        "picker_time",
        "picker_datetime",
        "select_img",
        "checker",
        "interactive_container",
    ];
    for element in &elements {
        let tag = element_wire_tag(element);
        assert_eq!(element_has_options(element), option_tags.contains(&tag));
        assert_eq!(element_has_behaviors(element), behavior_tags.contains(&tag));
    }
}

#[test]
fn diagnostic_paths_follow_nested_columns_and_containers() {
    let invalid_width = ValidationError::InvalidColumnWidth("15px".into());
    let invalid_weight = ValidationError::InvalidColumnWeight(0);
    let invalid_flex_mode = ValidationError::ColumnWidthRequiresFixedFlexMode;
    let elements = [Element::Form(
        Form::new("form").element(Element::ColumnSet(
            ColumnSet::new().column(
                Column::new().element(Element::ColumnSet(
                    ColumnSet::new()
                        .flex_mode(ColumnFlexMode::Stretch)
                        .column(Column::new().width(ColumnWidth::Pixels("15px".into())))
                        .column(Column::new().weight(0)),
                )),
            ),
        )),
    )];
    assert_eq!(
        find_column_path(&elements, &invalid_width, "/body/elements"),
        Some("/body/elements/0/elements/0/columns/0/elements/0/columns/0/width".into())
    );
    assert_eq!(
        find_column_path(&elements, &invalid_weight, "/body/elements"),
        Some("/body/elements/0/elements/0/columns/0/elements/0/columns/1/weight".into())
    );
    assert_eq!(
        find_column_set_flex_mode_path(&elements, "/body/elements"),
        Some("/body/elements/0/elements/0/columns/0/elements/0/flex_mode".into())
    );
    assert_eq!(
        find_element_error_path(&elements, &invalid_flex_mode, "/body/elements"),
        None
    );

    let nested = [Element::InteractiveContainer(
        InteractiveContainer::new(callback()).element(Element::Form(
            Form::new("form").element(Element::Div(Div::new(plain_text()))),
        )),
    )];
    assert_eq!(
        find_element_error_path(
            &nested,
            &ValidationError::InvalidDivWidth("bad".into()),
            "/body/elements",
        ),
        Some("/body/elements/0/elements/0/elements/0/width".into())
    );
    assert!(matches!(element_children(&nested[0]), Some(children) if children.len() == 1));
}

#[test]
fn diagnostic_field_mappings_are_stable_for_each_validation_family() {
    let cases = vec![
        (
            Element::Div(Div::new(plain_text())),
            ValidationError::InvalidDivWidth("bad".into()),
            "width",
        ),
        (
            Element::Img(Image::new("img", plain_text())),
            ValidationError::ImageSizeRequiresCropScale,
            "scale_type",
        ),
        (
            Element::Img(Image::new("img", plain_text())),
            ValidationError::InvalidCornerRadius("bad".into()),
            "corner_radius",
        ),
        (
            Element::ImgCombination(ImageCombination::new(ImageCombinationMode::Double)),
            ValidationError::TooManyImagesInCombination {
                mode: ImageCombinationMode::Double,
                count: 3,
            },
            "img_list",
        ),
        (
            Element::ImgCombination(ImageCombination::new(ImageCombinationMode::Double)),
            ValidationError::InvalidCornerRadius("bad".into()),
            "corner_radius",
        ),
        (
            Element::PersonList(PersonList::new()),
            ValidationError::InvalidPersonListLines,
            "lines",
        ),
        (
            Element::Chart(Chart::new(serde_json::json!({}).into())),
            ValidationError::InvalidChartSpec,
            "chart_spec",
        ),
        (
            Element::Chart(Chart::new(serde_json::json!({}).into())),
            ValidationError::InvalidChartHeight("bad".into()),
            "height",
        ),
        (
            Element::Table(Table::new()),
            ValidationError::EmptyTableColumns,
            "columns",
        ),
        (
            Element::Table(Table::new()),
            ValidationError::EmptyTableRows,
            "rows",
        ),
        (
            Element::Table(Table::new()),
            ValidationError::InvalidTablePageSize(0),
            "page_size",
        ),
        (
            Element::Table(Table::new()),
            ValidationError::InvalidTableRowHeight("bad".into()),
            "row_height",
        ),
        (
            Element::Table(Table::new()),
            ValidationError::InvalidTableRowMaxHeight("bad".into()),
            "row_max_height",
        ),
        (
            Element::Table(Table::new()),
            ValidationError::TableRowMaxHeightRequiresAutoRowHeight,
            "row_max_height",
        ),
        (
            Element::ColumnSet(ColumnSet::new()),
            ValidationError::EmptyColumnSet,
            "columns",
        ),
        (
            Element::Form(Form::new("form")),
            ValidationError::EmptyForm("form".into()),
            "name",
        ),
        (
            Element::Form(Form::new("form")),
            ValidationError::FormNestedOutsideBody,
            "tag",
        ),
        (
            Element::Form(Form::new("form")),
            ValidationError::MissingFormSubmit("form".into()),
            "elements",
        ),
        (
            Element::InteractiveContainer(InteractiveContainer::new(callback())),
            ValidationError::EmptyInteractiveContainer,
            "elements",
        ),
        (
            Element::InteractiveContainer(InteractiveContainer::new(callback())),
            ValidationError::MissingInteractiveContainerBehavior,
            "behaviors",
        ),
        (
            Element::InteractiveContainer(InteractiveContainer::new(callback())),
            ValidationError::InvalidInteractiveContainerWidth("bad".into()),
            "width",
        ),
        (
            Element::InteractiveContainer(InteractiveContainer::new(callback())),
            ValidationError::InvalidInteractiveContainerHeight("bad".into()),
            "height",
        ),
        (
            Element::Button(Button::new(plain_text())),
            ValidationError::ButtonTextRequiresPlainText,
            "text",
        ),
        (
            Element::Button(Button::new(plain_text())),
            ValidationError::MissingButtonBehavior,
            "behaviors",
        ),
        (
            Element::Button(Button::new(plain_text())),
            ValidationError::MissingFormButtonAction,
            "form_action_type",
        ),
        (
            Element::Button(Button::new(plain_text())),
            ValidationError::ButtonBehaviorConflict,
            "behaviors",
        ),
        (
            Element::Input(Input::new("input")),
            ValidationError::InvalidInputMaxLength(0),
            "max_length",
        ),
        (
            Element::Input(Input::new("input")),
            ValidationError::InvalidControlWidth("bad".into()),
            "width",
        ),
        (
            Element::SelectImg(ImageSelect::new(vec![], callback())),
            ValidationError::MissingImageSelectBehavior,
            "behaviors",
        ),
        (
            Element::SelectImg(ImageSelect::new(vec![], callback())),
            ValidationError::ImagePreviewOutsideForm,
            "can_preview",
        ),
        (
            Element::CollapsiblePanel(CollapsiblePanel::new(CollapsiblePanelHeader::new(
                plain_text(),
            ))),
            ValidationError::InvalidCornerRadius("bad".into()),
            "border/corner_radius",
        ),
        (
            Element::Table(Table::new()),
            ValidationError::TableNestedOutsideBody,
            "tag",
        ),
        (
            Element::SelectImg(ImageSelect::new(vec![], callback())),
            ValidationError::MultiSelectImageOutsideForm,
            "multi_select",
        ),
        (
            Element::Checker(Checker::new()),
            ValidationError::TooManyCheckerButtons(4),
            "button_area/buttons",
        ),
        (
            Element::Overflow(Overflow::new()),
            ValidationError::EmptyOptions("overflow"),
            "options",
        ),
        (
            Element::DatePicker(DatePicker::new()),
            ValidationError::MissingPickerValue("date_picker"),
            "placeholder",
        ),
        (
            Element::DatePicker(DatePicker::new()),
            ValidationError::MissingFormControlName("date_picker"),
            "name",
        ),
        (
            Element::MultiSelectStatic(MultiStaticSelect::new()),
            ValidationError::InvalidInitialOption("multi_select_static", "bad".into()),
            "selected_values",
        ),
        (
            Element::SelectStatic(StaticSelect::new()),
            ValidationError::InvalidInitialIndex(3),
            "initial_index",
        ),
        (
            Element::PickerDatetime(DatetimePicker::new()),
            ValidationError::InvalidPickerInitialValue("picker_datetime", "bad".into()),
            "initial_datetime",
        ),
        (
            Element::SelectPerson(PersonSelect::new()),
            ValidationError::DuplicateOptionValue("bad".into()),
            "options",
        ),
        (
            Element::Input(Input::new("input")),
            ValidationError::PlainTextRequired("input.placeholder"),
            "placeholder",
        ),
        (
            Element::Input(Input::new("input")),
            ValidationError::TextTooLong {
                field: "input.placeholder",
                length: 301,
            },
            "placeholder",
        ),
        (
            Element::Checker(Checker::new()),
            ValidationError::InvalidOpenUrl("bad".into()),
            "behaviors",
        ),
        (
            Element::Form(Form::new("form")),
            ValidationError::DuplicateFormControlName("name".into()),
            "elements",
        ),
        (
            Element::ColumnSet(ColumnSet::new()),
            ValidationError::TooDeeplyNestedContainer(6),
            "elements",
        ),
    ];
    for (element, error, field) in cases {
        assert_eq!(element_error_field(&element, &error), Some(field));
    }
}
