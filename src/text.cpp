#include "util.cpp"
#include <SFML/Graphics/Text.hpp>
#include <SFML/System/Vector2.hpp>
#include <UILib/text.hpp>
void ui::Text::drawToWindow(sf::RenderWindow *window,
                            ElementRenderContext context) {
  sf::Text t = createTextObject();
  t.setPosition(context.topLeft - t.getLocalBounds().position);
  t.setFillColor(toSFColor(textColor));
  window->draw(t);
}

sf::Vector2f ui::Text::recalculateMinimumSize() {
  sf::Text t = createTextObject();
  sf::FloatRect bounds = t.getLocalBounds();

  return bounds.size;
}

sf::Text ui::Text::createTextObject() {
  sf::Text t(font);
  t.setString(text);
  t.setCharacterSize(fontSize);
  return t;
}
