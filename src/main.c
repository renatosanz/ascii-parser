#include "glib-object.h"
#include "glib.h"
#include "stb/stb_image.h"
#include "stb/stb_image_write.h"
#include "stb/stb_truetype.h"
#include "types.h"
#include <ascii_gtk.h>
#include <bits/getopt_core.h>
#include <getopt.h>
#include <gio/gio.h>
#include <gtk/gtk.h>
#include <logic.h>
#include <regex.h>
#include <render.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Global application and window references
AppData *app_data;

static const char *font_options[] = {
    "CascadiaCodeNF-Bold.ttf",
    "CascadiaCodeNF-Regular.ttf",
    "FiraCode-Bold.ttf",
    "FiraCode-Regular.ttf",
    "Hack-Bold.ttf",
    "Hack-Regular.ttf",
    "JetBrainsMonoNL-Bold.ttf",
    "JetBrainsMonoNL-Regular.ttf",
};

static const int default_percent_value = 6;

static const RGB default_background_color = {255, 255, 255};

static void *on_activate(GtkApplication *app, gpointer user_data) {
  AppData *app_data = (AppData *)user_data;
  GtkBuilder *builder = gtk_builder_new();
  gtk_builder_add_from_resource(
      builder, "/org/asciiparser/data/ui/ascii-parser.ui", NULL);
  app_data->active_win =
      GTK_WINDOW(gtk_builder_get_object(builder, "ascii-parser-win"));
  g_object_unref(builder);

  load_actions(app_data);

  gtk_application_add_window(GTK_APPLICATION(app_data->app),
                             GTK_WINDOW(app_data->active_win));
  gtk_window_present(GTK_WINDOW(app_data->active_win));
  return 0;
}

int load_file_metadata(char *filepath, AppData *app_data) {
  if (!filepath) {
    fprintf(stderr, "Error: Input file required (-i)\n");
    return -1;
  }
  app_data->input_filepath = g_strdup_printf("%s", filepath);

  if (!stbi_info(app_data->input_filepath, &app_data->img_w, &app_data->img_h,
                 &app_data->img_bpp)) {
    printf("Error: Unsupported image format\n");
    return -1;
  }
  // extract image data
  app_data->rgb_image = stbi_load(app_data->input_filepath, &app_data->img_w,
                                  &app_data->img_h, &app_data->img_bpp, 0);
  return 0;
}

void init_globals(AppData *app_data, char *filepath) {
  app_data->input_filepath = g_strdup_printf("%s", filepath);
  app_data->selected_font = font_options[0];
  app_data->out_h = (int)(app_data->img_h * default_percent_value / 100);
  app_data->out_w = (int)(app_data->img_w * default_percent_value / 100);
  app_data->bg_color = g_new0(RGB, 1);
  app_data->bg_color->r = default_background_color.r;
  app_data->bg_color->g = default_background_color.g;
  app_data->bg_color->b = default_background_color.b;
}

