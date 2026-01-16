#include "util.cpp"
#include <SFML/Graphics/CircleShape.hpp>
#include <SFML/Graphics/RectangleShape.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/System/Vector2.hpp>
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

void ui::Circle::drawToWindow(sf::RenderWindow *window,
                              ElementRenderContext context) {
  sf::CircleShape shape;
  shape.setFillColor(toSFColor(fillColor));
  if (context.size.x > context.size.y) {
    shape.setRadius(context.size.y * 0.5);
    shape.setPosition(
        context.topLeft +
        sf::Vector2f((context.size.x - context.size.y) * 0.5, 0.));
  } else {
    shape.setRadius(context.size.x * 0.5);
    shape.setPosition(
        context.topLeft +
        sf::Vector2f(0., (context.size.y - context.size.x) * 0.5));
  }
  window->draw(shape);
}
