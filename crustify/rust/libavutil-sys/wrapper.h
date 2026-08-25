#include <libavutil/audio_fifo.h>
#include <libavutil/avutil.h>
#include <libavutil/avstring.h>
#include <libavutil/bprint.h>
#include <libavutil/buffer_internal.h>
#include <libavutil/channel_layout.h>
#include <libavutil/dict.h>
#include <libavutil/dovi_meta.h>
#include <libavutil/fifo.h>
#include <libavutil/file.h>
#include <libavutil/film_grain_params.h>
#include <libavutil/frame.h>
#include <libavutil/hash.h>
#include <libavutil/hdr_dynamic_metadata.h>
#include <libavutil/hdr_dynamic_vivid_metadata.h>
#include <libavutil/hwcontext.h>
#include <libavutil/iamf.h>
#include <libavutil/imgutils.h>
#include <libavutil/log.h>
#include <libavutil/mathematics.h>
#include <libavutil/md5.h>
#include <libavutil/mem.h>
#include <libavutil/opt.h>
#include <libavutil/pixdesc.h>
#include <libavutil/rational.h>
#include <libavutil/samplefmt.h>
#include <libavutil/pixfmt.h>
#include <libavutil/stereo3d.h>
#include <libavutil/time.h>

int crustify_av_ceil_log2_c(int value);
int64_t crustify_av_clip64_c(int64_t value, int64_t min, int64_t max);
int crustify_av_clip_c(int value, int min, int max);
int16_t crustify_av_clip_int16_c(int value);
int8_t crustify_av_clip_int8_c(int value);
int crustify_av_clip_intp2_c(int value, int bits);
uint16_t crustify_av_clip_uint16_c(int value);
uint8_t crustify_av_clip_uint8_c(int value);
unsigned crustify_av_clip_uintp2_c(int value, int bits);
double crustify_av_clipd_c(double value, double min, double max);
float crustify_av_clipf_c(float value, float min, float max);
int32_t crustify_av_clipl_int32_c(int64_t value);
int crustify_av_isdigit(int value);
int crustify_av_isgraph(int value);
int crustify_av_isspace(int value);
int crustify_av_isxdigit(int value);
AVDOVIRpuDataHeader *crustify_av_dovi_get_header(const AVDOVIMetadata *data);
AVDOVIDataMapping *crustify_av_dovi_get_mapping(const AVDOVIMetadata *data);
AVDOVIColorMetadata *crustify_av_dovi_get_color(const AVDOVIMetadata *data);
void crustify_av_bprintf_string(AVBPrint *buf, const char *text);
void crustify_av_vbprintf_string(AVBPrint *buf, const char *text);
void *crustify_av_iamf_param_definition_get_subblock(const AVIAMFParamDefinition *par,
                                                     unsigned int idx);
