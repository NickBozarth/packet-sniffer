use byte_slice::{Bytes, SliceToUnsigned};
use anyhow::Result;


// https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml


#[derive(Debug, Default)]
pub enum TransportLayer<'a> {
    #[default]
    NULL,

    UndefinedData(&'a Bytes<'a>),
    ICMP(ICMP<'a>),
    TCP(TCP<'a>),
    UDP(UDP<'a>),
}

impl<'a> TransportLayer<'a> {
    pub fn from_data(layer_type: u8, bytes: &'a mut Bytes) -> Result<Self> {
        Ok(
            match layer_type {
                0x01 => Self::ICMP(ICMP::from_bytes(bytes)?),
                0x06 => Self::TCP(TCP::from_bytes(bytes)?),
                0x11 => Self::UDP(UDP::from_bytes(bytes)?),
                _ => Self::UndefinedData(bytes)
            }
        )
    }
}




#[derive(Debug, Default)]
pub enum MessageType {
    EchoReply             = 0,
    // covers many different types, 
    // 1 is just the first one
    #[default]
    Unassigned                      = 1,
    DesinationUnreachable           = 3,
    SourceQuench                    = 4,
    Redirect                        = 5,
    AlternateHostAddress            = 6,
    // Unassigned                   = 7           
    EchoRequest                     = 8,
    RouterAdvertisement             = 9,
    RouterSolicitation              = 10,
    TimeExceeded                    = 11,
    ParameterProblem                = 12,
    Timestamp                       = 13,
    TimestampReply                  = 14,
    InformationRequest              = 15,
    InformationReply                = 16,
    AddressMaskRequest              = 17,
    AddressMaskReply                = 18,
    Unassigned19                    = 19,
    // 20..=29
    Unassigned20                    = 20,
    Traceroute                      = 30,
    DatagramConversionError         = 31,
    MobileHostRedirect              = 32,
    WhereAreYou                     = 33,
    HereIAm                         = 34,
    MobileRegistrationRequest       = 35,
    MobileRegistrationReply         = 36,
    DomainNameRequest               = 37,
    DomainNameReply                 = 38,
    SkipAlgorithmDiscoveryProtocol  = 39,
    Photuris                        = 40,
    Experimental41                  = 41,
    ExtendedEchoRequest             = 42,
    ExtendedEchoReply               = 43,
    // Unassigned                   = 44..=252
    Experimental253                 = 253,
    Experimental254                 = 254,
    // Unassigned                   = 255
}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            0 =>         MessageType::EchoReply, 
            1..=2 =>     MessageType::Unassigned,
            3 =>         MessageType::DesinationUnreachable, 
            4 =>         MessageType::SourceQuench, 
            5 =>         MessageType::Redirect, 
            6 =>         MessageType::AlternateHostAddress,
            7 =>         MessageType::Unassigned,
            8 =>         MessageType::EchoRequest, 
            9 =>         MessageType::RouterAdvertisement,
            10 =>        MessageType::RouterSolicitation,
            11 =>        MessageType::TimeExceeded, 
            12 =>        MessageType::ParameterProblem, 
            13 =>        MessageType::Timestamp, 
            14 =>        MessageType::TimestampReply, 
            15 =>        MessageType::InformationRequest, 
            16 =>        MessageType::InformationReply, 
            17 =>        MessageType::AddressMaskRequest,
            18 =>        MessageType::AddressMaskReply,
            19 =>        MessageType::Unassigned19,
            20..=29 =>   MessageType::Unassigned20,
            30 =>        MessageType::Traceroute,
            31 =>        MessageType::DatagramConversionError,
            32 =>        MessageType::MobileHostRedirect,
            33 =>        MessageType::WhereAreYou,
            34 =>        MessageType::HereIAm,
            35 =>        MessageType::MobileRegistrationRequest,
            36 =>        MessageType::MobileRegistrationReply,
            37 =>        MessageType::DomainNameRequest,
            38 =>        MessageType::DomainNameReply,
            39 =>        MessageType::SkipAlgorithmDiscoveryProtocol,
            40 =>        MessageType::Photuris,
            41 =>        MessageType::Experimental41,
            42 =>        MessageType::ExtendedEchoRequest,
            43 =>        MessageType::ExtendedEchoReply,
            44..=252 =>  MessageType::Unassigned,
            253 =>       MessageType::Experimental253,
            254 =>       MessageType::Experimental254,
            255 =>       MessageType::Unassigned,
        }
    }
}



