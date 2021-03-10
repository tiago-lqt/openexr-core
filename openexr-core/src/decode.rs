pub struct ChannelDecodeInfo<'a> {
    pub(crate) channel_info: openexr_sys::exr_channel_decode_info_t,
    pub(crate) _marker: std::marker::PhantomData<&'a ()>,
}

pub struct DecodeChunkInfo<'a> {
    pub(crate) chunk_info: openexr_sys::exr_decode_chunk_info__t,
    pub(crate) _marker: std::marker::PhantomData<&'a ()>,
}
