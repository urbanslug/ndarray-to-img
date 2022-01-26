#include <cstdint>
#ifndef _EXAMPLE_H
#define _EXAMPLE_H


struct position_t {
  uint32_t x;
  uint32_t y;

  position_t(uint32_t x_pos, uint32_t y_pos) : x(x_pos), y(y_pos) {}
};

struct color_t {
  uint8_t red;
  uint8_t green;
  uint8_t blue;
  uint8_t alpha;

  color_t(uint8_t r, uint8_t g, uint8_t b, uint8_t a)
      : red(r), green(g), blue(b), alpha(a) {}
};

struct cell_t {
  position_t pos;
  int value;
  // RGB and alpha channel
  color_t color;
};

#ifdef __cplusplus
extern "C"{
#endif

void read_cells(cell_t const *cell, std::size_t length,
                           std::size_t nrow, std::size_t ncol);

int double_input(int input);

#ifdef __cplusplus
}
#endif
#endif
