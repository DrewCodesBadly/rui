#include <SFML/Graphics.hpp>
#include <UILib/util.hpp>

ui::Color fromSFColor(sf::Color color) {
  return ui::Color(color.r, color.g, color.b, color.a);
};
sf::Color toSFColor(ui::Color color) {
  return sf::Color(color.r, color.g, color.b, color.a);
};