int lauch_processing_window(char *filepath) {
  // stbi_set_flip_vertically_on_load(true);
  if (load_file_metadata(filepath, app_data)) {
    return EXIT_FAILURE;
  };

  if (app_data->active_win) {
    gtk_window_destroy(app_data->active_win);
  }

  init_globals(app_data, filepath);

  // create the builder
  GtkBuilder *builder = gtk_builder_new();
  gtk_builder_add_from_resource(
      builder, "/org/asciiparser/data/ui/process_win.ui", NULL);
  // get the window
  app_data->active_win =
      GTK_WINDOW(gtk_builder_get_object(builder, "process_win"));
  // filename label
  GtkLabel *filepath_label =
      GTK_LABEL(gtk_builder_get_object(builder, "filepath_label"));
  gtk_label_set_label(
      filepath_label,
      g_strdup_printf("Selected file: %s",
                      g_file_get_basename(g_file_new_for_path(filepath))));
  // font drop down
  GtkDropDown *drop =
      GTK_DROP_DOWN(gtk_builder_get_object(builder, "font_drop_down"));
  gtk_drop_down_set_model(drop,
                          G_LIST_MODEL(gtk_string_list_new(font_options)));
  g_signal_connect(GTK_WIDGET(drop), "notify::selected",
                   G_CALLBACK(select_font_action), app_data);
  // picture thumbnail
  gtk_picture_set_file(
      GTK_PICTURE(gtk_builder_get_object(builder, "selected_img")),
      g_file_new_for_path(filepath));
  // manual sizing zone
  app_data->manual_sizing_box =
      GTK_BOX(gtk_builder_get_object(builder, "manual_sizing_box"));
  gtk_widget_set_sensitive(GTK_WIDGET(app_data->manual_sizing_box),
                           app_data->manual_sizing_enabled);
  // output size label
  app_data->label_show_output_size =
      GTK_LABEL(gtk_builder_get_object(builder, "label_show_output_size"));
  gtk_label_set_text(app_data->label_show_output_size,
                     g_strdup_printf("Output size: %dx%d (chars)",
                                     app_data->out_h, app_data->out_w));
  // by percent
  app_data->percent_sizing_box =
      GTK_BOX(gtk_builder_get_object(builder, "percent_sizing_box"));
  // by percent (range)
  GtkScale *percent_scale = GTK_SCALE(
      gtk_scale_new_with_range(GTK_ORIENTATION_HORIZONTAL, 1, 12, 0.01));
  gtk_widget_set_hexpand(GTK_WIDGET(percent_scale), true);
  gtk_range_set_value(GTK_RANGE(percent_scale), default_percent_value);
  gtk_range_set_round_digits(GTK_RANGE(percent_scale), 1);
  gtk_scale_set_draw_value(GTK_SCALE(percent_scale), true);
  gtk_widget_set_sensitive(GTK_WIDGET(percent_scale),
                           !app_data->manual_sizing_enabled);
  g_signal_connect(GTK_RANGE(percent_scale), "value-changed",
                   G_CALLBACK(handle_percent_sliding), app_data);
  gtk_box_append(app_data->percent_sizing_box, GTK_WIDGET(percent_scale));

  // TODO: implement independient input (h x w)

  g_print("FLAAAAAAAAAAAG");

  // background color dialog
  GdkRGBA color;
  gdk_rgba_parse(&color, g_strdup_printf("rgba(%d, %d, %d, 1.0)",
                                         default_background_color.r,
                                         default_background_color.g,
                                         default_background_color.b));
  app_data->color_btn =
      GTK_COLOR_DIALOG_BUTTON(gtk_builder_get_object(builder, "color_btn"));
  gtk_color_dialog_button_set_rgba(app_data->color_btn, &color);
  g_signal_connect(GTK_WIDGET(app_data->color_btn), "notify::rgba",
                   G_CALLBACK(select_background_action), app_data);
  // drop the builder
  g_object_unref(builder);
  // show app
  gtk_application_add_window(GTK_APPLICATION(app_data->app),
                             GTK_WINDOW(app_data->active_win));
  gtk_window_present(GTK_WINDOW(app_data->active_win));
  return EXIT_SUCCESS;
}

void on_shutdown(GtkApplication *app, gpointer user_data) { g_free(app_data); }

int main(int argc, char **argv) {
  gtk_init();
  app_data = g_new0(AppData, 1);
  app_data->manual_sizing_enabled = false;
  app_data->bg_color = g_new0(RGB, 1);
  app_data->input_filepath = NULL;
  app_data->app = gtk_application_new("org.riprtx.asciiParser",
                                      G_APPLICATION_DEFAULT_FLAGS);
  g_signal_connect(app_data->app, "activate", G_CALLBACK(on_activate),
                   app_data);
  g_signal_connect(app_data->app, "shutdown", G_CALLBACK(on_shutdown),
                   app_data);
  int status = g_application_run(G_APPLICATION(app_data->app), argc, argv);
  g_object_unref(app_data->app);

  return status;
}

/**
 * @brief Main program entry point
 * @param argc Argument count
 * @param argv Argument vector
 * @return Program exit status
 *
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
  // bg color of the ascii render (black or white)
  static RGB bg_color_render = {0, 0, 0};

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

 */
