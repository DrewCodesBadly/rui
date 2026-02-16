#include "../util.cpp"
#include "UILib/element.hpp"
#include "UILib/listeners.hpp"
#include "UILib/shapes.hpp"
#include "UILib/util.hpp"
#include <SFML/Graphics/RectangleShape.hpp>
#include <UILib/widgets/buttons.hpp>
#include <initializer_list>

bool ui::Button::onLeftClickReleased() { return onPressed(); }

ui::Button::Button(ui::Color color, std::function<bool()> onPressed,
                   std::initializer_list<Element *> children)
    : color(color), onPressed(onPressed) {
  this->children.push_back(&backgroundBox);
  this->children = children;
}

void ui::Button::drawToWindow(ui::ElementRenderContext context) {
  MouseListener::drawToWindow(context);
  sf::RectangleShape shape;
  shape.setSize(sf::Vector2f(context.size.x, context.size.y));
  shape.setPosition(sf::Vector2f(context.topLeft.x, context.topLeft.y));
  shape.setFillColor(toSFColor(color));
  context.window->draw(shape);
}
