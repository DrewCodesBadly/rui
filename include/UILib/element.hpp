#pragma once
#include "shapes.hpp"
#include "util.hpp"
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/System/Vector2.hpp>
#include <vector>

namespace ui {
class ElementRenderContext {
public:
  sf::Vector2u topLeft;
  sf::Vector2u size;
};

class Element {
private:
protected:
  sf::Vector2u minSize = sf::Vector2u(0, 0);

public:
  ui::BackgroundSource background = EmptyBackground();
  ui::RectBorders inset;
  ui::Shape shape = RectShape();
  std::vector<Element> children;
  Direction childArrangeDirection = Direction::Horizontal;
  // Determines the proportion of leftover space given to this element.
  unsigned int vertialSpacePriority = 0;
  unsigned int horizontalSpacePriority = 0;

  void drawToWindow(sf::RenderWindow window, ElementRenderContext context);

  sf::Vector2u getMiniminumSize();
};
} // namespace ui
