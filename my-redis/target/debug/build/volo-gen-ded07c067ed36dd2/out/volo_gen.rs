pub mod volo_gen {
    #![allow(warnings, clippy::all)]

    pub mod my_redis {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct ItemServiceRedisCommandArgsSend {
            pub req: Item,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for ItemServiceRedisCommandArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceRedisCommandArgsSend",
                };

                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut req = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                req = Some(::pilota::thrift::Message::decode(protocol)?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!(
                                "decode struct `ItemServiceRedisCommandArgsSend` field(#{}) failed",
                                field_id
                            ),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;

                let Some(req) = req else {
                    return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };

                let data = Self { req };
                Ok(data)
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut req = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin().await?;
                if let Err(err) = async {
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                req =
                                    Some(::pilota::thrift::Message::decode_async(protocol).await?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }

                        protocol.read_field_end().await?;
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                }
                .await
                {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!(
                                "decode struct `ItemServiceRedisCommandArgsSend` field(#{}) failed",
                                field_id
                            ),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end().await?;

                let Some(req) = req else {
                    return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };

                let data = Self { req };
                Ok(data)
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceRedisCommandArgsSend",
                }) + protocol.struct_field_len(Some(1), &self.req)
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        impl ::std::convert::From<ItemType> for i32 {
            fn from(e: ItemType) -> Self {
                e as _
            }
        }

        impl ::std::convert::TryFrom<i32> for ItemType {
            type Error = ::pilota::EnumConvertError<i32>;

            #[allow(non_upper_case_globals)]
            fn try_from(v: i32) -> ::std::result::Result<Self, ::pilota::EnumConvertError<i32>> {
                const Get: i32 = ItemType::Get as i32;
                const Set: i32 = ItemType::Set as i32;
                const Del: i32 = ItemType::Del as i32;
                const Ping: i32 = ItemType::Ping as i32;
                const Subscribe: i32 = ItemType::Subscribe as i32;
                const Publish: i32 = ItemType::Publish as i32;
                match v {
                    Get => ::std::result::Result::Ok(ItemType::Get),
                    Set => ::std::result::Result::Ok(ItemType::Set),
                    Del => ::std::result::Result::Ok(ItemType::Del),
                    Ping => ::std::result::Result::Ok(ItemType::Ping),
                    Subscribe => ::std::result::Result::Ok(ItemType::Subscribe),
                    Publish => ::std::result::Result::Ok(ItemType::Publish),

                    _ => ::std::result::Result::Err(::pilota::EnumConvertError::InvalidNum(
                        v, "ItemType",
                    )),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq)]
        #[repr(i32)]
        #[derive(Copy)]
        pub enum ItemType {
            #[derivative(Default)]
            Get = 0,

            Set = 1,

            Del = 2,

            Ping = 3,

            Subscribe = 4,

            Publish = 5,
        }

        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for ItemType {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_i32(*self as i32)?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let value = protocol.read_i32()?;
                Ok(::std::convert::TryFrom::try_from(value).map_err(|err| {
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        format!("invalid enum value for ItemType, value: {}", value),
                    )
                })?)
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let value = protocol.read_i32().await?;
                Ok(::std::convert::TryFrom::try_from(value).map_err(|err| {
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        format!("invalid enum value for ItemType, value: {}", value),
                    )
                })?)
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.i32_len(*self as i32)
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Item {
            pub key: ::std::option::Option<::pilota::FastStr>,

            pub value: ::std::option::Option<::pilota::FastStr>,

            pub expire_time: ::std::option::Option<i32>,

            pub request_type: ItemType,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for Item {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Item" };

                protocol.write_struct_begin(&struct_ident)?;
                if let Some(value) = self.key.as_ref() {
                    protocol.write_faststr_field(1, (value).clone())?;
                }
                if let Some(value) = self.value.as_ref() {
                    protocol.write_faststr_field(2, (value).clone())?;
                }
                if let Some(value) = self.expire_time.as_ref() {
                    protocol.write_i32_field(3, *value)?;
                }
                protocol.write_i32_field(4, (*&self.request_type).into())?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut key = None;
                let mut value = None;
                let mut expire_time = None;
                let mut request_type = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                key = Some(protocol.read_faststr()?);
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                value = Some(protocol.read_faststr()?);
                            }
                            Some(3) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                expire_time = Some(protocol.read_i32()?);
                            }
                            Some(4) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                request_type = Some(::pilota::thrift::Message::decode(protocol)?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `Item` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;

                let Some(request_type) = request_type else {
                    return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "field request_type is required".to_string(),
                    ));
                };

                let data = Self {
                    key,
                    value,
                    expire_time,
                    request_type,
                };
                Ok(data)
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut key = None;
                let mut value = None;
                let mut expire_time = None;
                let mut request_type = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin().await?;
                if let Err(err) = async {
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                key = Some(protocol.read_faststr().await?);
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                value = Some(protocol.read_faststr().await?);
                            }
                            Some(3) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                expire_time = Some(protocol.read_i32().await?);
                            }
                            Some(4) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                request_type =
                                    Some(::pilota::thrift::Message::decode_async(protocol).await?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }

                        protocol.read_field_end().await?;
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                }
                .await
                {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `Item` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end().await?;

                let Some(request_type) = request_type else {
                    return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "field request_type is required".to_string(),
                    ));
                };

                let data = Self {
                    key,
                    value,
                    expire_time,
                    request_type,
                };
                Ok(data)
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Item" })
                    + self
                        .key
                        .as_ref()
                        .map_or(0, |value| protocol.faststr_field_len(Some(1), value))
                    + self
                        .value
                        .as_ref()
                        .map_or(0, |value| protocol.faststr_field_len(Some(2), value))
                    + self
                        .expire_time
                        .as_ref()
                        .map_or(0, |value| protocol.i32_field_len(Some(3), *value))
                    + protocol.i32_field_len(Some(4), (*&self.request_type).into())
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct ItemServiceRedisCommandArgsRecv {
            pub req: Item,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for ItemServiceRedisCommandArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceRedisCommandArgsRecv",
                };

                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut req = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                req = Some(::pilota::thrift::Message::decode(protocol)?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!(
                                "decode struct `ItemServiceRedisCommandArgsRecv` field(#{}) failed",
                                field_id
                            ),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;

                let Some(req) = req else {
                    return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };

                let data = Self { req };
                Ok(data)
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut req = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin().await?;
                if let Err(err) = async {
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                req =
                                    Some(::pilota::thrift::Message::decode_async(protocol).await?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }

                        protocol.read_field_end().await?;
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                }
                .await
                {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!(
                                "decode struct `ItemServiceRedisCommandArgsRecv` field(#{}) failed",
                                field_id
                            ),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end().await?;

                let Some(req) = req else {
                    return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };

                let data = Self { req };
                Ok(data)
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceRedisCommandArgsRecv",
                }) + protocol.struct_field_len(Some(1), &self.req)
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq)]

        pub enum ItemServiceRedisCommandResultSend {
            #[derivative(Default)]
            Ok(ItemResponse),
        }

        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for ItemServiceRedisCommandResultSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceRedisCommandResultSend",
                })?;
                match self {
                    ItemServiceRedisCommandResultSend::Ok(ref value) => {
                        protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let mut ret = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        protocol.field_stop_len();
                        break;
                    } else {
                        protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
                        Some(0) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                protocol.struct_len(&field_ident);
                                ret = Some(ItemServiceRedisCommandResultSend::Ok(field_ident));
                            } else {
                                return Err(::pilota::thrift::DecodeError::new(
                                    ::pilota::thrift::DecodeErrorKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                protocol.read_field_end()?;
                protocol.read_struct_end()?;
                if let Some(ret) = ret {
                    Ok(ret)
                } else {
                    Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut ret = None;
                protocol.read_struct_begin().await?;
                loop {
                    let field_ident = protocol.read_field_begin().await?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    } else {
                    }
                    match field_ident.id {
                        Some(0) => {
                            if ret.is_none() {
                                let field_ident =
                                    ::pilota::thrift::Message::decode_async(protocol).await?;

                                ret = Some(ItemServiceRedisCommandResultSend::Ok(field_ident));
                            } else {
                                return Err(::pilota::thrift::DecodeError::new(
                                    ::pilota::thrift::DecodeErrorKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        _ => {
                            protocol.skip(field_ident.field_type).await?;
                        }
                    }
                }
                protocol.read_field_end().await?;
                protocol.read_struct_end().await?;
                if let Some(ret) = ret {
                    Ok(ret)
                } else {
                    Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceRedisCommandResultSend",
                }) + match self {
                    ItemServiceRedisCommandResultSend::Ok(ref value) => {
                        protocol.struct_field_len(Some(0), value)
                    }
                } + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct ItemResponse {
            pub response_type: ResponseType,

            pub value: ::std::option::Option<::pilota::FastStr>,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for ItemResponse {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "ItemResponse",
                };

                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_i32_field(1, (*&self.response_type).into())?;
                if let Some(value) = self.value.as_ref() {
                    protocol.write_faststr_field(2, (value).clone())?;
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut response_type = None;
                let mut value = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                response_type = Some(::pilota::thrift::Message::decode(protocol)?);
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                value = Some(protocol.read_faststr()?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `ItemResponse` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;

                let Some(response_type) = response_type else {
                    return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "field response_type is required".to_string(),
                    ));
                };

                let data = Self {
                    response_type,
                    value,
                };
                Ok(data)
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut response_type = None;
                let mut value = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin().await?;
                if let Err(err) = async {
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                response_type =
                                    Some(::pilota::thrift::Message::decode_async(protocol).await?);
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                value = Some(protocol.read_faststr().await?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }

                        protocol.read_field_end().await?;
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                }
                .await
                {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `ItemResponse` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end().await?;

                let Some(response_type) = response_type else {
                    return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "field response_type is required".to_string(),
                    ));
                };

                let data = Self {
                    response_type,
                    value,
                };
                Ok(data)
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ItemResponse",
                }) + protocol.i32_field_len(Some(1), (*&self.response_type).into())
                    + self
                        .value
                        .as_ref()
                        .map_or(0, |value| protocol.faststr_field_len(Some(2), value))
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        impl ::std::convert::From<ResponseType> for i32 {
            fn from(e: ResponseType) -> Self {
                e as _
            }
        }

        impl ::std::convert::TryFrom<i32> for ResponseType {
            type Error = ::pilota::EnumConvertError<i32>;

            #[allow(non_upper_case_globals)]
            fn try_from(v: i32) -> ::std::result::Result<Self, ::pilota::EnumConvertError<i32>> {
                const Success: i32 = ResponseType::Success as i32;
                const Error: i32 = ResponseType::Error as i32;
                match v {
                    Success => ::std::result::Result::Ok(ResponseType::Success),
                    Error => ::std::result::Result::Ok(ResponseType::Error),

                    _ => ::std::result::Result::Err(::pilota::EnumConvertError::InvalidNum(
                        v,
                        "ResponseType",
                    )),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq)]
        #[repr(i32)]
        #[derive(Copy)]
        pub enum ResponseType {
            #[derivative(Default)]
            Success = 0,

            Error = 1,
        }

        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for ResponseType {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_i32(*self as i32)?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let value = protocol.read_i32()?;
                Ok(::std::convert::TryFrom::try_from(value).map_err(|err| {
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        format!("invalid enum value for ResponseType, value: {}", value),
                    )
                })?)
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let value = protocol.read_i32().await?;
                Ok(::std::convert::TryFrom::try_from(value).map_err(|err| {
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        format!("invalid enum value for ResponseType, value: {}", value),
                    )
                })?)
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.i32_len(*self as i32)
            }
        }
        #[::async_trait::async_trait]
        pub trait ItemService {
            async fn redis_command(
                &self,
                req: Item,
            ) -> ::core::result::Result<ItemResponse, ::volo_thrift::AnyhowError>;
        }
        pub struct ItemServiceServer<S> {
            inner: S, // handler
        }

        pub struct MkItemServiceGenericClient;

        pub type ItemServiceClient = ItemServiceGenericClient<
            ::volo::service::BoxCloneService<
                ::volo_thrift::context::ClientContext,
                ItemServiceRequestSend,
                ::std::option::Option<ItemServiceResponseRecv>,
                ::volo_thrift::Error,
            >,
        >;

        impl<S> ::volo::client::MkClient<::volo_thrift::Client<S>> for MkItemServiceGenericClient {
            type Target = ItemServiceGenericClient<S>;
            fn mk_client(&self, service: ::volo_thrift::Client<S>) -> Self::Target {
                ItemServiceGenericClient(service)
            }
        }

        #[derive(Clone)]
        pub struct ItemServiceGenericClient<S>(pub ::volo_thrift::Client<S>);

        pub struct ItemServiceOneShotClient<S>(pub ::volo_thrift::Client<S>);

        impl<
                S: ::volo::service::Service<
                        ::volo_thrift::context::ClientContext,
                        ItemServiceRequestSend,
                        Response = ::std::option::Option<ItemServiceResponseRecv>,
                        Error = ::volo_thrift::Error,
                    > + Send
                    + Sync
                    + 'static,
            > ItemServiceGenericClient<S>
        {
            pub fn with_callopt<
                Opt: ::volo::client::Apply<::volo_thrift::context::ClientContext>,
            >(
                self,
                opt: Opt,
            ) -> ItemServiceOneShotClient<::volo::client::WithOptService<S, Opt>> {
                ItemServiceOneShotClient(self.0.with_opt(opt))
            }

            pub async fn redis_command(
                &self,
                req: Item,
            ) -> ::std::result::Result<
                ItemResponse,
                ::volo_thrift::error::ResponseError<std::convert::Infallible>,
            > {
                let req =
                    ItemServiceRequestSend::RedisCommand(ItemServiceRedisCommandArgsSend { req });
                let mut cx = self.0.make_cx("RedisCommand", false);
                #[allow(unreachable_patterns)]
                let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
                    Some(ItemServiceResponseRecv::RedisCommand(
                        ItemServiceRedisCommandResultRecv::Ok(resp),
                    )) => Ok(resp),
                    None => unreachable!(),
                    _ => unreachable!(),
                };
                ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
                    let mut cache = cache.borrow_mut();
                    if cache.len() < cache.capacity() {
                        cache.push(cx);
                    }
                });
                resp
            }
        }

        impl<
                S: ::volo::client::OneShotService<
                        ::volo_thrift::context::ClientContext,
                        ItemServiceRequestSend,
                        Response = ::std::option::Option<ItemServiceResponseRecv>,
                        Error = ::volo_thrift::Error,
                    > + Send
                    + Sync
                    + 'static,
            > ItemServiceOneShotClient<S>
        {
            pub async fn redis_command(
                self,
                req: Item,
            ) -> ::std::result::Result<
                ItemResponse,
                ::volo_thrift::error::ResponseError<std::convert::Infallible>,
            > {
                let req =
                    ItemServiceRequestSend::RedisCommand(ItemServiceRedisCommandArgsSend { req });
                let mut cx = self.0.make_cx("RedisCommand", false);
                #[allow(unreachable_patterns)]
                let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
                    Some(ItemServiceResponseRecv::RedisCommand(
                        ItemServiceRedisCommandResultRecv::Ok(resp),
                    )) => Ok(resp),
                    None => unreachable!(),
                    _ => unreachable!(),
                };
                ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
                    let mut cache = cache.borrow_mut();
                    if cache.len() < cache.capacity() {
                        cache.push(cx);
                    }
                });
                resp
            }
        }

        pub struct ItemServiceClientBuilder {}

        impl ItemServiceClientBuilder {
            pub fn new(
                service_name: impl AsRef<str>,
            ) -> ::volo_thrift::client::ClientBuilder<
                ::volo::layer::Identity,
                ::volo::layer::Identity,
                MkItemServiceGenericClient,
                ItemServiceRequestSend,
                ItemServiceResponseRecv,
                ::volo::net::dial::DefaultMakeTransport,
                ::volo_thrift::codec::default::DefaultMakeCodec<
                    ::volo_thrift::codec::default::ttheader::MakeTTHeaderCodec<
                        ::volo_thrift::codec::default::framed::MakeFramedCodec<
                            ::volo_thrift::codec::default::thrift::MakeThriftCodec,
                        >,
                    >,
                >,
                ::volo::loadbalance::LbConfig<
                    ::volo::loadbalance::random::WeightedRandomBalance<()>,
                    ::volo::discovery::DummyDiscover,
                >,
            > {
                ::volo_thrift::client::ClientBuilder::new(service_name, MkItemServiceGenericClient)
            }
        }

        impl<S> ItemServiceServer<S>
        where
            S: ItemService + ::core::marker::Send + ::core::marker::Sync + 'static,
        {
            pub fn new(
                inner: S,
            ) -> ::volo_thrift::server::Server<
                Self,
                ::volo::layer::Identity,
                ItemServiceRequestRecv,
                ::volo_thrift::codec::default::DefaultMakeCodec<
                    ::volo_thrift::codec::default::ttheader::MakeTTHeaderCodec<
                        ::volo_thrift::codec::default::framed::MakeFramedCodec<
                            ::volo_thrift::codec::default::thrift::MakeThriftCodec,
                        >,
                    >,
                >,
                ::volo_thrift::tracing::DefaultProvider,
            > {
                ::volo_thrift::server::Server::new(Self { inner })
            }
        }

        impl<T>
            ::volo::service::Service<::volo_thrift::context::ServerContext, ItemServiceRequestRecv>
            for ItemServiceServer<T>
        where
            T: ItemService + Send + Sync + 'static,
        {
            type Response = ItemServiceResponseSend;
            type Error = ::anyhow::Error;

            type Future<'cx> = impl ::std::future::Future<Output = ::std::result::Result<Self::Response, Self::Error>>
                + 'cx;

            fn call<'cx, 's>(
                &'s self,
                _cx: &'cx mut ::volo_thrift::context::ServerContext,
                req: ItemServiceRequestRecv,
            ) -> Self::Future<'cx>
            where
                's: 'cx,
            {
                async move {
                    match req {
                        ItemServiceRequestRecv::RedisCommand(args) => {
                            Ok(ItemServiceResponseSend::RedisCommand(
                                match self.inner.redis_command(args.req).await {
                                    Ok(resp) => ItemServiceRedisCommandResultSend::Ok(resp),
                                    Err(err) => return Err(err),
                                },
                            ))
                        }
                    }
                }
            }
        }
        #[derive(Debug, Clone)]
        pub enum ItemServiceRequestRecv {
            RedisCommand(ItemServiceRedisCommandArgsRecv),
        }

        #[derive(Debug, Clone)]
        pub enum ItemServiceRequestSend {
            RedisCommand(ItemServiceRedisCommandArgsSend),
        }

        #[derive(Debug, Clone)]
        pub enum ItemServiceResponseRecv {
            RedisCommand(ItemServiceRedisCommandResultRecv),
        }

        #[derive(Debug, Clone)]
        pub enum ItemServiceResponseSend {
            RedisCommand(ItemServiceRedisCommandResultSend),
        }

        #[::async_trait::async_trait]
        impl ::volo_thrift::EntryMessage for ItemServiceRequestRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::core::result::Result<(), ::pilota::thrift::EncodeError> {
                match self {
                    Self::RedisCommand(value) => {
                        ::pilota::thrift::Message::encode(value, protocol).map_err(|err| err.into())
                    }
                }
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
                msg_ident: &::pilota::thrift::TMessageIdentifier,
            ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                Ok(match &*msg_ident.name {
                    "RedisCommand" => {
                        Self::RedisCommand(::pilota::thrift::Message::decode(protocol)?)
                    }
                    _ => {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                            format!("unknown method {}", msg_ident.name),
                        ));
                    }
                })
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
                msg_ident: &::pilota::thrift::TMessageIdentifier,
            ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                Ok(match &*msg_ident.name {
                    "RedisCommand" => {
                        Self::RedisCommand(::pilota::thrift::Message::decode_async(protocol).await?)
                    }
                    _ => {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                            format!("unknown method {}", msg_ident.name),
                        ));
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                match self {
                    Self::RedisCommand(value) => ::volo_thrift::Message::size(value, protocol),
                }
            }
        }

        #[::async_trait::async_trait]
        impl ::volo_thrift::EntryMessage for ItemServiceRequestSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::core::result::Result<(), ::pilota::thrift::EncodeError> {
                match self {
                    Self::RedisCommand(value) => {
                        ::pilota::thrift::Message::encode(value, protocol).map_err(|err| err.into())
                    }
                }
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
                msg_ident: &::pilota::thrift::TMessageIdentifier,
            ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                Ok(match &*msg_ident.name {
                    "RedisCommand" => {
                        Self::RedisCommand(::pilota::thrift::Message::decode(protocol)?)
                    }
                    _ => {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                            format!("unknown method {}", msg_ident.name),
                        ));
                    }
                })
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
                msg_ident: &::pilota::thrift::TMessageIdentifier,
            ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                Ok(match &*msg_ident.name {
                    "RedisCommand" => {
                        Self::RedisCommand(::pilota::thrift::Message::decode_async(protocol).await?)
                    }
                    _ => {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                            format!("unknown method {}", msg_ident.name),
                        ));
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                match self {
                    Self::RedisCommand(value) => ::volo_thrift::Message::size(value, protocol),
                }
            }
        }
        #[::async_trait::async_trait]
        impl ::volo_thrift::EntryMessage for ItemServiceResponseRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::core::result::Result<(), ::pilota::thrift::EncodeError> {
                match self {
                    Self::RedisCommand(value) => {
                        ::pilota::thrift::Message::encode(value, protocol).map_err(|err| err.into())
                    }
                }
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
                msg_ident: &::pilota::thrift::TMessageIdentifier,
            ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                Ok(match &*msg_ident.name {
                    "RedisCommand" => {
                        Self::RedisCommand(::pilota::thrift::Message::decode(protocol)?)
                    }
                    _ => {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                            format!("unknown method {}", msg_ident.name),
                        ));
                    }
                })
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
                msg_ident: &::pilota::thrift::TMessageIdentifier,
            ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                Ok(match &*msg_ident.name {
                    "RedisCommand" => {
                        Self::RedisCommand(::pilota::thrift::Message::decode_async(protocol).await?)
                    }
                    _ => {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                            format!("unknown method {}", msg_ident.name),
                        ));
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                match self {
                    Self::RedisCommand(value) => ::volo_thrift::Message::size(value, protocol),
                }
            }
        }

        #[::async_trait::async_trait]
        impl ::volo_thrift::EntryMessage for ItemServiceResponseSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::core::result::Result<(), ::pilota::thrift::EncodeError> {
                match self {
                    Self::RedisCommand(value) => {
                        ::pilota::thrift::Message::encode(value, protocol).map_err(|err| err.into())
                    }
                }
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
                msg_ident: &::pilota::thrift::TMessageIdentifier,
            ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                Ok(match &*msg_ident.name {
                    "RedisCommand" => {
                        Self::RedisCommand(::pilota::thrift::Message::decode(protocol)?)
                    }
                    _ => {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                            format!("unknown method {}", msg_ident.name),
                        ));
                    }
                })
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
                msg_ident: &::pilota::thrift::TMessageIdentifier,
            ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                Ok(match &*msg_ident.name {
                    "RedisCommand" => {
                        Self::RedisCommand(::pilota::thrift::Message::decode_async(protocol).await?)
                    }
                    _ => {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                            format!("unknown method {}", msg_ident.name),
                        ));
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                match self {
                    Self::RedisCommand(value) => ::volo_thrift::Message::size(value, protocol),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq)]

        pub enum ItemServiceRedisCommandResultRecv {
            #[derivative(Default)]
            Ok(ItemResponse),
        }

        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for ItemServiceRedisCommandResultRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceRedisCommandResultRecv",
                })?;
                match self {
                    ItemServiceRedisCommandResultRecv::Ok(ref value) => {
                        protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let mut ret = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        protocol.field_stop_len();
                        break;
                    } else {
                        protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
                        Some(0) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                protocol.struct_len(&field_ident);
                                ret = Some(ItemServiceRedisCommandResultRecv::Ok(field_ident));
                            } else {
                                return Err(::pilota::thrift::DecodeError::new(
                                    ::pilota::thrift::DecodeErrorKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                protocol.read_field_end()?;
                protocol.read_struct_end()?;
                if let Some(ret) = ret {
                    Ok(ret)
                } else {
                    Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut ret = None;
                protocol.read_struct_begin().await?;
                loop {
                    let field_ident = protocol.read_field_begin().await?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    } else {
                    }
                    match field_ident.id {
                        Some(0) => {
                            if ret.is_none() {
                                let field_ident =
                                    ::pilota::thrift::Message::decode_async(protocol).await?;

                                ret = Some(ItemServiceRedisCommandResultRecv::Ok(field_ident));
                            } else {
                                return Err(::pilota::thrift::DecodeError::new(
                                    ::pilota::thrift::DecodeErrorKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        _ => {
                            protocol.skip(field_ident.field_type).await?;
                        }
                    }
                }
                protocol.read_field_end().await?;
                protocol.read_struct_end().await?;
                if let Some(ret) = ret {
                    Ok(ret)
                } else {
                    Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceRedisCommandResultRecv",
                }) + match self {
                    ItemServiceRedisCommandResultRecv::Ok(ref value) => {
                        protocol.struct_field_len(Some(0), value)
                    }
                } + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
    }
}
