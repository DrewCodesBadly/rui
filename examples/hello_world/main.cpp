#include "UILib/element.hpp"
#include "UILib/text.hpp"
#include "UILib/util.hpp"
#include <SFML/Graphics/Font.hpp>
#include <SFML/System/Vector2.hpp>
#include <UILib/lib.hpp>
#include <UILib/shapes.hpp>

int main() {
  ui::App app = ui::App("Hello World Example")
                    .withVsync(true)
                    .withBackgroundColor(ui::Color(0, 0, 0));
  ui::Text t = ui::Text(sf::Font("assets/Roboto-Regular.ttf"), "Hello World!");
  ui::Box b1 = ui::Box({&t});
  ui::Box b2 = ui::Box();
  ui::Box b3 = ui::Box();
  ui::Box root = ui::Box({&b1, &b2, &b3});

  b1.background = ui::Color(100, 100, 100);
  b1.inset = ui::RectBorders(20.0);
  b2.background = ui::Color(255, 0, 0);
  b2.spacePriority = 1;
  b2.perpendicularSizing = ui::PerpendicularSizing::Stretch;
  b3.background = ui::Color(0, 255, 0);
  b3.spacePriority = 2;
  b3.perpendicularSizing = ui::PerpendicularSizing::Stretch;

  root.background = ui::Color(255, 255, 255);
  root.verticalChildren = false;
  root.spacePriority = 1;
  root.perpendicularSizing = ui::PerpendicularSizing::Expand;

  app.setRootElement(&root);
  app.openWindow();
  return 0;
}
