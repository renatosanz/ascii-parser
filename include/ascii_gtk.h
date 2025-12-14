#ifndef ASCII_GTK
#define ASCII_GTK
#include "gtk/gtk.h"
#include "types.h"

void load_actions(AppData *app);
void lauch_processing_window(char *filepath);
void select_font_action(GtkDropDown *drop, GParamSpec *pspec,
                        AppData *app_data);
void select_background_action(GtkColorDialogButton *color_btn,
                              GParamSpec *pspec, AppData *app_data);
void handle_manual_entry_height(GtkEntry *self, AppData *app_data);
void handle_manual_entry_width(GtkEntry *self, AppData *app_data);
void handle_percent_sliding(GtkRange *self, AppData *app_data);
void update_loading_modal_to_rendering(LoadingModal *data);
void update_loading_modal_to_finish(LoadingModal *data, char *output_file);
#endif // !ASCII_GTK
