#ifndef TYPES_H
#define TYPES_H
#include <gtk/gtk.h>
#include <regex.h>

typedef struct {
  uint8_t r;
  uint8_t g;
  uint8_t b;
} RGB;

typedef struct {
  GtkWindow *window;
  GtkProgressBar *progress_bar;
  GtkLabel *label;
  GtkButton *cancel_btn;
  GtkButton *accept_btn;
  GtkButton *open_output_btn;
  GtkPicture *output_thumbail;
  GtkSpinner *spinner;
} LoadingModal;

typedef struct {
  GtkApplication *app;
  GtkWindow *active_win;
  GtkColorDialogButton *color_btn;
  GtkLabel *label_show_output_size;
  GtkBox *manual_sizing_box;
  GtkBox *percent_sizing_box;
  char *selected_font;
  char *input_filepath;
  char *output_filepath;
  char *output_text_filepath;

  LoadingModal *loading_modal;

  RGB *bg_color;
  bool manual_sizing_enabled;

  int out_h, out_w;         // output size w*h, in chars
  int max_out_h, max_out_w; // max output size w*h, in chars
  int min_out_h, min_out_w; // min output size w*h, in chars

  int reduct;       // reduct percent, %6 by default
  int img_w, img_h; // input image size w*h, in pixels
  int img_bpp;      // number of channels in the image
  int total_chars;

  unsigned char *ascii_colors;
  uint8_t *rgb_image;
  gboolean create_ascii_file;

  regex_t decimal_regex;
} AppData;
#endif // !TYPES
