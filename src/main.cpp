#include "example.h"
#include <cstddef>
#include <iostream>
#include <vector>

using namespace std;

extern "C" void show_vector(float const *data, std::size_t size);

extern "C" void show_matrix(float const *data, std::size_t nrow,
                            std::size_t ncol);

int main() {
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
