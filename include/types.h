#ifndef TYPES_H
#define TYPES_H
#include <gtk/gtk.h>

typedef struct {
  uint8_t r;
  uint8_t g;
  uint8_t b;
} RGB;

typedef struct {
  GtkApplication *app;
  GtkWindow *active_win;
  GtkColorDialogButton *color_btn;
  GtkBox *manual_sizing_box;
  GtkBox *percent_sizing_box;
  char *selected_font;
  char *input_filepath;
  char *output_filepath;
  char *output_text_filepath;

  RGB *bg_color;
  bool manual_sizing_enabled;

  int out_h, out_w; // output size w*h, in chars
  int reduct;       // reduct percent, %6 by default
  int img_w, img_h; // input image size w*h, in pixels
  int img_bpp;      // number of channels in the image

  unsigned char *ascii_colors;
  uint8_t *rgb_image;
} AppData;
#endif // !TYPES
