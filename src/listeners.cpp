#include "UILib/element.hpp"
#include <SFML/Graphics/RenderWindow.hpp>
#include <UILib/listeners.hpp>
#include <vector>

static std::vector<ui::MouseListener *> activeListeners;

void ui::MouseListener::drawToWindow(sf::RenderWindow *window,
                                     ElementRenderContext context) {
  // No-op, but adds self to the listener list
  currentX = context.topLeft.x;
  currentY = context.topLeft.y;
  currentRight = context.size.x + currentX;
  currentBottom = context.size.y + currentY;
  activeListeners.push_back(this);
}

bool ui::MouseListener::contains(int x, int y) {
  return x >= currentX && x <= currentRight && y >= currentY &&
         y <= currentBottom;
}
