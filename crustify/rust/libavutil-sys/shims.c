#include <libavutil/avstring.h>
#include <libavutil/common.h>
#include <libavutil/dovi_meta.h>
#include <libavutil/bprint.h>
#include <stdarg.h>

int crustify_av_ceil_log2_c(int value) { return av_ceil_log2_c(value); }
int64_t crustify_av_clip64_c(int64_t value, int64_t min, int64_t max) { return av_clip64_c(value, min, max); }
int crustify_av_clip_c(int value, int min, int max) { return av_clip_c(value, min, max); }
int16_t crustify_av_clip_int16_c(int value) { return av_clip_int16_c(value); }
int8_t crustify_av_clip_int8_c(int value) { return av_clip_int8_c(value); }
int crustify_av_clip_intp2_c(int value, int bits) { return av_clip_intp2_c(value, bits); }
uint16_t crustify_av_clip_uint16_c(int value) { return av_clip_uint16_c(value); }
uint8_t crustify_av_clip_uint8_c(int value) { return av_clip_uint8_c(value); }
unsigned crustify_av_clip_uintp2_c(int value, int bits) { return av_clip_uintp2_c(value, bits); }
double crustify_av_clipd_c(double value, double min, double max) { return av_clipd_c(value, min, max); }
float crustify_av_clipf_c(float value, float min, float max) { return av_clipf_c(value, min, max); }
int32_t crustify_av_clipl_int32_c(int64_t value) { return av_clipl_int32_c(value); }
int crustify_av_isdigit(int value) { return av_isdigit(value); }
int crustify_av_isgraph(int value) { return av_isgraph(value); }
int crustify_av_isspace(int value) { return av_isspace(value); }
int crustify_av_isxdigit(int value) { return av_isxdigit(value); }
AVDOVIRpuDataHeader *crustify_av_dovi_get_header(const AVDOVIMetadata *data) { return av_dovi_get_header(data); }
AVDOVIDataMapping *crustify_av_dovi_get_mapping(const AVDOVIMetadata *data) { return av_dovi_get_mapping(data); }
AVDOVIColorMetadata *crustify_av_dovi_get_color(const AVDOVIMetadata *data) { return av_dovi_get_color(data); }
void crustify_av_bprintf_string(AVBPrint *buf, const char *text) { av_bprintf(buf, "%s", text); }
static void crustify_call_vbprintf(AVBPrint *buf, const char *fmt, ...) {
    va_list ap;
    va_start(ap, fmt);
    av_vbprintf(buf, fmt, ap);
    va_end(ap);
}
void crustify_av_vbprintf_string(AVBPrint *buf, const char *text) { crustify_call_vbprintf(buf, "%s", text); }