#[derive(Debug, Default)]
pub enum MessageCode {
    // used when a type with multiple codes is provided but code is not defined
    UndefinedCode,

    // used for most of the Unassigned Types
    #[default]
    Reserved,
    // 0
    EchoReply,
    // 1..=2
    // Reserved,
    // 3
    DestinationNetworkUnreachable,
    DestinationHostUnreachable,
    DestinationProtocolUnreachable,
    DestinationPortUnreachable,
    FragmentationIsNeededAndTheDfFlagSet,
    SourceRouteFailed,
    DestinationNetworkUnknown,
    DestinationHostUnknown,
    SourceHostIsolated,
    NetworkAdministrativelyProhibited,
    HostAdministrativelyProhibited,
    NetworkUnreachableForTos,
    HostUnreachableForTos,
    CommunicationAdministrativelyProhibited,
    HostPrecedenceViolation,
    PrecedenceCutoffInEffect,
    // 4
    SourceQuench,
    // 5
    RedirectTheDatagramForTheNetwork,
    RedirectTheDatagramForTheHost,
    RedirectTheDatagramForTheServiceAndNetwork,
    RedirectTheDatagramForTheServiceAndHost,
    // 6
    AlternateHostAddress,
    // 7
    // Reserved
    // 8
    EchoRequest,
    // 9
    RouterAdvertisement,
    // 10
    RouterDiscovery,
    // 11
    TimeToLiveExpiredInTransit,
    FragmentReassemblyTimeExceeded,
    // 12
    PointerIndicatesTheError,
    MissingARequiredOption,
    BadLength,
    // 13
    Timestamp,
    // 14
    TimestampReply,
    // 15
    InformationRequest,
    // 16
    InformationReply,
    // 17
    AddressMaskRequest,
    // 18
    AddressMaskReply,
    // 19
    ReservedForSecurity,
    // 20..=29
    ReservedForRobustnessExperiment,
    // 30
    // InformationRequest
    // 31
    DatagramConversionError,
    // 32
    MobileHostRedirect,
    // 33
    WhereAreYou,
    // 34
    HereIAm,
    // 35
    MobileRegistrationRequest,
    // 36
    MobileRegistrationReply,
    // 37
    DomainNameRequest,
    // 38
    DomainNameReply,
    // 39
    SkipAlgorithmDiscoveryProtocol,
    // 40
    Photuris,
    // 41
    IcmpForExperimentalMobilityProtocols,
    // 42
    RequestExtenddedEcho,
    // 43
    NoError,
    MalformedQuery,
    NoSuchInterface,
    NoSuchEntry,
    MultipleInterfacesSatisfyQuery,
    // 44..=252
    // Reserved,
    // 253
    Rfc362StyleExperiment1,
    // 254
    Rfc362StyleExperiment2,
    // 255
    // Reserved
}

macro_rules! code_0_defined {
    ($message_code_type:expr, $message_code:expr) => {
        match $message_code {
            0 => $message_code_type,
            _ => MessageCode::UndefinedCode,
        }
    };
}

