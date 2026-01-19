#pragma once
#include "util.hpp"
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/System/Vector2.hpp>
#include <initializer_list>
#include <optional>
#include <vector>

namespace ui {
enum PerpendicularSizing { Shrink, Stretch, Expand };

class ElementRenderContext {
public:
  sf::Vector2f topLeft;
  sf::Vector2f size;
};

/// Base class for UI elements.
class Element {
protected:
  std::optional<sf::Vector2f> cachedMinSize;

  void clearCachedMinSize();

public:
  sf::Vector2f minSize = sf::Vector2f(0., 0.);
  ui::RectBorders inset;
  std::vector<Element *> children;
  bool verticalChildren = false;
  // Alignment of children along the parallel axis.
  float childParallelAlign = 0.5;
  float childPerpendicularAlign = 0.5;

  // Info for placement inside of its parent.

  // Determines the proportion of leftover space given to this element.
  unsigned int spacePriority = 0;
  PerpendicularSizing perpendicularSizing = PerpendicularSizing::Shrink;

  // Determines this widget's alignment within allotted space
  // on a scale of [0,1], like UV coordinates.
  float perpendicularAlign = 0.5;

  // This method is separate so it can be overridden easily by other subclasses.
  virtual void drawToWindow(sf::RenderWindow *window,
                            ElementRenderContext context);
  void renderChildren(sf::RenderWindow *window, ElementRenderContext context);

  sf::Vector2f getMiniminumSize();
  virtual sf::Vector2f recalculateMinimumSize();

  Element() {};
  Element(std::initializer_list<Element *> children) : children(children) {};

  virtual ~Element() {};
};
} // namespace ui
