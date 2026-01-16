#include "util.cpp"
#include <SFML/Graphics/RectangleShape.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Window/Window.hpp>
#include <UILib/element.hpp>
#include <UILib/shapes.hpp>

void ui::Box::drawToWindow(sf::RenderWindow *window,
                           ui::ElementRenderContext context) {
  // Draws a rectangle in the window
  sf::RectangleShape shape;
  shape.setSize(sf::Vector2f(context.size.x, context.size.y));
  shape.setPosition(sf::Vector2f(context.topLeft.x, context.topLeft.y));
  shape.setFillColor(toSFColor(background));
  window->draw(shape);
}

ui::Box *ui::Box::withBackground(ui::Color color) {
  background = color;
  return this;
}
