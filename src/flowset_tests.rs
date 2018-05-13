#[allow(unused_imports)]
use super::flowset::*;
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

    let res = DataFlow::from_bytes(&packet_bytes);
    assert!(res.is_ok());
}

// TODO: can use macro?
fn is_template(flowset: &FlowSet) -> bool {
    match flowset {
        &FlowSet::DataTemplate(_) => true,
        _ => false,
    }
}

fn is_option(flowset: &FlowSet) -> bool {
    match flowset {
        &FlowSet::OptionTemplate(_) => true,
        _ => false,
    }
}

fn is_dataflow(flowset: &FlowSet) -> bool {
    match flowset {
        &FlowSet::DataFlow(_) => true,
        _ => false,
    }
}

#[test]
fn test_netflow9() {
    let packet_bytes = &test_data::NETFLOWV9_DATA[..];
    let res = NetFlow9::from_bytes(&packet_bytes);
    assert!(res.is_ok());

    let netflow = res.unwrap();
    assert_eq!(netflow.version, 9);
    assert_eq!(netflow.count, 7);
    assert_eq!(netflow.sys_uptime, 5502099);
    assert_eq!(netflow.timestamp, 1523936618);
    assert_eq!(netflow.flow_sequence, 883);
    assert_eq!(netflow.flow_sets.len(), 7);
    assert!(is_template(&netflow.flow_sets[0]));
    assert!(is_template(&netflow.flow_sets[1]));
    assert!(is_template(&netflow.flow_sets[2]));
    assert!(is_template(&netflow.flow_sets[3]));
    assert!(is_option(&netflow.flow_sets[4]));
    assert!(is_dataflow(&netflow.flow_sets[5]));
    assert!(is_dataflow(&netflow.flow_sets[6]));
}
