#include <stdio.h>
#include <stdlib.h>
#include <utils.h>
#include <stdint.h>
#include "stb/stb_truetype.h"


/*
 * @brief Renders ASCII art to a PNG image
 * @param output_filename Base output filename
 * @param output_w Output width in chars
 * @param output_h Output height in chars
 * @param ascii_colors Color data for ASCII chars
 * @param bg_color Color data for background image
 * @param font_family Font filename for rendering 
 * @return 0 on success, 1 on failure
 */
 int renderAsciiPNG(char *output_filename, int output_w,
                   int output_h, unsigned char *ascii_colors, RGB bg_color, char *font_family) ;

/*
 * @brief Load a stb_truetype font
 * @param font_filename Font filename
 * @param font_buffer unsigned char buffer 
 * @param font stbtt_fontinfo font pointer
 * @return 0 on success, 1 on failure
 */
int load_font(const char *font_resource_path, unsigned char **font_buffer,
              stbtt_fontinfo *font) ;

/*
 * @brief Extract all the text in a file
 * @param filename text filename
 * @param full_content pointer referencing the all the text from file
 * @return 0 on success, 1 on failure
 */
static int get_text_from_file(char *filename, char **full_content) ;

static char switch_to_render_char(const char c);
void displayRenderMenu(RGB *bg_color_render, char *font_family);
