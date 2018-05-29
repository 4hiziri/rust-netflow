#[allow(unused_imports)]
use flowset::*;
use super::test_data;

#[test]
fn test_data_template() {
    let data_template_payload = &test_data::TEMPLATE_DATA[..];

    // parsing process test
    let template: Result<(&[u8], DataTemplate), ()> =
        DataTemplate::from_bytes(&data_template_payload);
    assert!(template.is_ok());

    // parsing result test
    let (_rest, template): (&[u8], DataTemplate) = template.unwrap();
    assert_eq!(template.flowset_id, 0);
    assert_eq!(template.length, 92);
    assert_eq!(template.template_id, 1024);
    assert_eq!(template.field_count, 21);
    // TODO: Field test
}

#[test]
fn test_option_template() {
    let packet_bytes = &test_data::OPTION_DATA[..];

    let option: Result<(&[u8], OptionTemplate), ()> = OptionTemplate::from_bytes(&packet_bytes);
    assert!(option.is_ok());

    let (_rest, option): (&[u8], OptionTemplate) = option.unwrap();
    assert_eq!(option.flowset_id, 1);
    assert_eq!(option.length, 26);
    assert_eq!(option.template_id, 4096);
    assert_eq!(option.option_scope_length, 4);
    assert_eq!(option.option_length, 12);
}

#[test]
fn test_data_flow() {
    let packet_bytes = &test_data::DATAFLOW_DATA[..];

    let res = DataFlow::from_bytes_notemplate(&packet_bytes);
    assert!(res.is_ok());
}

#[test]
fn test_dataflow_with_template() {
    let template = DataTemplate::from_bytes(&test_data::TEMPLATE_AND_DATA.0);
    assert!(template.is_ok());
    let template = template.unwrap().1;

    let dataflow = DataFlow::from_bytes(&test_data::TEMPLATE_AND_DATA.1, &[template]);
    assert!(dataflow.is_ok());
    let dataflow: DataFlow = dataflow.unwrap().1;

    assert!(dataflow.record_bytes.is_none());
    assert!(dataflow.records.is_some());
    assert_eq!(dataflow.flowset_id, 1024);
    assert_eq!(dataflow.length, 484);

    let records = dataflow.records.unwrap();
    assert_eq!(records.len(), 8);
}
