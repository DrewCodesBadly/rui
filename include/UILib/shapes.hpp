#pragma once
#include "element.hpp"
#include "util.hpp"
#include <SFML/Graphics/RenderWindow.hpp>
#include <initializer_list>

namespace ui {
/// Draws a box inside its available area.
class Box : public Element {

public:
  ui::Color background = ui::Color(0, 0, 0, 255);

  void drawToWindow(ElementRenderContext context) override;

  Box() {};
  Box(std::initializer_list<Element *> children) { this->children = children; };
  ~Box() {};
};

/// Draws a circle inside its available area.
class Circle : public Element {
public:
  ui::Color fillColor = ui::Color(0, 0, 0);
  void drawToWindow(ElementRenderContext context) override;

  Circle() {};
  Circle(std::initializer_list<Element *> children) {
    this->children = children;
  };
  ~Circle() {};
};

} // namespace ui
