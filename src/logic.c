#include "ascii_gtk.h"
#include "gtk/gtk.h"
#include "gtk/gtkshortcut.h"
#include "render.h"
#include "types.h"
#include <pthread.h>
#include <regex.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include <logic.h>
#include <unistd.h>

/**
 * @brief Converts an RGB image to ASCII art and saves to file
 * @param output_filename Output file path
 * @param rgb_image Input image data
 * @param width Image width in pixels
 * @param height Image height in pixels
 * @param w_step Width sampling step
 * @param h_step Height sampling step
 * @param channels Number of color channels
 * @param output_w Output width in characters
 * @param output_h Output height in characters
 * @param ascii_colors Buffer to store color data
 * @return 0 on success, -1 on failure
 */
int parse2file(char *output_filename, uint8_t *rgb_image, int width, int height,
               int w_step, int h_step, int channels,
               unsigned char *ascii_colors, GtkProgressBar *progress_bar,
               int total_chars, GtkWindow *dialog) {

  char gradient[] = {'@', '&', '%', '#', '*', '+',  '~', '=',
                     '_', '-', ';', ':', '`', '\'', '.', ' '};
  int num_chars = sizeof(gradient) / sizeof(gradient[0]);

  FILE *asciifile = fopen(output_filename, "w");
  if (!asciifile) {
    perror("Error opening file");
    return -1;
  }

  int counter_z = 0;
  for (int y = 0; y < height; y += h_step) {
    for (int x = 0; x < width; x += w_step) {
      size_t index = (y * width + x) * channels;
      unsigned char r = 0;
      unsigned char g = 0;
      unsigned char b = 0;

      if (rgb_image[index] && rgb_image[index + 1] && rgb_image[index + 2]) {
        r = rgb_image[index];
        g = rgb_image[index + 1];
        b = rgb_image[index + 2];
      }

      int intensity = (r + g + b) / 3;
      int gradient_index = (intensity * (num_chars - 1)) / 255;
      fprintf(asciifile, "%c", gradient[gradient_index]);

      ascii_colors[counter_z * 3] = r;
      ascii_colors[counter_z * 3 + 1] = g;
      ascii_colors[counter_z * 3 + 2] = b;
      counter_z++;
      gtk_progress_bar_set_fraction(progress_bar,
                                    (float)(counter_z / total_chars));
    }
    fprintf(asciifile, "\n");
  }
  fclose(asciifile);
  sleep(1);
  return 0;
}

/*
 * Start the parsing and rendering on a different thread
 * */
void *start_on_background(void *arg) {
  AppData *app_data = (AppData *)arg;
  // if no issues happend while generating the text file, then finish
  if (!parse2file(app_data->output_text_filepath, app_data->rgb_image,
                  app_data->img_w, app_data->img_h,
                  app_data->img_w / app_data->out_w,
                  app_data->img_h / app_data->out_h, app_data->img_bpp,
                  app_data->ascii_colors, app_data->loading_modal->progress_bar,
                  app_data->total_chars, app_data->loading_modal->window)) {
    printf("ASCII conversion complete: %s\n", app_data->output_text_filepath);
    // if rndr_flag is enable, then open a ncurses menu to select a font_family
    // on the gresources and a background color (black = 0 or white = 255)
    update_loading_modal_to_rendering(app_data->loading_modal);
    renderAsciiPNG(app_data->output_text_filepath, app_data->out_w,
                   app_data->out_h, app_data->ascii_colors, app_data->bg_color,
                   app_data->selected_font, app_data->loading_modal,
                   app_data->total_chars);
    update_loading_modal_to_finish(app_data->loading_modal,
                                   app_data->output_filepath);
    printf("PNG rendering complete\n");

    if (!app_data->create_ascii_file) {
      remove_ascii_file(app_data);
    }

    free(app_data->ascii_colors);
    app_data->ascii_colors = NULL;
  } else {
    // if error then free all the memory and exit
    printf("Error during ASCII conversion\n");
    free(app_data->ascii_colors);
    app_data->ascii_colors = NULL;
  }
  pthread_exit(NULL);
}
