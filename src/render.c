#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include <render.h>

#define STB_TRUETYPE_IMPLEMENTATION
#include "stb/stb_truetype.h"
#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "stb/stb_image_write.h"
#define STB_IMAGE_IMPLEMENTATION
#include "stb/stb_image.h"
/**
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
                   int output_h, unsigned char *ascii_colors) {
  // creare the img data
  float char_h = 32.0f;
  float char_w = char_h / 2;
  int width = output_w * char_w;
  int height = output_h * char_h;

  unsigned char *pixels = malloc(width * height * 3);
  if (!pixels) {
    printf("Error: Failed to allocate pixel buffer\n");
    return 1;
  }
  memset(pixels, 0, width * height * 3);

  // load font file
  unsigned char *font_buffer = NULL;
  stbtt_fontinfo font;
  if (load_font("MesloLGS-NF-Bold.ttf", font_buffer, &font)) {
    free(pixels);
    return EXIT_FAILURE;
  }

  // read ASCII content
  char *fcontent = NULL;
  int res = get_text_from_file(output_filename, &fcontent);
  if (res) {
    free(font_buffer);
    free(pixels);
    return EXIT_FAILURE;
  }

  // render text
  const char *text = fcontent;
  float scale = stbtt_ScaleForPixelHeight(&font, char_h);
  int x = 0, y = 0;

  int ascent, descent, line_gap;
  stbtt_GetFontVMetrics(&font, &ascent, &descent, &line_gap);
  y += (int)(ascent * scale);

  int counter = 0;
  for (const char *c = text; *c; c++) {
    if (*c == '\n') {
      x = 0;
      y += (int)((ascent - descent + line_gap) * scale);
      continue;
    }

    int advance, lsb, x0, y0, x1, y1;
    char render_char = switch_to_render_char(c);

    stbtt_GetCodepointHMetrics(&font, render_char, &advance, &lsb);
    stbtt_GetCodepointBitmapBox(&font, render_char, scale, scale, &x0, &y0, &x1,
                                &y1);

    unsigned char *bitmap = malloc((x1 - x0) * (y1 - y0));
    if (!bitmap) {
      printf("Error: Failed to allocate bitmap\n");
      free(fcontent);
      free(font_buffer);
      free(pixels);
      return 1;
    }

    stbtt_MakeCodepointBitmap(&font, bitmap, x1 - x0, y1 - y0, x1 - x0, scale,
                              scale, render_char);

    // Draw character with ASCII colors
    for (int dy = 0; dy < y1 - y0; dy++) {
      for (int dx = 0; dx < x1 - x0; dx++) {
        int pos_pixel = ((y + y0 + dy) * width + (x + x0 + dx)) * 3;
        if (bitmap[dy * (x1 - x0) + dx] == 255 &&
            counter < output_w * output_h) {
          pixels[pos_pixel] = ascii_colors[counter * 3];
          pixels[pos_pixel + 1] = ascii_colors[counter * 3 + 1];
          pixels[pos_pixel + 2] = ascii_colors[counter * 3 + 2];
        }
      }
    }
    free(bitmap);
    x += (int)(advance * scale);
    counter++;
  }

  // Save PNG
  char png_filename[256];
  snprintf(png_filename, sizeof(png_filename), "%s.png", output_filename);
  if (!stbi_write_png(png_filename, width, height, 3, pixels, width * 3)) {
    printf("Error saving PNG image\n");
  }

  // Cleanup
  free(fcontent);
  free(font_buffer);
  free(pixels);

  printf("Image generated: %s\n", png_filename);
  return 0;
}

int load_font(char *font_filename, unsigned char *font_buffer,
              stbtt_fontinfo *font) {
  long font_size;
  FILE *font_file = fopen(font_filename, "rb");
  if (!font_file) {
    printf("Error: Failed to load font file\n");
    return 1;
  }

  fseek(font_file, 0, SEEK_END);
  font_size = ftell(font_file);
  fseek(font_file, 0, SEEK_SET);

  font_buffer = malloc(font_size);
  if (!font_buffer) {
    printf("Error: Failed to allocate font buffer\n");
    fclose(font_file);
    return 1;
  }

  if (fread(font_buffer, 1, font_size, font_file) != font_size) {
    printf("Error: Failed to read font file\n");
    fclose(font_file);
    free(font_buffer);
    return 1;
  }
  fclose(font_file);

  // Initialize font
  if (!stbtt_InitFont(font, font_buffer, 0)) {
    printf("Error: Failed to initialize font\n");
    free(font_buffer);
    return 1;
  }
  return 0;
}

char switch_to_render_char(char *c) {

  char render_gradient[] = {'$', '&', '8', 'W', 'M', 'B', '@', '%',
                            '#', '*', '+', '=', '-', ':', '.', ' '};

  switch (*c) {
  case '@':
    return render_gradient[0];
    break;
  case '&':
    return render_gradient[1];
    break;
  case '%':
    return render_gradient[2];
    break;
  case '#':
    return render_gradient[3];
    break;
  case '*':
    return render_gradient[4];
    break;
  case '+':
    return render_gradient[5];
    break;
  case '~':
    return render_gradient[6];
    break;
  case '=':
    return render_gradient[7];
    break;
  case '_':
    return render_gradient[8];
    break;
  case '-':
    return render_gradient[9];
    break;
  case ';':
    return render_gradient[10];
    break;
  case ':':
    return render_gradient[11];
    break;
  case '`':
    return render_gradient[12];
    break;
  case '\'':
    return render_gradient[13];
    break;
  case '.':
    return render_gradient[14];
    break;
  case ' ':
    return render_gradient[15];
    break;
  default:
    printf("index error");
    return EXIT_FAILURE;
    break;
  }
}

int get_text_from_file(char *filename, char **full_content) {
  long fsize = 0;
  FILE *fp = fopen(filename, "r");
  if (fp) {
    fseek(fp, 0, SEEK_END);
    fsize = ftell(fp);
    rewind(fp);

    *full_content = (char *)malloc(fsize + 1);
    if (!*full_content) {
      printf("Error: Failed to allocate content buffer\n");
      fclose(fp);
      return 1;
    }

    if (fread(*full_content, 1, fsize, fp) != fsize) {
      printf("Error: Failed to read content file\n");
      fclose(fp);
      free(*full_content);
      *full_content = NULL;
      return 1;
    }
    (*full_content)[fsize] = '\0';
    fclose(fp);
    return 0;
  } else {
    printf("Error: Failed to open text file\n");
    return 1;
  }
}
