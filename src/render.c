#include <gio/gio.h>
#include <glib.h>
#include <gmodule.h>
#include <ncurses.h>
#include <render.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define STB_TRUETYPE_IMPLEMENTATION
#include "stb/stb_truetype.h"
#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "stb/stb_image_write.h"
#define STB_IMAGE_IMPLEMENTATION
#include "stb/stb_image.h"

#define NUM_FONTS 8

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
int renderAsciiPNG(char *output_filename, int output_w, int output_h,
                   unsigned char *ascii_colors, uint8_t bg_color,
                   char *font_name) {
  // creare the img data
  float char_h = 32.0f;
  float char_w = char_h / 2;
  int width = output_w * char_w;
  int height = output_h * char_h;

  char font_file[] = "/org/asciiparser/data/fonts/";
  strcat(font_file, font_name);

  unsigned char *pixels = malloc(width * height * 3);
  if (!pixels) {
    printf("Error: Failed to allocate pixel buffer\n");
    pixels = NULL;
    return 1;
  }
  memset(pixels, bg_color, width * height * 3);

  // load font file
  unsigned char *font_buffer = NULL;
  stbtt_fontinfo font;

  if (load_font(font_file, &font_buffer, &font)) {
    free(pixels);
    pixels = NULL;
    printf("error loading font\n");
    return EXIT_FAILURE;
  }

  // read ASCII content from the output text file
  char *fcontent = NULL;
  int res = get_text_from_file(output_filename, &fcontent);
  if (res) {
    // free up memory and set NULL the pointers
    // as good programming practices
    free(font_buffer);
    free(pixels);
    font_buffer = NULL;
    pixels = NULL;
    return EXIT_FAILURE;
  }

  // render text
  const char *text = fcontent;
  float scale = stbtt_ScaleForPixelHeight(&font, char_h); // set the font size

  // variables to locate the x,y position of each char in the image
  int x = 0, y = 0;
  // metrics of the font
  int ascent, descent, line_gap;
  stbtt_GetFontVMetrics(&font, &ascent, &descent, &line_gap);
  y += (int)(ascent * scale);

  // for every char, draw it inside the image in the x,y position and then add
  // enought space to the next char
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
      pixels = NULL;
      font_buffer = NULL;
      fcontent = NULL;
      return 1;
    }

    stbtt_MakeCodepointBitmap(&font, bitmap, x1 - x0, y1 - y0, x1 - x0, scale,
                              scale, render_char);

    // draw character with ascii colors
    for (int dy = 0; dy < y1 - y0; dy++) {
      for (int dx = 0; dx < x1 - x0; dx++) {
        int pixel_y = height - 1 - (y + y0 + dy);
        int pixel_x = x + x0 + dx;

        if (pixel_y >= 0 && pixel_y < height && pixel_x >= 0 &&
            pixel_x < width) {
          int pos_pixel =
              ((height - 1 - (y + y0 + dy)) * width + (x + x0 + dx)) * 3;
          if (bitmap[dy * (x1 - x0) + dx] == 255 &&
              counter < output_w * output_h) {
            pixels[pos_pixel] = ascii_colors[counter * 3];
            pixels[pos_pixel + 1] = ascii_colors[counter * 3 + 1];
            pixels[pos_pixel + 2] = ascii_colors[counter * 3 + 2];
          } else if (bitmap[dy * (x1 - x0) + dx] == 0 &&
                     counter < output_w * output_h) {
            pixels[pos_pixel] = bg_color;
            pixels[pos_pixel + 1] = bg_color;
            pixels[pos_pixel + 2] = bg_color;
          }
        }
      }
    }
    free(bitmap); // clean up the bitmap data
    x += (int)(advance * scale);
    counter++;
  }

  // save to png
  char png_filename[256];
  snprintf(png_filename, sizeof(png_filename), "%s.png", output_filename);
  if (!stbi_write_png(png_filename, width, height, 3, pixels, width * 3)) {
    printf("Error saving PNG image\n");
  }

  // Cleanup
  free(fcontent);
  free(font_buffer);
  free(pixels);
  pixels = NULL;
  font_buffer = NULL;
  fcontent = NULL;

  printf("Image rendered: %s\n", png_filename);
  return 0;
}

// load a font from gresources by its filepath and save it in the font_buffer
// variable
int load_font(const char *font_resource_path, unsigned char **font_buffer,
              stbtt_fontinfo *font) {
  // check if the font exists embembed in gresources
  GBytes *font_data = g_resources_lookup_data(
      font_resource_path, G_RESOURCE_LOOKUP_FLAGS_NONE, NULL);
  if (!font_data) {
    printf("Error: Failed to load font resource at path '%s'\n",
           font_resource_path);
    return 1;
  }

  // get the font data from gresources
  gsize font_size = 0;
  *font_buffer = (unsigned char *)g_bytes_get_data(font_data, &font_size);

  // copy the font data to free up the GBytes data
  *font_buffer = malloc(font_size);
  if (!*font_buffer) {
    printf("Error: Failed to allocate font buffer\n");
    g_bytes_unref(font_data);
    return 1;
  }
  memcpy(*font_buffer, g_bytes_get_data(font_data, NULL), font_size);

  // free up GBytes
  g_bytes_unref(font_data);

  // init  the font with stb
  if (!stbtt_InitFont(font, *font_buffer, 0)) {
    printf("Error: Failed to initialize font\n");
    free(*font_buffer);
    *font_buffer = NULL;
    return 1;
  }

  return 0;
}

