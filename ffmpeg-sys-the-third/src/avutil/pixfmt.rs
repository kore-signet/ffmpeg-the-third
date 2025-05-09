use libc::c_int;

use crate::AVPixelFormat::*;
use crate::{AVChromaLocation, AVPixelFormat};

impl AVChromaLocation {
    pub fn from_c_int(n: c_int) -> Option<Self> {
        use AVChromaLocation as AVCL;

        Some(match n {
            n if n == AVCL::AVCHROMA_LOC_UNSPECIFIED as c_int => AVCL::AVCHROMA_LOC_UNSPECIFIED,
            n if n == AVCL::AVCHROMA_LOC_LEFT as c_int => AVCL::AVCHROMA_LOC_LEFT,
            n if n == AVCL::AVCHROMA_LOC_CENTER as c_int => AVCL::AVCHROMA_LOC_CENTER,
            n if n == AVCL::AVCHROMA_LOC_TOPLEFT as c_int => AVCL::AVCHROMA_LOC_TOPLEFT,
            n if n == AVCL::AVCHROMA_LOC_TOP as c_int => AVCL::AVCHROMA_LOC_TOP,
            n if n == AVCL::AVCHROMA_LOC_BOTTOMLEFT as c_int => AVCL::AVCHROMA_LOC_BOTTOMLEFT,
            n if n == AVCL::AVCHROMA_LOC_BOTTOM as c_int => AVCL::AVCHROMA_LOC_BOTTOM,

            _ => return None,
        })
    }
}

#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_RGB32: AVPixelFormat = AV_PIX_FMT_BGRA;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_RGB32_1: AVPixelFormat = AV_PIX_FMT_ABGR;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_BGR32: AVPixelFormat = AV_PIX_FMT_RGBA;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_BGR32_1: AVPixelFormat = AV_PIX_FMT_ARGB;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_0RGB32: AVPixelFormat = AV_PIX_FMT_BGR0;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_0BGR32: AVPixelFormat = AV_PIX_FMT_RGB0;

#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_GRAY16: AVPixelFormat = AV_PIX_FMT_GRAY16LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YA16: AVPixelFormat = AV_PIX_FMT_YA16LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_RGB48: AVPixelFormat = AV_PIX_FMT_RGB48LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_RGB565: AVPixelFormat = AV_PIX_FMT_RGB565LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_RGB555: AVPixelFormat = AV_PIX_FMT_RGB555LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_RGB444: AVPixelFormat = AV_PIX_FMT_RGB444LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_BGR48: AVPixelFormat = AV_PIX_FMT_BGR48LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_BGR565: AVPixelFormat = AV_PIX_FMT_BGR565LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_BGR555: AVPixelFormat = AV_PIX_FMT_BGR555LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_BGR444: AVPixelFormat = AV_PIX_FMT_BGR444LE;

#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUV420P9: AVPixelFormat = AV_PIX_FMT_YUV420P9LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUV422P9: AVPixelFormat = AV_PIX_FMT_YUV422P9LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUV444P9: AVPixelFormat = AV_PIX_FMT_YUV444P9LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUV420P10: AVPixelFormat = AV_PIX_FMT_YUV420P10LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUV422P10: AVPixelFormat = AV_PIX_FMT_YUV422P10LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUV440P10: AVPixelFormat = AV_PIX_FMT_YUV440P10LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUV444P10: AVPixelFormat = AV_PIX_FMT_YUV444P10LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUV420P12: AVPixelFormat = AV_PIX_FMT_YUV420P12LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUV422P12: AVPixelFormat = AV_PIX_FMT_YUV422P12LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUV440P12: AVPixelFormat = AV_PIX_FMT_YUV440P12LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUV444P12: AVPixelFormat = AV_PIX_FMT_YUV444P12LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUV420P14: AVPixelFormat = AV_PIX_FMT_YUV420P14LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUV422P14: AVPixelFormat = AV_PIX_FMT_YUV422P14LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUV444P14: AVPixelFormat = AV_PIX_FMT_YUV444P14LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUV420P16: AVPixelFormat = AV_PIX_FMT_YUV420P16LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUV422P16: AVPixelFormat = AV_PIX_FMT_YUV422P16LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUV444P16: AVPixelFormat = AV_PIX_FMT_YUV444P16LE;

#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_GBRP9: AVPixelFormat = AV_PIX_FMT_GBRP9LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_GBRP10: AVPixelFormat = AV_PIX_FMT_GBRP10LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_GBRP12: AVPixelFormat = AV_PIX_FMT_GBRP12LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_GBRP14: AVPixelFormat = AV_PIX_FMT_GBRP14LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_GBRP16: AVPixelFormat = AV_PIX_FMT_GBRP16LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_GBRAP16: AVPixelFormat = AV_PIX_FMT_GBRAP16LE;

#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_BAYER_BGGR16: AVPixelFormat = AV_PIX_FMT_BAYER_BGGR16LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_BAYER_RGGB16: AVPixelFormat = AV_PIX_FMT_BAYER_RGGB16LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_BAYER_GBRG16: AVPixelFormat = AV_PIX_FMT_BAYER_GBRG16LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_BAYER_GRBG16: AVPixelFormat = AV_PIX_FMT_BAYER_GRBG16LE;

