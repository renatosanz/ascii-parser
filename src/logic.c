#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include <logic.h>

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
               unsigned char *ascii_colors) {

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
      unsigned char r;
      unsigned char g;
      unsigned char b;
      if (rgb_image[index] && rgb_image[index + 1] && rgb_image[index + 2]) {
        r = rgb_image[index];
        g = rgb_image[index + 1];
        b = rgb_image[index + 2];
      } else {
        r = 0;
        g = 0;
        b = 0;
      }
      int intensity = (r + g + b) / 3;
      int gradient_index = (intensity * (num_chars - 1)) / 255;
      fprintf(asciifile, "%c", gradient[gradient_index]);

      ascii_colors[counter_z * 3] = r;
      ascii_colors[counter_z * 3 + 1] = g;
      ascii_colors[counter_z * 3 + 2] = b;
      counter_z++;
    }
    fprintf(asciifile, "\n");
  }
  fclose(asciifile);
  return 0;
}
