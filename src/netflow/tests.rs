use super::test_data;
use netflow::*;

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

// TODO: extract as combination test
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
