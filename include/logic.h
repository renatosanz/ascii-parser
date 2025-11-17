#ifndef LOGIC_H
#define LOGIC_H

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <types.h>

/**
 * @brief Converts an RGB image to ASCII art and saves to file
 * @param output_filename Output file path
 * @param rgb_image Input image data
 * @param width Image width in pixels
 * @param height Image height in pixels
 * @param w_step Width sampling step
 * @param h_step Height sampling step
 * @param channels Number of color channels
 * @param ascii_colors Buffer to store color data
 * @return 0 on success, -1 on failure
 */
int parse2file(char *output_filename, uint8_t *rgb_image, int width, int height,
               int w_step, int h_step, int channels,
               unsigned char *ascii_colors);
#endif // !LOGIC_H