impl MessageCode {
    fn from_type_and_code(message_type: &MessageType, message_code: u8) -> Self {
        match message_type {
            MessageType::EchoReply =>                       code_0_defined!(MessageCode::EchoReply, message_code),
            MessageType::Unassigned =>                      code_0_defined!(MessageCode::Reserved, message_code),
            MessageType::SourceQuench =>                    code_0_defined!(MessageCode::SourceQuench, message_code),
            MessageType::AlternateHostAddress =>            code_0_defined!(MessageCode::AlternateHostAddress, message_code),
            MessageType::EchoRequest =>                     code_0_defined!(MessageCode::EchoRequest, message_code),
            MessageType::RouterAdvertisement =>             code_0_defined!(MessageCode::RouterAdvertisement, message_code),
            MessageType::RouterSolicitation =>              code_0_defined!(MessageCode::RouterDiscovery, message_code),
            MessageType::Timestamp =>                       code_0_defined!(MessageCode::Timestamp, message_code),
            MessageType::TimestampReply =>                  code_0_defined!(MessageCode::TimestampReply, message_code),
            MessageType::InformationRequest =>              code_0_defined!(MessageCode::InformationRequest, message_code),
            MessageType::InformationReply =>                code_0_defined!(MessageCode::InformationReply, message_code),
            MessageType::AddressMaskRequest =>              code_0_defined!(MessageCode::AddressMaskRequest, message_code),
            MessageType::AddressMaskReply =>                code_0_defined!(MessageCode::AddressMaskReply, message_code),
            MessageType::Unassigned19 =>                    code_0_defined!(MessageCode::ReservedForSecurity, message_code),
            MessageType::Unassigned20 =>                    code_0_defined!(MessageCode::ReservedForRobustnessExperiment, message_code),
            MessageType::Traceroute =>                      code_0_defined!(MessageCode::InformationRequest, message_code),
            MessageType::DatagramConversionError =>         code_0_defined!(MessageCode::DatagramConversionError, message_code),
            MessageType::MobileHostRedirect =>              code_0_defined!(MessageCode::MobileHostRedirect, message_code),
            MessageType::WhereAreYou =>                     code_0_defined!(MessageCode::WhereAreYou, message_code),
            MessageType::HereIAm =>                         code_0_defined!(MessageCode::HereIAm, message_code),
            MessageType::MobileRegistrationRequest =>       code_0_defined!(MessageCode::MobileRegistrationRequest, message_code),
            MessageType::MobileRegistrationReply =>         code_0_defined!(MessageCode::MobileRegistrationReply, message_code),
            MessageType::DomainNameRequest =>               code_0_defined!(MessageCode::DomainNameRequest, message_code),
            MessageType::DomainNameReply =>                 code_0_defined!(MessageCode::DomainNameReply, message_code),
            MessageType::SkipAlgorithmDiscoveryProtocol =>  code_0_defined!(MessageCode::SkipAlgorithmDiscoveryProtocol, message_code),
            MessageType::Photuris =>                        code_0_defined!(MessageCode::Photuris, message_code),
            MessageType::Experimental41 =>                  code_0_defined!(MessageCode::IcmpForExperimentalMobilityProtocols, message_code),
            MessageType::Experimental253 =>                 code_0_defined!(MessageCode::Rfc362StyleExperiment1, message_code),
            MessageType::Experimental254 =>                 code_0_defined!(MessageCode::Rfc362StyleExperiment2, message_code),
            MessageType::ExtendedEchoRequest =>             code_0_defined!(MessageCode::RequestExtenddedEcho, message_code),

            MessageType::DesinationUnreachable => match message_code {
                        0 =>  MessageCode::DestinationNetworkUnreachable,
                        1 =>  MessageCode::DestinationHostUnreachable,
                        2 =>  MessageCode::DestinationProtocolUnreachable,
                        3 =>  MessageCode::DestinationPortUnreachable,
                        4 =>  MessageCode::FragmentationIsNeededAndTheDfFlagSet,
                        5 =>  MessageCode::SourceRouteFailed,
                        6 =>  MessageCode::DestinationNetworkUnknown,
                        7 =>  MessageCode::DestinationHostUnknown,
                        8 =>  MessageCode::SourceHostIsolated,
                        9 =>  MessageCode::NetworkAdministrativelyProhibited,
                        10 => MessageCode::HostAdministrativelyProhibited,
                        11 => MessageCode::NetworkUnreachableForTos,
                        12 => MessageCode::HostUnreachableForTos,
                        13 => MessageCode::CommunicationAdministrativelyProhibited,
                        14 => MessageCode::HostPrecedenceViolation,
                        15 => MessageCode::PrecedenceCutoffInEffect,
                        _ =>  MessageCode::UndefinedCode
            },
            MessageType::Redirect => match message_code {
                        0 => MessageCode::RedirectTheDatagramForTheNetwork,
                        1 => MessageCode::RedirectTheDatagramForTheHost,
                        2 => MessageCode::RedirectTheDatagramForTheServiceAndNetwork,
                        3 => MessageCode::RedirectTheDatagramForTheServiceAndHost,
                        _ => MessageCode::UndefinedCode
            },
            MessageType::TimeExceeded => match message_code {
                        0 => MessageCode::TimeToLiveExpiredInTransit,
                        1 => MessageCode::FragmentReassemblyTimeExceeded,
                        _ => MessageCode::UndefinedCode
            }
            MessageType::ParameterProblem => match message_code {
                        0 => MessageCode::PointerIndicatesTheError,
                        1 => MessageCode::MissingARequiredOption,
                        2 => MessageCode::BadLength,
                        _ => MessageCode::UndefinedCode
            },
            MessageType::ExtendedEchoReply => match message_code {
                        0 =>  MessageCode::NoError,
                        1 =>  MessageCode::MalformedQuery,
                        2 =>  MessageCode::NoSuchInterface,
                        3 =>  MessageCode::NoSuchEntry,
                        4 =>  MessageCode::MultipleInterfacesSatisfyQuery,
                        _ =>  MessageCode::UndefinedCode
            }
        }
    }
}





