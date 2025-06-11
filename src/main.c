#include "glib-object.h"
#include "glib.h"
#include <bits/getopt_core.h>
#include <getopt.h>
#include <gtk/gtk.h>
#include <regex.h>
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

// Global application and window references
GtkApplication *app;
GtkWindow *win;

/**
 * @brief Callback function for GTK application activation
 * @param app The GTK application instance
 * @param user_data User data passed to the callback
 * @return Always returns 0
 */
static void *on_activate(GtkApplication *app, gpointer user_data) {
  GtkBuilder *builder = gtk_builder_new();
  gtk_builder_add_from_resource(builder, "/org/dec2bin/data/ui/dec2bin.ui",
                                NULL);

  win = GTK_WINDOW(gtk_builder_get_object(builder, "dec2bin_win"));
  g_object_unref(builder);

  gtk_application_add_window(GTK_APPLICATION(app), GTK_WINDOW(win));
  gtk_window_present(GTK_WINDOW(win));
  return 0;
}

/**
 * @brief Initializes and runs the GTK application
 * @param argc Argument count
 * @param argv Argument vector
 * @return Application exit status
 */
int mainGtkVersion(int argc, char **argv) {
  gtk_init();
  app = gtk_application_new("org.riprtx.asciiParser",
                            G_APPLICATION_DEFAULT_FLAGS);
  gtk_window_set_default_icon_name("dec2bin");
  g_signal_connect(app, "activate", G_CALLBACK(on_activate), NULL);
  int status = g_application_run(G_APPLICATION(app), argc, argv);
  g_object_unref(app);

  return status;
}

/**
 * @brief Converts an RGB image to ASCII art and saves to file
 * @param output_filename Output file path
 * @param rgb_image Input image data
 * @param width Image width in pixels
 * @param height Image height in pixels
 * @param w_step Width sampling step
 * @param h_step Height sampling step
 * @param channels Number of color channels
 * @param output_w Output width in characters
 * @param output_h Output height in characters
 * @param ascii_colors Buffer to store color data
 * @return 0 on success, -1 on failure
 */
