use crate::proto::UnitDesc;

pub trait Unit: Send {
    fn kind(&self) -> String;
    fn spawn(&self, uuid: String) -> Box<dyn UnitHandle>;
}

pub trait UnitHandle: Send {
    fn desc(&self) -> UnitDesc;
}