#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUVA420P9: AVPixelFormat = AV_PIX_FMT_YUVA420P9LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUVA422P9: AVPixelFormat = AV_PIX_FMT_YUVA422P9LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUVA444P9: AVPixelFormat = AV_PIX_FMT_YUVA444P9LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUVA420P10: AVPixelFormat = AV_PIX_FMT_YUVA420P10LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUVA422P10: AVPixelFormat = AV_PIX_FMT_YUVA422P10LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUVA444P10: AVPixelFormat = AV_PIX_FMT_YUVA444P10LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUVA420P16: AVPixelFormat = AV_PIX_FMT_YUVA420P16LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUVA422P16: AVPixelFormat = AV_PIX_FMT_YUVA422P16LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_YUVA444P16: AVPixelFormat = AV_PIX_FMT_YUVA444P16LE;

#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_XYZ12: AVPixelFormat = AV_PIX_FMT_XYZ12LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_NV20: AVPixelFormat = AV_PIX_FMT_NV20LE;
#[cfg(target_endian = "little")]
pub const AV_PIX_FMT_AYUV64: AVPixelFormat = AV_PIX_FMT_AYUV64LE;

#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_RGB32: AVPixelFormat = AV_PIX_FMT_ARGB;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_RGB32_1: AVPixelFormat = AV_PIX_FMT_RGBA;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_BGR32: AVPixelFormat = AV_PIX_FMT_ABGR;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_BGR32_1: AVPixelFormat = AV_PIX_FMT_BGRA;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_0RGB32: AVPixelFormat = AV_PIX_FMT_0RGB;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_0BGR32: AVPixelFormat = AV_PIX_FMT_0BGR;

#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_GRAY16: AVPixelFormat = AV_PIX_FMT_GRAY16BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YA16: AVPixelFormat = AV_PIX_FMT_YA16BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_RGB48: AVPixelFormat = AV_PIX_FMT_RGB48BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_RGB565: AVPixelFormat = AV_PIX_FMT_RGB565BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_RGB555: AVPixelFormat = AV_PIX_FMT_RGB555BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_RGB444: AVPixelFormat = AV_PIX_FMT_RGB444BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_BGR48: AVPixelFormat = AV_PIX_FMT_BGR48BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_BGR565: AVPixelFormat = AV_PIX_FMT_BGR565BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_BGR555: AVPixelFormat = AV_PIX_FMT_BGR555BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_BGR444: AVPixelFormat = AV_PIX_FMT_BGR444BE;

#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUV420P9: AVPixelFormat = AV_PIX_FMT_YUV420P9BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUV422P9: AVPixelFormat = AV_PIX_FMT_YUV422P9BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUV444P9: AVPixelFormat = AV_PIX_FMT_YUV444P9BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUV420P10: AVPixelFormat = AV_PIX_FMT_YUV420P10BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUV422P10: AVPixelFormat = AV_PIX_FMT_YUV422P10BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUV440P10: AVPixelFormat = AV_PIX_FMT_YUV440P10BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUV444P10: AVPixelFormat = AV_PIX_FMT_YUV444P10BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUV420P12: AVPixelFormat = AV_PIX_FMT_YUV420P12BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUV422P12: AVPixelFormat = AV_PIX_FMT_YUV422P12BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUV440P12: AVPixelFormat = AV_PIX_FMT_YUV440P12BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUV444P12: AVPixelFormat = AV_PIX_FMT_YUV444P12BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUV420P14: AVPixelFormat = AV_PIX_FMT_YUV420P14BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUV422P14: AVPixelFormat = AV_PIX_FMT_YUV422P14BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUV444P14: AVPixelFormat = AV_PIX_FMT_YUV444P14BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUV420P16: AVPixelFormat = AV_PIX_FMT_YUV420P16BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUV422P16: AVPixelFormat = AV_PIX_FMT_YUV422P16BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUV444P16: AVPixelFormat = AV_PIX_FMT_YUV444P16BE;

#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_GBRP9: AVPixelFormat = AV_PIX_FMT_GBRP9BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_GBRP10: AVPixelFormat = AV_PIX_FMT_GBRP10BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_GBRP12: AVPixelFormat = AV_PIX_FMT_GBRP12BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_GBRP14: AVPixelFormat = AV_PIX_FMT_GBRP14BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_GBRP16: AVPixelFormat = AV_PIX_FMT_GBRP16BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_GBRAP16: AVPixelFormat = AV_PIX_FMT_GBRAP16BE;

#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_BAYER_BGGR16: AVPixelFormat = AV_PIX_FMT_BAYER_BGGR16BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_BAYER_RGGB16: AVPixelFormat = AV_PIX_FMT_BAYER_RGGB16BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_BAYER_GBRG16: AVPixelFormat = AV_PIX_FMT_BAYER_GBRG16BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_BAYER_GRBG16: AVPixelFormat = AV_PIX_FMT_BAYER_GRBG16BE;

#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUVA420P9: AVPixelFormat = AV_PIX_FMT_YUVA420P9BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUVA422P9: AVPixelFormat = AV_PIX_FMT_YUVA422P9BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUVA444P9: AVPixelFormat = AV_PIX_FMT_YUVA444P9BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUVA420P10: AVPixelFormat = AV_PIX_FMT_YUVA420P10BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUVA422P10: AVPixelFormat = AV_PIX_FMT_YUVA422P10BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUVA444P10: AVPixelFormat = AV_PIX_FMT_YUVA444P10BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUVA420P16: AVPixelFormat = AV_PIX_FMT_YUVA420P16BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUVA422P16: AVPixelFormat = AV_PIX_FMT_YUVA422P16BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_YUVA444P16: AVPixelFormat = AV_PIX_FMT_YUVA444P16BE;

#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_XYZ12: AVPixelFormat = AV_PIX_FMT_XYZ12BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_NV20: AVPixelFormat = AV_PIX_FMT_NV20BE;
#[cfg(target_endian = "big")]
pub const AV_PIX_FMT_AYUV64: AVPixelFormat = AV_PIX_FMT_AYUV64BE;
