(function() {var implementors = {
"base64":[["impl&lt;'e, E: <a class=\"trait\" href=\"base64/engine/trait.Engine.html\" title=\"trait base64::engine::Engine\">Engine</a>, R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"base64/read/struct.DecoderReader.html\" title=\"struct base64::read::DecoderReader\">DecoderReader</a>&lt;'e, E, R&gt;"]],
"brotli":[["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>, BufferType: <a class=\"trait\" href=\"brotli/writer/trait.SliceWrapperMut.html\" title=\"trait brotli::writer::SliceWrapperMut\">SliceWrapperMut</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.u8.html\">u8</a>&gt;, Alloc: <a class=\"trait\" href=\"brotli/enc/combined_alloc/trait.BrotliAlloc.html\" title=\"trait brotli::enc::combined_alloc::BrotliAlloc\">BrotliAlloc</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"brotli/enc/reader/struct.CompressorReaderCustomAlloc.html\" title=\"struct brotli::enc::reader::CompressorReaderCustomAlloc\">CompressorReaderCustomAlloc</a>&lt;R, BufferType, Alloc&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"brotli/enc/reader/struct.CompressorReader.html\" title=\"struct brotli::enc::reader::CompressorReader\">CompressorReader</a>&lt;R&gt;"]],
"brotli_decompressor":[["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>, BufferType: <a class=\"trait\" href=\"brotli_decompressor/trait.SliceWrapperMut.html\" title=\"trait brotli_decompressor::SliceWrapperMut\">SliceWrapperMut</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.u8.html\">u8</a>&gt;, AllocU8: <a class=\"trait\" href=\"brotli_decompressor/trait.Allocator.html\" title=\"trait brotli_decompressor::Allocator\">Allocator</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.u8.html\">u8</a>&gt;, AllocU32: <a class=\"trait\" href=\"brotli_decompressor/trait.Allocator.html\" title=\"trait brotli_decompressor::Allocator\">Allocator</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.u32.html\">u32</a>&gt;, AllocHC: <a class=\"trait\" href=\"brotli_decompressor/trait.Allocator.html\" title=\"trait brotli_decompressor::Allocator\">Allocator</a>&lt;<a class=\"struct\" href=\"brotli_decompressor/reader/struct.HuffmanCode.html\" title=\"struct brotli_decompressor::reader::HuffmanCode\">HuffmanCode</a>&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"brotli_decompressor/reader/struct.DecompressorCustomAlloc.html\" title=\"struct brotli_decompressor::reader::DecompressorCustomAlloc\">DecompressorCustomAlloc</a>&lt;R, BufferType, AllocU8, AllocU32, AllocHC&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"brotli_decompressor/reader/struct.Decompressor.html\" title=\"struct brotli_decompressor::reader::Decompressor\">Decompressor</a>&lt;R&gt;"]],
"bytes":[["impl&lt;B: <a class=\"trait\" href=\"bytes/buf/trait.Buf.html\" title=\"trait bytes::buf::Buf\">Buf</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"bytes/buf/struct.Reader.html\" title=\"struct bytes::buf::Reader\">Reader</a>&lt;B&gt;"]],
"cfb":[["impl&lt;'a, F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Seek.html\" title=\"trait std::io::Seek\">Seek</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"cfb/struct.Stream.html\" title=\"struct cfb::Stream\">Stream</a>&lt;'a, F&gt;"]],
"digest":[["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"digest/core_api/struct.XofReaderCoreWrapper.html\" title=\"struct digest::core_api::XofReaderCoreWrapper\">XofReaderCoreWrapper</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"digest/core_api/trait.XofReaderCore.html\" title=\"trait digest::core_api::XofReaderCore\">XofReaderCore</a>,\n    T::<a class=\"associatedtype\" href=\"digest/core_api/trait.BlockSizeUser.html#associatedtype.BlockSize\" title=\"type digest::core_api::BlockSizeUser::BlockSize\">BlockSize</a>: <a class=\"trait\" href=\"typenum/type_operators/trait.IsLess.html\" title=\"trait typenum::type_operators::IsLess\">IsLess</a>&lt;<a class=\"type\" href=\"digest/consts/type.U256.html\" title=\"type digest::consts::U256\">U256</a>&gt;,\n    <a class=\"type\" href=\"typenum/operator_aliases/type.Le.html\" title=\"type typenum::operator_aliases::Le\">Le</a>&lt;T::<a class=\"associatedtype\" href=\"digest/core_api/trait.BlockSizeUser.html#associatedtype.BlockSize\" title=\"type digest::core_api::BlockSizeUser::BlockSize\">BlockSize</a>, <a class=\"type\" href=\"digest/consts/type.U256.html\" title=\"type digest::consts::U256\">U256</a>&gt;: <a class=\"trait\" href=\"typenum/marker_traits/trait.NonZero.html\" title=\"trait typenum::marker_traits::NonZero\">NonZero</a>,</span>"]],
"flate2":[["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/write/struct.DeflateDecoder.html\" title=\"struct flate2::write::DeflateDecoder\">DeflateDecoder</a>&lt;W&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/read/struct.DeflateDecoder.html\" title=\"struct flate2::read::DeflateDecoder\">DeflateDecoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/bufread/struct.DeflateDecoder.html\" title=\"struct flate2::bufread::DeflateDecoder\">DeflateDecoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/bufread/struct.ZlibEncoder.html\" title=\"struct flate2::bufread::ZlibEncoder\">ZlibEncoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/bufread/struct.ZlibDecoder.html\" title=\"struct flate2::bufread::ZlibDecoder\">ZlibDecoder</a>&lt;R&gt;"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/write/struct.DeflateEncoder.html\" title=\"struct flate2::write::DeflateEncoder\">DeflateEncoder</a>&lt;W&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/bufread/struct.MultiGzDecoder.html\" title=\"struct flate2::bufread::MultiGzDecoder\">MultiGzDecoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/struct.CrcReader.html\" title=\"struct flate2::CrcReader\">CrcReader</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/read/struct.ZlibDecoder.html\" title=\"struct flate2::read::ZlibDecoder\">ZlibDecoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/read/struct.ZlibEncoder.html\" title=\"struct flate2::read::ZlibEncoder\">ZlibEncoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/read/struct.MultiGzDecoder.html\" title=\"struct flate2::read::MultiGzDecoder\">MultiGzDecoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/write/struct.GzEncoder.html\" title=\"struct flate2::write::GzEncoder\">GzEncoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/read/struct.GzDecoder.html\" title=\"struct flate2::read::GzDecoder\">GzDecoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/read/struct.GzEncoder.html\" title=\"struct flate2::read::GzEncoder\">GzEncoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/bufread/struct.GzEncoder.html\" title=\"struct flate2::bufread::GzEncoder\">GzEncoder</a>&lt;R&gt;"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/write/struct.ZlibEncoder.html\" title=\"struct flate2::write::ZlibEncoder\">ZlibEncoder</a>&lt;W&gt;"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/write/struct.GzDecoder.html\" title=\"struct flate2::write::GzDecoder\">GzDecoder</a>&lt;W&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/bufread/struct.GzDecoder.html\" title=\"struct flate2::bufread::GzDecoder\">GzDecoder</a>&lt;R&gt;"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/write/struct.ZlibDecoder.html\" title=\"struct flate2::write::ZlibDecoder\">ZlibDecoder</a>&lt;W&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/bufread/struct.DeflateEncoder.html\" title=\"struct flate2::bufread::DeflateEncoder\">DeflateEncoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/read/struct.DeflateEncoder.html\" title=\"struct flate2::read::DeflateEncoder\">DeflateEncoder</a>&lt;R&gt;"]],
"rand_core":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for dyn <a class=\"trait\" href=\"rand_core/trait.RngCore.html\" title=\"trait rand_core::RngCore\">RngCore</a>"]],
"tar":[["impl&lt;'a, R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"tar/struct.Entry.html\" title=\"struct tar::Entry\">Entry</a>&lt;'a, R&gt;"]],
"tempfile":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"tempfile/struct.SpooledTempFile.html\" title=\"struct tempfile::SpooledTempFile\">SpooledTempFile</a>"],["impl&lt;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"tempfile/struct.NamedTempFile.html\" title=\"struct tempfile::NamedTempFile\">NamedTempFile</a>&lt;F&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for &amp;<a class=\"struct\" href=\"tempfile/struct.NamedTempFile.html\" title=\"struct tempfile::NamedTempFile\">NamedTempFile</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.69.0/std/fs/struct.File.html\" title=\"struct std::fs::File\">File</a>&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()