#pragma once
#include <SFML/Graphics/Color.hpp>
#include <cstdint>

namespace ui {
class RectBorders {
public:
  float left = 0;
  float top = 0;
  float right = 0;
  float bottom = 0;
  RectBorders() {};
  RectBorders(float left, float top, float right, float bottom);
  RectBorders(float horizontal, float veritcal)
      : left(horizontal), top(veritcal), right(horizontal), bottom(veritcal) {};
  RectBorders(float allSides)
      : left(allSides), right(allSides), top(allSides), bottom(allSides) {};
};

class BackgroundSource {};
class EmptyBackground : public BackgroundSource {
public:
};
class ColorBackground : public BackgroundSource {
public:
  sf::Color color;
  ColorBackground(sf::Color color) : color(color) {};
};

class Color {
public:
  uint8_t r;
  uint8_t g;
  uint8_t b;
  uint8_t a = 255;
  Color(uint8_t r, uint8_t g, uint8_t b, uint8_t a = 255)
      : r(r), g(g), b(b), a(a) {};
};

enum Direction { Vertical, Horizontal };

} // namespace ui
