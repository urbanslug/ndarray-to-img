#include "ndarray_to_img.h"

void do_cells() {
  color_t black (0, 0, 0, 0);
  position_t up (20, 10);
  cell_t c1 = cell_t{up, 10, black};

  color_t white (255, 255, 255, 0);
  position_t down (30, 50);
  cell_t c2 = cell_t{down, 10, white};

  std::vector<cell_t> v = {c1, c2};

  read_cells(v.data(), 2, 100, 100);
}

int main() {
	do_cells();

}
