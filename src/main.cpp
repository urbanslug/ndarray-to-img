#include "example.h"
#include <cstddef>
#include <cstdint>
#include <iostream>
#include <tuple>
#include <vector>

using namespace std;

extern "C" void show_vector(float const *data, std::size_t size);

extern "C" void show_matrix(float const *data, std::size_t nrow, std::size_t ncol);



typedef std::tuple<uint8_t, uint8_t, uint8_t,
                   uint8_t>
    color_t;

typedef std::tuple<uint32_t, uint32_t> position_t;

struct cell {
	int value;
	// RGB and alpha channel
	color_t color;
};

struct pos_t {
  uint32_t x;
  uint32_t y;

  pos_t(uint32_t x_pos, uint32_t y_pos) : x(x_pos), y(y_pos) {}
};

struct cell_p {
	position_t pos;
  int value;
  // RGB and alpha channel
  color_t color;
};

extern "C" void read_cells(cell const *cell, std::size_t nrow,
                            std::size_t ncol);



void do_vectors() {
  std::vector<float> vec{10, 20, 30};
  std::vector<float> vec2{40, 50, 60};
  std::vector<vector<float>> matrix{vec, vec2, vec};

  // linearize the matrix
  std::vector<float> matrix2;
  for (auto it = matrix.begin(); it != matrix.end(); it++) {
    for (auto t = it->begin(); t != it->end(); t++) {
      matrix2.push_back(*t);
    }
  }

  show_matrix(matrix[0].data(), 3, 3);
  show_matrix(matrix2.data(), 3, 3);
  // show_vector(vec.data(), vec.size());
}

void do_cell() {
  // ---
  // cell stuff
  // ---
  color_t black = std::make_tuple(0, 0, 0, 0);
  cell c1 = cell{10, black};

  color_t white = std::make_tuple(255, 255, 255, 0);
  cell c2 = cell{10, white};

  cout << "cpp Size of cell : " << sizeof(cell) << endl;

  std::vector<cell> col1{c1};
  std::vector<cell> col2{c2};

  std::vector<std::vector<cell>> matrix3{col1, col2};

  // linearize the matrix
  std::vector<cell> matrix4;
  for (auto it = matrix3.begin(); it != matrix3.end(); it++) {
    for (auto t = it->begin(); t != it->end(); t++) {
      matrix4.push_back(*t);
    }
  }

  read_cells(matrix4.data(), 2, 1);
}

extern "C" void read_cells_with_position(cell_p const *cell, std::size_t size);

void do_cell_with_position() {
  color_t black = std::make_tuple(0, 0, 0, 0);
  position_t up = std::make_tuple(20, 10);
  cell_p c1 = cell_p{up, 10, black};

  color_t white = std::make_tuple(255, 255, 255, 0);
  position_t down = std::make_tuple(30, 50);
  cell_p c2 = cell_p{down, 10, white};

  std::vector<cell_p> v{c1, c2};

  read_cells_with_position(v.data(), 2);
}

int main() {

	do_cell_with_position();

	pos_t pos_t(2,3);
}
