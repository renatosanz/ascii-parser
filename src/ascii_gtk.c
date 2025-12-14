#include "gdk/gdk.h"
#include "glib-object.h"
#include "glib.h"
#include "gtk/gtk.h"
#include "gtk/gtkdropdown.h"
#include "logic.h"
#include "render.h"
#include "stb/stb_image.h"
#include "stb/stb_image_write.h"
#include "types.h"
#include <ascii_gtk.h>
#include <bits/pthreadtypes.h>
#include <inttypes.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

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

void set_output_size_label(AppData *app_data) {
  printf("output values: %d x %d\n", app_data->out_h, app_data->out_w);
  gtk_label_set_text(app_data->label_show_output_size,
                     g_strdup_printf("Output size: %dx%d (chars)",
                                     app_data->out_h, app_data->out_w));
}

void handle_manual_entry_height(GtkEntry *height_entry, AppData *app_data) {
  const char *value = gtk_editable_get_text(GTK_EDITABLE(height_entry));

  if (strlen(value) == 0 || strlen(value) > 5) {
    return;
  }

  int regex_val = regexec(&app_data->decimal_regex, value, 0, NULL, 0);
  if (regex_val == REG_NOMATCH) {
    return;
  }

  int int_value = (int)atoi(value);
  if (int_value > app_data->max_out_h || int_value < app_data->min_out_h) {
    return;
  }

  printf("entry height %d\n", int_value);
  app_data->out_h = int_value;
  set_output_size_label(app_data);
}

void handle_manual_entry_width(GtkEntry *width_entry, AppData *app_data) {
  const char *value = gtk_editable_get_text(GTK_EDITABLE(width_entry));
  if (strlen(value) == 0 || strlen(value) > 5) { // verify lenght
    return;
  }

  int regex_val = regexec(&app_data->decimal_regex, value, 0, NULL, 0);
  if (regex_val == REG_NOMATCH) {
    return;
  }

  int int_value = (int)atoi(value);
  if (int_value > app_data->max_out_w || int_value < app_data->min_out_w) {
    return;
  }

  printf("entry width%d\n", int_value);
  app_data->out_w = int_value;
  set_output_size_label(app_data);
}

void handle_percent_sliding(GtkRange *self, AppData *app_data) {
  float scaling_percent = gtk_range_get_value(self);
  printf("slide value: %f \n", scaling_percent);
  app_data->out_h = (int)(app_data->img_h * scaling_percent / 100);
  app_data->out_w = (int)(app_data->img_w * scaling_percent / 100) * 2;
  set_output_size_label(app_data);
}

