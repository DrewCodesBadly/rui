#pragma once
#include "shapes.hpp"
#include "util.hpp"
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/System/Vector2.hpp>
#include <vector>

namespace ui {
class ElementRenderConstraints {
public:
  sf::Vector2u topLeft;
  sf::Vector2u size;
};

class Element {
private:
public:
  sf::Vector2u minSize = sf::Vector2u(0, 0);
  ui::BackgroundSource background = EmptyBackground();
  ui::RectBorders inset;
  ui::Shape shape = RectShape();
  std::vector<Element> children;
  Direction childArrangeDirection = Direction::Horizontal;

  void drawToWindow(sf::RenderWindow window,
                    ElementRenderConstraints constraints);
};
} // namespace ui
