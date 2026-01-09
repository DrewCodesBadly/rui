#include <SFML/Graphics/RectangleShape.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/System/Vector2.hpp>
#include <UILib/element.hpp>
#include <algorithm>

/// Draws this element to the screen.
void ui::Element::drawToWindow(sf::RenderWindow *window,
                               ElementRenderContext context) {
  // Draws a rectangle in the window
  sf::RectangleShape shape;
  shape.setSize(sf::Vector2f(context.size.x, context.size.y));
  shape.setPosition(sf::Vector2f(context.topLeft.x, context.topLeft.y));
  window->draw(shape);
}

/// Draws all children of this element.
void ui::Element::renderChildren(sf::RenderWindow *window,
                                 ElementRenderContext context) {
  unsigned int usedSpace = 0;
  unsigned int perpendicularMinSize = 0;
  unsigned int totalAxisSplit = 0;

  // Move in to allow for padding
  context.topLeft += sf::Vector2u(inset.left, inset.top);
  // TODO: fix underflows?
  context.size -= sf::Vector2u(inset.right, inset.bottom);

  // TODO: replace with getMinimumSize since that's all this does.
  // Divide space among children
  for (Element *child : children) {
    sf::Vector2u childMinSize = child->getMiniminumSize();
    if (verticalChildren) {
      perpendicularMinSize = std::max(perpendicularMinSize, childMinSize.x);
      usedSpace += childMinSize.y;
    } else {
      perpendicularMinSize = std::max(perpendicularMinSize, childMinSize.y);
      usedSpace += childMinSize.x;
    }
  }

  unsigned int portionedLeftoverSpace;
  if (totalAxisSplit > 0) {
    // again - underflows?
    portionedLeftoverSpace =
        verticalChildren ? context.size.y : context.size.x - usedSpace;
    portionedLeftoverSpace /= totalAxisSplit;
  } else {
    portionedLeftoverSpace = 0;
  }

  // Render each child
  for (Element *child : children) {
    ElementRenderContext childContext(context);
    sf::Vector2u childMinSize = child->getMiniminumSize();

    // Allocate correct amount of space to child
    if (verticalChildren) {
      childContext.size.y = childMinSize.y;
      childContext.size.y +=
          portionedLeftoverSpace * child->vertialSpacePriority;
      childContext.size.x = perpendicularMinSize;

      if (child->horizontalSpacePriority == 0) {
        childContext.topLeft.x +=
            (childContext.size.x - childMinSize.x) * child->halign;
        childContext.size.x = childMinSize.x;
      }

      context.topLeft.y += childContext.size.y;
    } else {
      childContext.size.x = childMinSize.x;
      childContext.size.x +=
          portionedLeftoverSpace * child->horizontalSpacePriority;
      childContext.size.y = perpendicularMinSize;

      if (child->vertialSpacePriority == 0) {
        childContext.topLeft.y +=
            (childContext.size.y - childMinSize.y) * child->halign;
        childContext.size.y = childMinSize.y;
      }

      context.topLeft.x += childContext.size.x;
    }

    // Draw child
    child->drawToWindow(window, childContext);
    child->renderChildren(window, childContext);
  }

  // Clears cached minimum size after rendering.
  clearCachedMinSize();
}

/// Gets the minimum size needed for this element to render correctly.
sf::Vector2u ui::Element::getMiniminumSize() {
  if (cachedMinSize.has_value()) {
    return cachedMinSize.value();
  } else {
    cachedMinSize.emplace(recalculateMinimumSize());
    return cachedMinSize.value();
  }
}

/// Recalculates the minimum size needed to render this element.
sf::Vector2u ui::Element::recalculateMinimumSize() {
  // TODO: Add logic
  return minSize;
}

void ui::Element::clearCachedMinSize() { cachedMinSize.reset(); }
