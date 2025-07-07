#include "glib-object.h"
#include "glib.h"
#include <bits/getopt_core.h>
#include <getopt.h>
#include <gio/gio.h>
#include <regex.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "stb/stb_image.h"
#include "stb/stb_image_write.h"
#include "stb/stb_truetype.h"

#include <logic.h>
#include <render.h>
#include <utils.h>

// Global application and window references
// GtkApplication *app;
// GtkWindow *win;
//
// /**
//  * @brief Callback function for GTK application activation
//  * @param app The GTK application instance
//  * @param user_data User data passed to the callback
//  * @return Always returns 0
//  */
// static void *on_activate(GtkApplication *app, gpointer user_data) {
//   GtkBuilder *builder = gtk_builder_new();
//   gtk_builder_add_from_resource(builder, "/org/dec2bin/data/ui/dec2bin.ui",
//                                 NULL);
//
//   win = GTK_WINDOW(gtk_builder_get_object(builder, "dec2bin_win"));
//   g_object_unref(builder);
//
//   gtk_application_add_window(GTK_APPLICATION(app), GTK_WINDOW(win));
//   gtk_window_present(GTK_WINDOW(win));
//   return 0;
// }
//
// /**
//  * @brief Initializes and runs the GTK application
//  * @param argc Argument count
//  * @param argv Argument vector
//  * @return Application exit status
//  */
// int mainGtkVersion(int argc, char **argv) {
//   gtk_init();
//   app = gtk_application_new("org.riprtx.asciiParser",
//                             G_APPLICATION_DEFAULT_FLAGS);
//   gtk_window_set_default_icon_name("dec2bin");
//   g_signal_connect(app, "activate", G_CALLBACK(on_activate), NULL);
//   int status = g_application_run(G_APPLICATION(app), argc, argv);
//   g_object_unref(app);
//
//   return status;
// }

/**
 * @brief Main program entry point
 * @param argc Argument count
 * @param argv Argument vector
 * @return Program exit status
 */
int main(int argc, char **argv) {
  // int variables
  static int out_h = 0, out_w = 0; // output size w*h, in chars
  static int reduct = 0;           // reduct percent, %6 by default
  static int img_w = 0, img_h = 0; // input image size w*h, in pixels
  static int img_bpp = 0;          // number of channels in the image

  // int flags (verbose,help,render,auto)
  static int verb_flag = 0, help_flag = 0, rndr_flag = 0, auto_flag = 0;

  // char* variables (string/text)
  static char *input_file = NULL;  // input filename
  static char *output_file = NULL; // output filename
  static char font_family[] = "";  // font family for rendering ascii image
  static unsigned char *ascii_colors = NULL; // colors of chars for rendering
  static uint8_t *rgb_image = NULL;          // image rgb data
  static uint8_t bg_color_render =
      0; // bg color of the ascii render (black or white)

  // accepted arguments:
  // --input=<filename.jpg> or -i <filename.jpg>
  // --output=<output_filename.jpg> or -o <output_filename.jpg>
  // --render or -r
  // --help or -h
  // --auto or -a
  // --verbose or -v
  static struct option long_options[] = {{"input", required_argument, 0, 'i'},
                                         {"output", required_argument, 0, 'o'},
                                         {"render", no_argument, 0, 'r'},
                                         {"verbose", no_argument, 0, 'v'},
                                         {"auto", no_argument, 0, 'a'},
                                         {"help", no_argument, 0, 'h'},
                                         {0, 0, 0, 0}};

  int opt; // aux variable for args options
  while ((opt = getopt_long(argc, argv, "i:o:rvha", long_options, NULL)) !=
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
      verb_flag = 1;
      break;
    case 'r':
      rndr_flag = 1;
      break;
    case 'h':
      help_flag = 1;
      break;
    case '?':
      printf("Usage:\n");
      printf(
          "  $ ascii-parser -i filename.png -o output.txt 300 200 --render\n");
      printf("Flags:\n");
      printf(" -r, --render : Open a terminal menu to generate colored PNG of "
             "ASCII art\n");
      return 1;
    default:
      fprintf(stderr, "Argument processing error\n");
      exit(EXIT_FAILURE);
    }
  }

  // display help if requested
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

  // validate input file
  if (!input_file) {
    fprintf(stderr, "Error: Input file required (-i)\n");
    exit(EXIT_FAILURE);
  }

  // load image info
  if (!stbi_info(input_file, &img_w, &img_h, &img_bpp)) {
    printf("Error: Unsupported image format\n");
    return EXIT_FAILURE;
  }

  // load image data
  rgb_image = stbi_load(input_file, &img_w, &img_h, &img_bpp, 0);
  if (verb_flag) {
    printf("Image dimensions: %dx%d, BPP: %d\n", img_w, img_h, img_bpp);
  }

  // set & calculate output dimensions (for txt and png files)
  if (auto_flag) {
    // if auto_flag is enable set 6% of reduction by default
    if (reduct < 0 || reduct > 10) {
      stbi_image_free(rgb_image);
      printf("Error: Reduction percentage must be between 1%% - 10%%\n");
      return EXIT_FAILURE;
    }
    out_h = img_h * reduct / 100;
    out_w = img_w * reduct / 100 * 2;
  } else {
    // either, the user must supply a width and height
    // values that should be around the 2% and 15% of the input image, higher
    // values may cause bugs and crashes
    if (argc <= optind + 1) {
      printf("Error: Output dimensions required\n");
      return EXIT_FAILURE;
    }
    out_h = atoi(argv[optind]);
    out_w = atoi(argv[optind + 1]);

    if ((out_w > img_w * 0.25 || out_h > img_h * 0.25) ||
        (out_w < img_w * 0.01 || out_h < img_h * 0.01)) {
      printf("Error: Output dimensions exceed image size\n");
      printf(
          "Recommended values: 1%% - 25%% percent of the input image size.\n");
      return EXIT_FAILURE;
    }
  }

  // if there's no output size return EXIT_FAILURE
  if (!out_w || !out_h) {
    stbi_image_free(rgb_image);
    printf("Error: Invalid output dimensions\n");
    return EXIT_FAILURE;
  }

  if (verb_flag) {
    printf("Output: %dx%d characters\n", out_h, out_w);
    printf("Sampling steps: %dx%d\n", img_w / out_w, img_h / out_h);
  }

  // process the image
  // allocate enought space in memory to save the colors of each char
  ascii_colors = malloc((out_h + 5) * (out_w + 5) * 3);
  // if no issues happend while generating the text file, then finish
  if (!parse2file(output_file, rgb_image, img_w, img_h, img_w / out_w,
                  img_h / out_h, img_bpp, ascii_colors)) {
    printf("ASCII conversion complete: %s\n", output_file);
    // if rndr_flag is enable, then open a ncurses menu to select a font_family
    // on the gresources and a background color (black = 0 or white = 255)
    if (rndr_flag) {
      displayRenderMenu(&bg_color_render, font_family);
      renderAsciiPNG(output_file, out_w, out_h, ascii_colors, bg_color_render,
                     font_family);
      printf("PNG rendering complete\n");
    }
    free(ascii_colors);
    ascii_colors = NULL;
  } else {
    // if error then free all the memory and exit
    printf("Error during ASCII conversion\n");
    free(ascii_colors);
    ascii_colors = NULL;
    return EXIT_FAILURE;
  }

  stbi_image_free(rgb_image);
  return EXIT_SUCCESS;
}
