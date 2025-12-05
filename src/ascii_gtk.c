#include "gdk/gdk.h"
#include "glib.h"
#include "gtk/gtk.h"
#include "gtk/gtkdropdown.h"
#include "logic.h"
#include "render.h"
#include "stb/stb_image.h"
#include "stb/stb_image_write.h"
#include "types.h"
#include <ascii_gtk.h>
#include <inttypes.h>
#include <stdio.h>

void file_dialog_response(GObject *source_object, GAsyncResult *result,
                          gpointer user_data) {
  (void)user_data;
  GtkFileDialog *dialog = GTK_FILE_DIALOG(source_object);
  GError *error = NULL;

  // Get selected file
  GFile *file = gtk_file_dialog_open_finish(dialog, result, &error);

  if (error) {
    g_printerr("File selection error: %s\n", error->message);
    g_error_free(error);
    return;
  }

  if (!file) {
    g_print("No file selected\n");
    return;
  }

  // Get file path
  char *filepath = g_file_get_path(file);
  if (!filepath) {
    g_object_unref(file);
    g_critical("Failed to get file path");
    return;
  }

  // g_print("path: %s\n", filepath);

  lauch_processing_window(filepath);

  // Clean up
  g_free(filepath);
  g_object_unref(file);
}

void handle_percent_sliding(GtkRange *self, AppData *app_data) {
  float scaling_percent = gtk_range_get_value(self);
  printf("slide value: %f \n", scaling_percent);
  app_data->out_h = (int)(app_data->img_h * scaling_percent / 100);
  app_data->out_w = (int)(app_data->img_w * scaling_percent / 100) * 2;
  printf("output values: %d x %d\n", app_data->out_h, app_data->out_w);
  gtk_label_set_text(app_data->label_show_output_size,
                     g_strdup_printf("Output size: %dx%d (chars)",
                                     app_data->out_h, app_data->out_w));
}

void open_new_file_dialog(GSimpleAction *action, GVariant *parameter,
                          AppData *app_data) {
  (void)action;
  (void)parameter;

  GtkWindow *window = GTK_WINDOW(app_data->active_win);
  GtkFileDialog *dialog = gtk_file_dialog_new();

  gtk_file_dialog_set_title(dialog, "Select an image");

  GtkFileFilter *filter = gtk_file_filter_new();
  gtk_file_filter_add_mime_type(filter, "image/*");
  gtk_file_dialog_set_default_filter(dialog, filter);

  gtk_file_dialog_open(dialog, window, NULL, file_dialog_response, NULL);
}

void toggle_manual_sizing(GSimpleAction *action, GVariant *parameter,
                          AppData *app_data) {
  app_data->manual_sizing_enabled = !app_data->manual_sizing_enabled;
  gtk_widget_set_sensitive(GTK_WIDGET(app_data->percent_sizing_box),
                           !app_data->manual_sizing_enabled);
  gtk_widget_set_sensitive(GTK_WIDGET(app_data->manual_sizing_box),
                           app_data->manual_sizing_enabled);
}

void select_font_action(GtkDropDown *drop, GParamSpec *pspec,
                        AppData *app_data) {
  app_data->selected_font = (char *)gtk_string_object_get_string(
      GTK_STRING_OBJECT(gtk_drop_down_get_selected_item(GTK_DROP_DOWN(drop))));

  g_print("%s", app_data->selected_font);
}

