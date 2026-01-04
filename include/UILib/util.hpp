#pragma once
#include <SFML/Graphics/Color.hpp>
#include <cstdint>

namespace ui {
class RectBorders {
public:
  unsigned int left = 0;
  unsigned int top = 0;
  unsigned int right = 0;
  unsigned int bottom = 0;
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
