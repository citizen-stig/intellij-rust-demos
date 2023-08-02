use intellij_demos_associated_type_defaults::MyCallMessage;

pub fn generate_call_messages(i: usize) -> Vec<MyCallMessage> {
    let mut result = vec![];
    for k in 0..i {
        let n = k as u8;
        result.push(MyCallMessage {
            data: vec![n, n + 1, n + 2],
        });
    }
    result
}