void init_image_processing(GtkButton *btn, GParamSpec *pspec,
                           AppData *app_data) {
  g_print("-----Starting Parsing process\n"
          "File: %s\n"
          "Background color RGB(%" PRIu8 ",%" PRIu8 ",%" PRIu8 ")\n"
          "Selected Font: %s\n"
          "Input size: %d x %d\n"
          "Output size: %d x %d\n",
          app_data->input_filepath,         /* %s (File) */
          app_data->bg_color->r,            /* %" PRIu8 " (R) */
          app_data->bg_color->g,            /* %" PRIu8 " (G) */
          app_data->bg_color->b,            /* %" PRIu8 " (B) */
          app_data->selected_font,          /* %s (Font) */
          app_data->img_w, app_data->img_h, /* %d x %d (Input Size) */
          app_data->out_w, app_data->out_h  /* %d x %d (Output Size) */
  );
  app_data->output_text_filepath =
      g_strdup_printf("%s.txt", app_data->input_filepath);
  app_data->output_filepath =
      g_strdup_printf("%s.txt.jpg", app_data->input_filepath);
  app_data->ascii_colors =
      malloc((app_data->out_h + 5) * (app_data->out_w + 5) * 3);

  printf("flagggggggggggggggggggg\n");

  // if no issues happend while generating the text file, then finish
  if (!parse2file(app_data->output_text_filepath, app_data->rgb_image,
                  app_data->img_w, app_data->img_h,
                  app_data->img_w / app_data->out_w,
                  app_data->img_h / app_data->out_h, app_data->img_bpp,
                  app_data->ascii_colors)) {
    printf("ASCII conversion complete: %s\n", app_data->output_text_filepath);
    // if rndr_flag is enable, then open a ncurses menu to select a font_family
    // on the gresources and a background color (black = 0 or white = 255)
    renderAsciiPNG(app_data->output_text_filepath, app_data->out_w,
                   app_data->out_h, app_data->ascii_colors, app_data->bg_color,
                   app_data->selected_font);
    printf("PNG rendering complete\n");
    free(app_data->ascii_colors);
    app_data->ascii_colors = NULL;
  } else {
    // if error then free all the memory and exit
    printf("Error during ASCII conversion\n");
    free(app_data->ascii_colors);
    app_data->ascii_colors = NULL;
  }

}

void select_background_action(GtkColorDialogButton *color_btn,
                              GParamSpec *pspec, AppData *app_data) {

  GdkRGBA *color = gtk_color_dialog_button_get_rgba(color_btn);
  g_print("r: %f, g: %f, b:%f\n", color->red, color->green, color->blue);
  app_data->bg_color->r = (uint8_t)(color->red * 255);
  app_data->bg_color->g = (uint8_t)(color->green * 255);
  app_data->bg_color->b = (uint8_t)(color->blue * 255);

  g_print("r: %" PRIu8 ", g: %" PRIu8 ", b:%" PRIu8 "\n",
          app_data->bg_color->r & 0xff, app_data->bg_color->g & 0xff,
          app_data->bg_color->b & 0xff);
}

void load_actions(AppData *app_data) {
  GSimpleAction *open_file_action_obj = g_simple_action_new("open-file", NULL);
  GSimpleAction *select_font_action_obj =
      g_simple_action_new("select-font", NULL);
  GSimpleAction *toggle_manual_sizing_action_obj =
      g_simple_action_new("toggle-manual-sizing", NULL);
  GSimpleAction *init_process_action_obj =
      g_simple_action_new("process-image", NULL);

  g_signal_connect(open_file_action_obj, "activate",
                   G_CALLBACK(open_new_file_dialog), app_data);
  g_signal_connect(toggle_manual_sizing_action_obj, "activate",
                   G_CALLBACK(toggle_manual_sizing), app_data);
  g_signal_connect(select_font_action_obj, "activate",
                   G_CALLBACK(select_font_action), app_data);
  g_signal_connect(init_process_action_obj, "activate",
                   G_CALLBACK(init_image_processing), app_data);

  GActionMap *action_map = G_ACTION_MAP(app_data->app);
  g_action_map_add_action(action_map, G_ACTION(open_file_action_obj));
  g_action_map_add_action(action_map, G_ACTION(select_font_action_obj));
  g_action_map_add_action(action_map, G_ACTION(init_process_action_obj));
  g_action_map_add_action(action_map,
                          G_ACTION(toggle_manual_sizing_action_obj));
}
