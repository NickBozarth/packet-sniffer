pub mod data_link;
pub mod network;



#[derive(Debug, Default)]
pub enum TansportLayer {
    #[default]
    UNDEFINED,

    TCP,
    UDP,
}