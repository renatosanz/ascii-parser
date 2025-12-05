#ifndef ASCII_GTK
#define ASCII_GTK
#include "gtk/gtk.h"
#include "types.h"

void load_actions(AppData *app);
int lauch_processing_window(char *filepath);
void select_font_action(GtkDropDown *drop, GParamSpec *pspec,
                        AppData *app_data);
void select_background_action(GtkColorDialogButton *color_btn,
                              GParamSpec *pspec, AppData *app_data);
void handle_percent_sliding(GtkRange *self, AppData *app_data);
#endif // !ASCII_GTK