void toggle_manual_sizing(GSimpleAction *action, GVariant *parameter,
                          AppData *app_data) {
  app_data->manual_sizing_enabled = !app_data->manual_sizing_enabled;
  gtk_widget_set_sensitive(GTK_WIDGET(app_data->percent_sizing_box),
                           !app_data->manual_sizing_enabled);
  gtk_widget_set_sensitive(GTK_WIDGET(app_data->manual_sizing_box),
                           app_data->manual_sizing_enabled);
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

void stop_processing(GtkButton *btn, AppData *window) {
  printf("TODO: free resources and stuff, then close\n");
}

void accept_result(GtkButton *btn, AppData *app_data) {
  gtk_window_close(app_data->loading_modal->window);
}

void open_output_file(GtkButton *btn, AppData *app_data) {
  pid_t pid = fork();
  if (pid == 0) {
    execlp("xdg-open", "xdg-open", app_data->output_filepath, NULL);
  } else if (pid < 0) {
    perror("failed fork to open image viewer");
  }
}

void open_loading_modal(AppData *app_data) {
  GtkBuilder *builder = gtk_builder_new();
  gtk_builder_add_from_resource(
      builder, "/org/asciiparser/data/ui/loading_modal.ui", NULL);
  app_data->loading_modal->window =
      GTK_WINDOW(gtk_builder_get_object(builder, "loading_modal"));
  gtk_window_set_modal(app_data->loading_modal->window, true);
  gtk_application_add_window(GTK_APPLICATION(app_data->app),
                             GTK_WINDOW(app_data->loading_modal->window));
  // set img_placeholder
  app_data->loading_modal->output_thumbail =
      GTK_PICTURE(gtk_builder_get_object(builder, "output_img"));
  gtk_picture_set_resource(app_data->loading_modal->output_thumbail,
                           "/org/asciiparser/data/icons/img_placeholder.png");

  app_data->loading_modal->cancel_btn =
      GTK_BUTTON(gtk_builder_get_object(builder, "cancel_btn"));
  g_signal_connect(GTK_WIDGET(app_data->loading_modal->cancel_btn), "clicked",
                   G_CALLBACK(stop_processing), app_data);

  app_data->loading_modal->accept_btn =
      GTK_BUTTON(gtk_builder_get_object(builder, "accept_btn"));
  g_signal_connect(GTK_WIDGET(app_data->loading_modal->accept_btn), "clicked",
                   G_CALLBACK(accept_result), app_data);

  app_data->loading_modal->open_output_btn =
      GTK_BUTTON(gtk_builder_get_object(builder, "open_output_btn"));
  g_signal_connect(GTK_WIDGET(app_data->loading_modal->open_output_btn),
                   "clicked", G_CALLBACK(open_output_file), app_data);

  app_data->loading_modal->label =
      GTK_LABEL(gtk_builder_get_object(builder, "progress_label"));
  app_data->loading_modal->spinner =
      GTK_SPINNER(gtk_builder_get_object(builder, "spinner"));

  app_data->loading_modal->progress_bar =
      GTK_PROGRESS_BAR(gtk_builder_get_object(builder, "progress_bar"));
  gtk_progress_bar_set_fraction(app_data->loading_modal->progress_bar, 0);
  g_object_unref(builder);

  gtk_widget_set_visible(GTK_WIDGET(app_data->loading_modal->window), true);
  gtk_window_present(GTK_WINDOW(app_data->loading_modal->window));
}

void update_loading_modal_to_rendering(LoadingModal *data) {
  gtk_label_set_text(data->label, "rendering (2/2)...");
}

void update_loading_modal_to_finish(LoadingModal *data, char *output_file) {
  gtk_label_set_text(data->label, "finished!");
  gtk_widget_set_visible(GTK_WIDGET(data->spinner), false);
  gtk_widget_set_visible(GTK_WIDGET(data->cancel_btn), false);
  gtk_widget_set_visible(GTK_WIDGET(data->accept_btn), true);
  gtk_widget_set_visible(GTK_WIDGET(data->open_output_btn), true);
  gtk_picture_set_file(GTK_PICTURE(data->output_thumbail),
                       g_file_new_for_path(output_file));
}

void select_font_action(GtkDropDown *drop, GParamSpec *pspec,
                        AppData *app_data) {
  app_data->selected_font = (char *)gtk_string_object_get_string(
      GTK_STRING_OBJECT(gtk_drop_down_get_selected_item(GTK_DROP_DOWN(drop))));
  g_print("%s", app_data->selected_font);
}

void init_image_loading(GtkButton *btn, GParamSpec *pspec, AppData *app_data) {

  open_loading_modal(app_data);

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
      g_strdup_printf("%s.txt.png", app_data->input_filepath);
  app_data->total_chars = (app_data->out_h) * (app_data->out_w);
  app_data->ascii_colors =
      malloc((app_data->out_h + 5) * (app_data->out_w + 5) * 3);
  // create a thread to speed up the processing
  pthread_t t_bg;
  pthread_create(&t_bg, NULL, start_on_background, (void *)app_data);
  pthread_detach(t_bg);
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
                   G_CALLBACK(init_image_loading), app_data);

  GActionMap *action_map = G_ACTION_MAP(app_data->app);
  g_action_map_add_action(action_map, G_ACTION(open_file_action_obj));
  g_action_map_add_action(action_map, G_ACTION(select_font_action_obj));
  g_action_map_add_action(action_map, G_ACTION(init_process_action_obj));
  g_action_map_add_action(action_map,
                          G_ACTION(toggle_manual_sizing_action_obj));
}