// for render the image I decided to use a different scale of chars, so this
// function executes the switch between the gradient of the text file and the
// image gradient, for more info please check the README.md file
static char switch_to_render_char(char *c) {
  static char render_gradient[] = {'$', '&', '8', 'W', 'M', 'B', '@', '%',
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

// extract all the text from a given filename
static int get_text_from_file(char *filename, char **full_content) {
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

// display a ncurses menu to select a font and
// bg color, only if rendering is enabled
void displayRenderMenu(uint8_t *bg_color_render, char *font_family) {
  static char *font_options[NUM_FONTS] = {
      "CascadiaCodeNF-Bold.ttf",
      "CascadiaCodeNF-Regular.ttf",
      "FiraCode-Bold.ttf",
      "FiraCode-Regular.ttf",
      "Hack-Bold.ttf",
      "Hack-Regular.ttf",
      "JetBrainsMonoNL-Bold.ttf",
      "JetBrainsMonoNL-Regular.ttf",
  };

  static char *bgcolors_options[2] = {"white", "black"};

  initscr();            // init screen to use ncurses
  cbreak();             // enable realtime char access
  noecho();             // disable input chars display
  keypad(stdscr, TRUE); // enable special keys
  curs_set(0);          // hide cursor

  // verify if the terminal performs colors
  if (has_colors()) {
    start_color();
    init_pair(1, COLOR_WHITE, COLOR_BLUE);
    init_pair(2, COLOR_BLACK, COLOR_WHITE);
  }

  int opt = 0;
  int key;

  while (1) {
    clear();

    mvprintw(2, (COLS - 10) / 2, "Select a font:");

    for (int i = 0; i < NUM_FONTS; i++) {
      if (i == opt) {
        attron(COLOR_PAIR(1) | A_BOLD);
        mvprintw(5 + i, (COLS - strlen(font_options[i])) / 2, "%s",
                 font_options[i]);
        attroff(COLOR_PAIR(1) | A_BOLD);
      } else {
        attron(COLOR_PAIR(2));
        mvprintw(5 + i, (COLS - strlen(font_options[i])) / 2, "%s",
                 font_options[i]);
        attroff(COLOR_PAIR(2));
      }
    }

    key = getch();

    if (key == 10) {
      strcpy(font_family, font_options[opt]);
      mvprintw(3, (COLS - 10) / 2, "%s", font_family);

      refresh();
      break;
    } else {
      switch (key) {
      case KEY_UP:
        opt = (opt > 0) ? opt - 1 : NUM_FONTS - 1;
        break;
      case KEY_DOWN:
        opt = (opt < NUM_FONTS - 1) ? opt + 1 : 0;
        break;
      }
    }
  }

  int tmp_length_ff = strlen(font_family);
  opt = 0;

  while (1) {
    clear();

    mvprintw(2, (COLS - (tmp_length_ff + 15)) / 2, "Font selected: %s",
             font_family);
    mvprintw(3, (COLS - 26) / 2, "Select a background color:");

    for (int i = 0; i < 2; i++) {
      if (i == opt) {
        attron(COLOR_PAIR(1) | A_BOLD);
        mvprintw(5 + i, (COLS - strlen(bgcolors_options[i])) / 2, "%s",
                 bgcolors_options[i]);
        attroff(COLOR_PAIR(1) | A_BOLD);
      } else {
        attron(COLOR_PAIR(2));
        mvprintw(5 + i, (COLS - strlen(bgcolors_options[i])) / 2, "%s",
                 bgcolors_options[i]);
        attroff(COLOR_PAIR(2));
      }
    }

    key = getch();

    switch (key) {
    case KEY_UP:
      opt = (opt > 0) ? opt - 1 : 2 - 1;
      break;
    case KEY_DOWN:
      opt = (opt < 2 - 1) ? opt + 1 : 0;
      break;
    }
    if (key == 10) {
      if (!strcmp(bgcolors_options[opt], "black")) {
        *bg_color_render = 0;
      } else {
        *bg_color_render = 255;
      }
      mvprintw(10, (COLS - 13 - strlen(font_family)) / 2, "font family: %s",
               font_family);
      mvprintw(11, (COLS - 18 - strlen(bgcolors_options[opt])) / 2,
               "background color: %s", bgcolors_options[opt]);
      mvprintw(12, (COLS - 25) / 2, "PRESS ANY KEY TO CONTINUE");

      refresh();
      getch();
      break;
    }
  }

  endwin();
}
