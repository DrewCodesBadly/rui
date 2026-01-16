#pragma once
#include "element.hpp"
#include "util.hpp"
#include <SFML/Graphics/RenderWindow.hpp>
#include <initializer_list>

namespace ui {
class Box : public Element {

public:
  ui::Color background = ui::Color(0, 0, 0, 255);

  void drawToWindow(sf::RenderWindow *window,
                    ElementRenderContext context) override;
  Box *withBackground(ui::Color color);

  Box() {};
  Box(std::initializer_list<Element *> children) { this->children = children; };
  ~Box() {};
};
} // namespace ui
