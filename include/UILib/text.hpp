#pragma once
#include "UILib/util.hpp"
#include "element.hpp"
#include <SFML/Graphics/Font.hpp>
#include <SFML/Graphics/Text.hpp>
#include <string>
namespace ui {
/// Draws text.
class Text : public Element {
private:
  sf::Text createTextObject();

public:
  sf::Font font;
  std::string text;
  unsigned int fontSize = 24;
  ui::Color textColor = Color(0, 0, 0);
  void drawToWindow(sf::RenderWindow *window,
                    ElementRenderContext context) override;
  sf::Vector2f recalculateMinimumSize() override;

  Text(sf::Font font, std::string text) : font(font), text(text) {};
  Text(std::initializer_list<Element *> children) {
    this->children = children;
  };
  ~Text() {};
};
} // namespace ui
