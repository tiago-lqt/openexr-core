use crate::{DecodeChunkInfo, ExrError, ExrFile};

impl ExrFile {
    /** returns the maximum unpacked size of a chunk for the file part.
     *
     * This may be used ahead of any actual reading of data, so can be
     * used to pre-allocate buffers for multiple threads in one block or
     * whatever your application may require.
     */
    fn exr_get_chunk_unpacked_size(&self, part_index: i32) -> u64 {
        unsafe { openexr_sys::exr_get_chunk_unpacked_size(self.file, part_index) }
    }

    /** @brief Computes the chunk_info for a particular scanline.
     *
     * Note that this may read data from the file to query the compressed
     * size of the data when the file is compressed or a deep scanline
     * file, so is not always a cheap operation, so should only be called
     * once per chunk request if possible (although this read must happen
     * at some point).
     *
     * Of further note is that the resulting number of scanlines and size
     * depends on the compression style used, and so may return a range of
     * scanlines that will be returned
     *
     * @param file File to be decoded
     * @param part_index Part index within the file
     * @param outinfo Structure to be initialized and filled in
     * @param y Scanline requested
     * @param own_scratch_space Sets the flag to indicate that the library
     *                          should allocate any necessary internal
     *                          scratch space. When 0, the application must
     *                          provide memory, or only request packed
     *                          buffers
     *
     * @return 0 upon success, otherwise an appropriate error code
     */
    fn exr_decode_chunk_init_scanline<'a>(
        &'a self,
        part_index: i32,
        y: i32,
        own_scratch_space: bool,
    ) -> Result<DecodeChunkInfo<'a>, ExrError> {
        unsafe {
            let mut chunk_info: openexr_sys::exr_decode_chunk_info_t = std::mem::zeroed();
            
            let result = openexr_sys::exr_decode_chunk_init_scanline(self.file, part_index, &mut chunk_info, y, own_scratch_space  as i32);

            if result == 0 {
                Ok(DecodeChunkInfo {
                    chunk_info,
                    _marker: Default::default()
                })
            }
            else {
                Err(result.into())
            }
        }
    }

    /** @brief Initializes the chunk_info for a particular tile.
     *
     * Note that this may read data from the file to query the compressed
     * size of the data when the file is compressed or a deep scanline
     * file, so is not always a cheap operation, so should only be called
     * once per chunk / tile if possible.
     *
     * @param file File to be decoded
     * @param part_index Part index within the file
     * @param outinfo Structure to be initialized and filled in
     * @param tilex x position in tile space (i.e. tile counts, not pixel)
     * @param tilex y position in tile space (i.e. tile counts, not pixel)
     * @param levelx x mip / rip level to read
     *               (must be 0 for single-level tile images)
     * @param levely y mip / rip level to read
     *               (must be 0 for single-level tile images, must be same as
     *                levelx for mip level files)
     * @param own_scratch_space Sets the flag to indicate that the library
     *                          should allocate any necessary internal
     *                          scratch space. When 0, the application must
     *                          provide memory, or only request packed
     *                          buffers
     *
     * @return 0 upon success, otherwise an appropriate error code
     */
    fn exr_decode_chunk_init_tile<'a>(
        &'a self,
        part_index: i32,
        tilex: i32,
        tiley: i32,
        levelx: i32,
        levely: i32,
        own_scratch_space: bool,
    ) -> Result<DecodeChunkInfo<'a>, ExrError> {
        unsafe {
            let mut chunk_info: openexr_sys::exr_decode_chunk_info_t = std::mem::zeroed();
            
            let result = openexr_sys::exr_decode_chunk_init_tile(self.file, part_index, &mut chunk_info, tilex, tiley, levelx, levely, own_scratch_space  as i32);

            if result == 0 {
                Ok(DecodeChunkInfo {
                    chunk_info,
                    _marker: Default::default()
                })
            }
            else {
                Err(result.into())
            }
        }
    }

    /** @brief Read a chunk from a file.
     *
     * This is only valid on a file that has been opened for read. This is
     * the only access mechanism for reading data from the file. Do note
     * that this is only provided on a per-chunk basis. The C++ layer has
     * buffering and other conveniences to read the entire image in a
     * single call, or just a single scanline at a time.
     *
     * Steps performed:
     * 1. Read the (compressed) data into the compressed scratch space,
     *    allocating first if own_scratch_space is non-zero.
     * 2. Decompress as appropriate. This may allocate a scratch buffer
     *    for unpacked if needed and own_scratch_space is non-zero.
     *    NB: If own_scratch_space is 0 and the unpack buffer is NULL,
     *        the routine will return at this point, allowing the user
     *        to provide custom decompression calls
     * 3. If pointers and stride information are provided to the channel
     *    decode information list, propagate the decompressed data into
     *    the output.
     *
     * if own_scratch_space is zero, buffers must be provided. In this
     * mode, reading will stop as soon as a buffer needed is NULL, but
     * after the read of the compressed data, will not report an error.
     * In this way, one is able to read the compressed data only, or only
     * read and decompress, or go all the way to decoding into output
     * pixel locations. Do note that some interpretation of the
     * compression type is performed, in that if the data is not
     * compressed, the intermediate buffers are not needed, and so the
     * routine can jump straight to reading data into the output as a
     * "minimal-copy" (ignoring O.S. level buffering, zero-copy).
     *
     * @param file The file to read from
     * @param cinfo The chunk to read, as computed using
     *              @sa compute_chunk_for_scanline or @sa compute_chunk_for_tile.
     */
    fn exr_read_chunk(&self) -> Result<DecodeChunkInfo, ExrError> {
        // openexr_sys::exr_read_chunk();

        todo!()
    }

    //  /** @brief Perform buffer decompression
    //   * TODO: This will need more info for all the compression types
    //   *       like b44 and pxr24 that do different things depending on
    //   *       the underlying buffer type
    //   */
    // TODO: Implement
    //  fn exr_decompress_data (
    //     &self,
    //      const exr_COMPRESSION_TYPE_t ctype,
    //      void *compressed_data, size_t comp_buf_size,
    //      void *uncompressed_data, size_t uncompressed_size ) -> i32 {
    //         todo!()
    //     }
}

impl<'a> DecodeChunkInfo<'a> {
    /** @brief Free any allocated data owned by the decode_chunk_info struct */
    fn exr_destroy_decode_chunk_info(mut self) -> () {
        unsafe {
            openexr_sys::exr_destroy_decode_chunk_info(&mut self.chunk_info)
        }
    }    
}