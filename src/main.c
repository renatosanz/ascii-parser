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

#include "stb/stb_image.h"
#include "stb/stb_image_write.h"
#include "stb/stb_truetype.h"

#include <logic.h>
#include <render.h>

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
