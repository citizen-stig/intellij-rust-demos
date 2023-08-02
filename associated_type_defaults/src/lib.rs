#![feature(associated_type_defaults)]

#[derive(Debug, PartialEq)]
pub enum NonInstantiable {}

impl borsh::BorshDeserialize for NonInstantiable {
    fn deserialize_reader<R: std::io::Read>(_reader: &mut R) -> std::io::Result<Self> {
        unreachable!()
    }
}

impl borsh::BorshSerialize for NonInstantiable {
    fn serialize<W: std::io::Write>(&self, _writer: &mut W) -> std::io::Result<()> {
        unreachable!()
    }
}

pub trait Module {
    type CallMessage: std::fmt::Debug + borsh::BorshSerialize + borsh::BorshDeserialize = NonInstantiable;

    fn call(&mut self, _message: Self::CallMessage) -> Result<(), anyhow::Error> {
        unreachable!("call message is not instantiable")
    }
}

pub struct MyModule {
    storage: Vec<u8>,
}

#[derive(Debug, PartialEq, borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct MyCallMessage {
    pub data: Vec<u8>,
}

impl Module for MyModule {
    type CallMessage = MyCallMessage;

    fn call(&mut self, message: Self::CallMessage) -> Result<(), anyhow::Error> {
        println!("MyModule::call({:?})", message);
        self.storage.extend(&message.data);
        Ok(())
    }
}

#[test]
fn test_call() {
    let mut module = MyModule {
        storage: vec![],
    };

    let call_message = MyCallMessage {
        data: vec![1, 2, 3],
    };

    // TODO: This problem reported here https://github.com/intellij-rust/intellij-rust/issues/10704
    module.call(call_message).unwrap();
}
