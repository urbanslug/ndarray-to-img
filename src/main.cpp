#include "example.h"
#include <cstddef>
#include <cstdint>
#include <iostream>
#include <tuple>
#include <vector>


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

extern "C" void read_cells(cell_t const *cell, std::size_t length,
                           std::size_t nrow, std::size_t ncol);


void do_cells() {
  color_t black (0, 0, 0, 0);
  position_t up (20, 10);
  cell_t c1 = cell_t{up, 10, black};

  color_t white (255, 255, 255, 0);
  position_t down (30, 50);
  cell_t c2 = cell_t{down, 10, white};

  std::vector<cell_t> v{c1, c2};

  read_cells(v.data(), 2, 1000, 1000);
}

int main() {
	do_cells();

}
