use super::test_data;
#[allow(unused_imports)]
use flowset::*;

#[test]
fn test_dataflow_with_template() {
    let template = DataTemplate::from_bytes(&test_data::TEMPLATE_AND_DATA.0);
    assert!(template.is_ok());
    let template: DataTemplate = template.unwrap().1;

    let dataflow = DataFlow::from_bytes(&test_data::TEMPLATE_AND_DATA.1, &template.templates);
    assert!(dataflow.is_ok());
    let dataflow: DataFlow = dataflow.unwrap().1;

    assert!(dataflow.record_bytes.is_none());
    assert!(dataflow.records.is_some());
    assert_eq!(dataflow.flowset_id, 1024);
    assert_eq!(dataflow.length, 484);

    let records = dataflow.records.unwrap();
    assert_eq!(records.len(), 8);
}
