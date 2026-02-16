#include "../util.cpp"
#include "UILib/element.hpp"
#include "UILib/listeners.hpp"
#include "UILib/shapes.hpp"
#include "UILib/util.hpp"
#include <SFML/Graphics/RectangleShape.hpp>
#include <UILib/widgets/buttons.hpp>
#include <initializer_list>
#include <iostream>

bool ui::Button::onLeftClickReleased() {
  activeColor = hoverColor;
  return onPressed();
}
bool ui::Button::onLeftClickPressed() {
  activeColor = pressedColor;
  return true;
}
bool ui::Button::onMouseEntered() {
  activeColor = hoverColor;
  return true;
}
bool ui::Button::onMouseExited() {
  activeColor = color;
  return true;
}

ui::Button::Button(ui::Color color, std::function<bool()> onPressed,
                   std::initializer_list<Element *> children)
    : color(color), onPressed(onPressed), activeColor(color) {
  this->children = children;
}

void ui::Button::drawToWindow(ui::ElementRenderContext context) {
  MouseListener::drawToWindow(context);
  sf::RectangleShape shape;
  shape.setSize(sf::Vector2f(context.size.x, context.size.y));
  shape.setPosition(sf::Vector2f(context.topLeft.x, context.topLeft.y));
  shape.setFillColor(toSFColor(activeColor));
  context.window->draw(shape);
}
