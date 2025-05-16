use byte_slice::Bytes;
use strum_macros::AsRefStr;
pub mod data_link;
pub mod network;
pub mod transport;

#[derive(Debug, Default, AsRefStr)]
pub enum Layer<'a> {
    #[default]
    NoLayer,

    Data(Bytes<'a>),

    DataLinkLayer(data_link::DataLinkLayer<'a>),
    NetworkLayer(network::NetworkLayer<'a>),
    TransportLayer(transport::TransportLayer<'a>),
}

pub trait LayerTrait {
    fn next_layer(&self) -> &Layer;
}


#[derive(Debug, Default)]
pub struct Packet<'a> {
    pub layer: Layer<'a>
}