int parse2file(char *output_filename, uint8_t *rgb_image, int width, int height,
               int w_step, int h_step, int channels, int output_w, int output_h,
               unsigned char *ascii_colors) {
  char gradient[] = {'@', '&', '%', '#', '*', '+',  '~', '=',
                     '_', '-', ';', ':', '`', '\'', '.', ' '};
  int num_chars = sizeof(gradient) / sizeof(gradient[0]);

  FILE *asciifile = fopen(output_filename, "w");
  if (!asciifile) {
    perror("Error opening file");
    return -1;
  }

  int counter_z = 0;
  for (int y = 0; y < height; y += h_step) {
    for (int x = 0; x < width; x += w_step) {
      size_t index = (y * width + x) * channels;
      unsigned char r = rgb_image[index];
      unsigned char g = rgb_image[index + 1];
      unsigned char b = rgb_image[index + 2];

      int intensity = (r + g + b) / 3;
      int gradient_index = (intensity * (num_chars - 1)) / 255;
      fprintf(asciifile, "%c", gradient[gradient_index]);

      ascii_colors[counter_z * 3] = r;
      ascii_colors[counter_z * 3 + 1] = g;
      ascii_colors[counter_z * 3 + 2] = b;
      counter_z++;
    }
    fprintf(asciifile, "\n");
  }
  fclose(asciifile);
  return 0;
}

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

  // Load font file
  unsigned char *font_buffer;
  long font_size;
  FILE *font_file = fopen("MesloLGS-NF-Bold.ttf", "rb");
  if (!font_file) {
    printf("Error: Failed to load font file\n");
    free(pixels);
    return 1;
  }

  fseek(font_file, 0, SEEK_END);
  font_size = ftell(font_file);
  fseek(font_file, 0, SEEK_SET);

  font_buffer = malloc(font_size);
  if (!font_buffer) {
    printf("Error: Failed to allocate font buffer\n");
    fclose(font_file);
    free(pixels);
    return 1;
  }

  if (fread(font_buffer, 1, font_size, font_file) != font_size) {
    printf("Error: Failed to read font file\n");
    fclose(font_file);
    free(font_buffer);
    free(pixels);
    return 1;
  }
  fclose(font_file);

  // Initialize font
  stbtt_fontinfo font;
  if (!stbtt_InitFont(&font, font_buffer, 0)) {
    printf("Error: Failed to initialize font\n");
    free(font_buffer);
    free(pixels);
    return 1;
  }

  // Read ASCII content
  char *fcontent = NULL;
  long fsize = 0;
  FILE *fp = fopen(output_filename, "r");
  if (fp) {
    fseek(fp, 0, SEEK_END);
    fsize = ftell(fp);
    rewind(fp);

    fcontent = (char *)malloc(fsize + 1);
    if (!fcontent) {
      printf("Error: Failed to allocate content buffer\n");
      fclose(fp);
      free(font_buffer);
      free(pixels);
      return 1;
    }

    if (fread(fcontent, 1, fsize, fp) != fsize) {
      printf("Error: Failed to read content file\n");
      fclose(fp);
      free(fcontent);
      free(font_buffer);
      free(pixels);
      return 1;
    }
    fcontent[fsize] = '\0';
    fclose(fp);
  } else {
    printf("Error: Failed to open text file\n");
    free(font_buffer);
    free(pixels);
    return 1;
  }

  // Render text
  const char *text = fcontent;
  float scale = stbtt_ScaleForPixelHeight(&font, char_h);
  int x = 0, y = 0;

  int ascent, descent, line_gap;
  stbtt_GetFontVMetrics(&font, &ascent, &descent, &line_gap);
  y += (int)(ascent * scale);

  char render_gradient[] = {'@', 'M', 'W', 'B', 'R', 'N', 'Q', 'D',
                            'G', 'O', 'U', 'S', 'V', 'Z', ':', ' '};
  int counter = 0;
  for (const char *c = text; *c; c++) {
    if (*c == '\n') {
      x = 0;
      y += (int)((ascent - descent + line_gap) * scale);
      continue;
    }

    int advance, lsb, x0, y0, x1, y1;
    char render_char = ' ';

    switch (*c) {
    case '@':
      render_char = render_gradient[0];
      break;
    case '&':
      render_char = render_gradient[1];
      break;
    case '%':
      render_char = render_gradient[2];
      break;
    case '#':
      render_char = render_gradient[3];
      break;
    case '*':
      render_char = render_gradient[4];
      break;
    case '+':
      render_char = render_gradient[5];
      break;
    case '~':
      render_char = render_gradient[6];
      break;
    case '=':
      render_char = render_gradient[7];
      break;
    case '_':
      render_char = render_gradient[8];
      break;
    case '-':
      render_char = render_gradient[9];
      break;
    case ';':
      render_char = render_gradient[10];
      break;
    case ':':
      render_char = render_gradient[11];
      break;
    case '`':
      render_char = render_gradient[12];
      break;
    case '\'':
      render_char = render_gradient[13];
      break;
    case '.':
      render_char = render_gradient[14];
      break;
    case ' ':
      render_char = render_gradient[15];
      break;
    default:
      printf("index error");
      return EXIT_FAILURE;
      break;
    }

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

/**
 * @brief Main program entry point
 * @param argc Argument count
 * @param argv Argument vector
 * @return Program exit status
 */
int main(int argc, char **argv) {
  // Initialize variables
  int out_h = 0, out_w = 0, reduct = 0;
  int img_width = 0, img_height = 0, img_bpp = 0;
  char *input_file = NULL, *output_file = NULL;
  uint8_t *rgb_image = NULL;
  unsigned char *ascii_colors = NULL;
  int verbose = 0, help_flag = 0, render = 0, auto_flag = 0;

  // Define command line options
  static struct option long_options[] = {{"input", required_argument, 0, 'i'},
                                         {"output", required_argument, 0, 'o'},
                                         {"verbose", no_argument, 0, 'v'},
                                         {"auto", no_argument, 0, 'a'},
                                         {"help", no_argument, 0, 'h'},
                                         {"renderimage", no_argument, 0, 'r'},
                                         {0, 0, 0, 0}};

  // Parse command line arguments
  int opt;
  while ((opt = getopt_long(argc, argv, "i:o:vhra", long_options, NULL)) !=
         -1) {
    switch (opt) {
    case 'i':
      input_file = optarg;
      break;
    case 'o':
      output_file = optarg;
      break;
    case 'a':
      reduct = 6;
      auto_flag = 1;
      break;
    case 'v':
      verbose = 1;
      break;
    case 'r':
      render = 1;
      break;
    case 'h':
      help_flag = 1;
      break;
    case '?':
      printf("Usage:\n");
      printf("  $ ascii-parser filename.png output.txt 300 200 --render\n");
      printf("  $ ascii-parser filename.png output.txt 40 --render\n");
      printf("Flags:\n");
      printf(" -r, --renderimage : Generate colored PNG of ASCII art\n");
      return 1;
    default:
      fprintf(stderr, "Argument processing error\n");
      exit(EXIT_FAILURE);
    }
  }

  // Show help if requested
  if (help_flag) {
    printf("Usage: %s [OPTIONS]... [FILES]...\n", argv[0]);
    printf("Generate ASCII files/images from input images\n\n");
    printf("Options:\n");
    printf("  -i, --input=file      Input image file\n");
    printf("  -o, --output=file     Output file (txt/png)\n");
    printf("  -v, --verbose         Show detailed information\n");
    printf("  -a, --auto            Reduce image to 6%% of original size\n");
    printf("  -r, --render          Generate PNG image of ASCII text\n");
    printf("  -h, --help            Show this help message\n");
    exit(EXIT_SUCCESS);
  }

  // Validate input file
  if (!input_file) {
    fprintf(stderr, "Error: Input file required (-i)\n");
    exit(EXIT_FAILURE);
  }

  // Load image info
  if (!stbi_info(input_file, &img_width, &img_height, &img_bpp)) {
    printf("Error: Unsupported image format\n");
    return EXIT_FAILURE;
  }

  // Load image data
  rgb_image = stbi_load(input_file, &img_width, &img_height, &img_bpp, 3);
  if (verbose) {
    printf("Image dimensions: %dx%d, BPP: %d\n", img_width, img_height,
           img_bpp);
  }

  // Calculate output dimensions
  if (auto_flag) {
    if (reduct < 0 || reduct > 10) {
      stbi_image_free(rgb_image);
      printf("Error: Reduction percentage must be between 20-70\n");
      return EXIT_FAILURE;
    }
    out_h = img_height * reduct / 100;
    out_w = img_width * reduct / 100;
  } else {
    if (argc <= optind + 1) {
      printf("Error: Output dimensions required\n");
      return EXIT_FAILURE;
    }
    out_h = atoi(argv[optind]);
    out_w = atoi(argv[optind + 1]);

    if (out_w > img_width || out_h > img_height) {
      printf("Error: Output dimensions exceed image size\n");
      return EXIT_FAILURE;
    }
  }

  if (!out_w || !out_h) {
    stbi_image_free(rgb_image);
    printf("Error: Invalid output dimensions\n");
    return EXIT_FAILURE;
  }

  if (verbose) {
    printf("Output: %dx%d characters\n", out_h, out_w);
    printf("Sampling steps: %dx%d\n", img_width / out_w, img_height / out_h);
  }

  // Process image
  ascii_colors = malloc((out_h + 5) * (out_w + 5) * 3);
  if (!parse2file(output_file, rgb_image, img_width, img_height,
                  img_width / out_w, img_height / out_h, img_bpp, out_w, out_h,
                  ascii_colors)) {
    printf("ASCII conversion complete: %s\n", output_file);

    if (render) {
      renderAsciiPNG(output_file, rgb_image, img_width, img_height,
                     img_width / out_w, img_height / out_h, img_bpp, out_w,
                     out_h, ascii_colors);
      printf("PNG rendering complete\n");
    }
    free(ascii_colors);
  } else {
    printf("Error during ASCII conversion\n");
    free(ascii_colors);
    return EXIT_FAILURE;
  }

  // Cleanup
  stbi_image_free(rgb_image);
  return EXIT_SUCCESS;
}
