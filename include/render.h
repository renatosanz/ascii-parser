#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include "stb/stb_truetype.h"

typedef struct {
  uint8_t  *data;
  size_t size;
} JPGMemoryBuffer;


/*
 * @brief Renders ASCII art to a PNG image
 * @param output_filename Base output filename
 * @param rgb_image Original image data
 * @param w Image width
 * @param h Image height
 * @param w_step Width step
 * @param h_step Height step
 * @param channels Color channels
 * @param output_w Output width in chars
 * @param output_h Output height in chars
 * @param ascii_colors Color data for ASCII chars
 * @return 0 on success, 1 on failure
 */
int renderAsciiPNG(char *output_filename, uint8_t *rgb_image, int w, int h,
                   int w_step, int h_step, int channels, int output_w,
                   int output_h, unsigned char *ascii_colors, uint8_t bg_color) ;

/*
 * @brief Load a stb_truetype font
 * @param font_filename Font filename
 * @param font_buffer unsigned char buffer 
 * @param font stbtt_fontinfo font pointer
 * @return 0 on success, 1 on failure
 */
int load_font(char *font_filename, unsigned char *font_buffer,   stbtt_fontinfo *font) ;

/*
 * @brief Extract all the text in a file
 * @param filename text filename
 * @param full_content pointer referencing the all the text from file
 * @return 0 on success, 1 on failure
 */
int get_text_from_file(char *filename, char **full_content) ;

char switch_to_render_char(char *c);

void jpg_write_to_memory(void *context, void *data, int size); 