#[derive(Debug, Default)]
pub struct ICMP<'a> {
    pub message_type: MessageType,
    pub code: MessageCode,
    pub checksum: u16,


    pub payload: &'a Bytes<'a>
}

impl<'a> ICMP<'a> {
    fn from_bytes(bytes: &'a mut Bytes) -> Result<Self> {
        let message_type = MessageType::from(bytes[0]);
        let code = MessageCode::from_type_and_code(&message_type, bytes[1]);
        let checksum = 0;

        Ok(
            Self {
                message_type,
                code,
                checksum,

                payload: bytes
            }
        )
    }
}





#[derive(Debug, Default)]
pub struct TCP<'a> {
    pub port_src: u16,
    pub port_dst: u16,
    pub sequence_num: u32,
    pub acknowledgement_num: u32,
    pub data_offset: u8, // 4 bit // number of 32 bit words in the header //  data_offset / 4 = header_len (in bytes)
    pub reserved: u8, // 3 bit // ALWAYS 0
    pub control_bits: u8, // 6 bits
    pub window: u16,
    pub checksum: u16,
    pub urgent_ptr: u16,
    // options


    pub payload: &'a Bytes<'a>
}

impl<'a> TCP<'a> {
    pub fn from_bytes(bytes: &'a mut Bytes) -> Result<Self> {

        let port_src = bytes[0..2].to_u16();
        let port_dst = bytes[2..4].to_u16();
        let sequence_num = bytes[4..8].to_u32();
        let acknowledgement_num = bytes[8..12].to_u32();
        let data_offset = bytes[12] >> 4;
        let reserved = 0;
        let control_bits = (bytes[12..14].to_u16() & 0x3f) as u8;
        let window = bytes[14..16].to_u16();
        let checksum = bytes[16..18].to_u16();
        let urgent_ptr = bytes[18..20].to_u16();


        Ok(
            Self {
                port_src,
                port_dst,
                sequence_num,
                acknowledgement_num,
                data_offset,
                reserved,
                control_bits,
                window,
                checksum,
                urgent_ptr,
                payload: bytes,

                ..Default::default()
            }
        )
    }
}




#[derive(Debug, Default)]
pub struct UDP<'a> {
    pub port_src: u16,
    pub port_dst: u16,
    pub length: u16,
    pub checksum: u16,

    pub payload: &'a Bytes<'a>,
}

impl<'a> UDP<'a> {
    fn from_bytes(bytes: &'a mut Bytes) -> Result<Self> {
        let port_src = bytes[0..2].to_u16();
        let port_dst = bytes[2..4].to_u16();
        let length = bytes[4..6].to_u16();
        let checksum = bytes[6..8].to_u16();

        bytes.shift_first(8)?;

        Ok(
            Self{
                port_src,
                port_dst,
                length,
                checksum,

                payload: bytes
            }
        )
    }
}