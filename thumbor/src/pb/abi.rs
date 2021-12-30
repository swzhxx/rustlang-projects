#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSpec {
    #[prost(message, repeated, tag = "1")]
    pub specs: ::prost::alloc::vec::Vec<Spec>,
}
