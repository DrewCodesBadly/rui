#pragma once
#include "util.hpp"
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/System/Vector2.hpp>
#include <optional>
#include <vector>

namespace ui {
class ElementRenderContext {
public:
  sf::Vector2u topLeft;
  sf::Vector2u size;
};

/// Base class for UI elements. Draws a rectangle in its area.
class Element {
private:
protected:
  sf::Vector2u minSize = sf::Vector2u(0, 0);
  std::optional<sf::Vector2u> cachedMinSize;

  void clearCachedMinSize();

public:
  ui::BackgroundSource background = EmptyBackground();
  ui::RectBorders inset;
  std::vector<Element *> children;
  bool verticalChildren = false;

  // Info for placement inside of its parent.

  // Determines the proportion of leftover space given to this element.
  unsigned int vertialSpacePriority = 0;
  unsigned int horizontalSpacePriority = 0;
  // Determines this widget's alignment within allotted space
  // on a scale of [0,1], like UV coordinates.
  double halign = 0.5;
  double valign = 0.5;

  // This method is separate so it can be overridden easily by other subclasses.
  void drawToWindow(sf::RenderWindow *window, ElementRenderContext context);
  void renderChildren(sf::RenderWindow *window, ElementRenderContext context);

  sf::Vector2u getMiniminumSize();
  sf::Vector2u recalculateMinimumSize();
};
} // namespace ui
