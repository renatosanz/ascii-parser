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

// Global application and window references
AppData *app_data;

static const char *const regex_dec = "^[0-9]+$";

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

static const int min_percent_value = 1;
static const int default_percent_value = 2;
static const int max_percent_value = 3;
static const float slider_step_size = 0.5;

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

  app_data->max_out_h = (int)(app_data->img_h * max_percent_value / 100);
  app_data->max_out_w = (int)(app_data->img_w * max_percent_value / 100) * 2;

  app_data->min_out_h = (int)(app_data->img_h * min_percent_value / 100);
  app_data->min_out_w = (int)(app_data->img_w * min_percent_value / 100);

  app_data->bg_color = g_new0(RGB, 1);
  app_data->bg_color->r = default_background_color.r;
  app_data->bg_color->g = default_background_color.g;
  app_data->bg_color->b = default_background_color.b;
}

void lauch_processing_window(char *filepath) {
  stbi_flip_vertically_on_write(1);

  if (load_file_metadata(filepath, app_data)) {
    return;
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
      gtk_scale_new_with_range(GTK_ORIENTATION_HORIZONTAL, min_percent_value,
                               max_percent_value, slider_step_size));
  gtk_widget_set_hexpand(GTK_WIDGET(percent_scale), true);
  gtk_range_set_value(GTK_RANGE(percent_scale), default_percent_value);
  gtk_range_set_round_digits(GTK_RANGE(percent_scale), 1);
  gtk_scale_set_draw_value(GTK_SCALE(percent_scale), true);
  gtk_widget_set_sensitive(GTK_WIDGET(percent_scale),
                           !app_data->manual_sizing_enabled);
  g_signal_connect(GTK_RANGE(percent_scale), "value-changed",
                   G_CALLBACK(handle_percent_sliding), app_data);
  gtk_box_append(app_data->percent_sizing_box, GTK_WIDGET(percent_scale));

  GtkEntry *height_entry =
      GTK_ENTRY(gtk_builder_get_object(builder, "height_entry"));
  g_signal_connect(GTK_WIDGET(height_entry), "changed",
                   G_CALLBACK(handle_manual_entry_height), app_data);

  GtkEntry *width_entry =
      GTK_ENTRY(gtk_builder_get_object(builder, "width_entry"));
  g_signal_connect(GTK_WIDGET(width_entry), "changed",
                   G_CALLBACK(handle_manual_entry_width), app_data);

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
}

void compile_decimal_regex(regex_t *regex) {
  regcomp(regex, regex_dec, REG_EXTENDED);
}

int main(int argc, char **argv) {
  gtk_init();

  app_data = g_new0(AppData, 1);
  app_data->loading_modal = g_new0(LoadingModal, 1);
  app_data->manual_sizing_enabled = false;
  app_data->bg_color = g_new0(RGB, 1);
  app_data->input_filepath = NULL;
  compile_decimal_regex(&app_data->decimal_regex);

  app_data->app = gtk_application_new("org.riprtx.asciiParser",
                                      G_APPLICATION_DEFAULT_FLAGS);
  g_signal_connect(app_data->app, "activate", G_CALLBACK(on_activate),
                   app_data);
  int status = g_application_run(G_APPLICATION(app_data->app), argc, argv);
  g_object_unref(app_data->app);

  return status;
}
