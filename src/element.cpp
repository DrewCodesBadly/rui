#include <SFML/Graphics/RectangleShape.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/System/Vector2.hpp>
#include <UILib/element.hpp>
#include <UILib/util.hpp>

/// Draws this element to the screen.
void ui::Element::drawToWindow(sf::RenderWindow *window,
                               ElementRenderContext context) {
  // no op
}

/// Draws all children of this element.
void ui::Element::renderChildren(sf::RenderWindow *window,
                                 ElementRenderContext context) {
  float usedSpace = 0;
  float perpendicularMinSize = 0;
  unsigned int totalAxisSplit = 0;

  // Move in to allow for padding
  context.topLeft += sf::Vector2f(inset.left, inset.top);
  context.size -= sf::Vector2f(inset.left, inset.top);
  context.size -= sf::Vector2f(inset.right, inset.bottom);

  // Divide space among children
  for (Element *child : children) {
    sf::Vector2f childMinSize = child->getMiniminumSize();
    perpendicularMinSize = std::max(perpendicularMinSize, childMinSize.y);
    usedSpace += childMinSize.x;
    if (child->perpendicularSizing == PerpendicularSizing::Expand) {
      perpendicularMinSize = context.size.y;
    }
    totalAxisSplit += child->spacePriority;
  }

  // Move down to align vertically
  context.topLeft.y +=
      (context.size.y - perpendicularMinSize) * childPerpendicularAlign;

  float portionedLeftoverSpace;
  if (totalAxisSplit > 0) {
    portionedLeftoverSpace =
        verticalChildren ? context.size.y : context.size.x - usedSpace;
    portionedLeftoverSpace /= totalAxisSplit;
  } else {
    portionedLeftoverSpace = 0.;
    context.topLeft.x += (context.size.x - usedSpace) * childParallelAlign;
  }

  // Render each child
  for (Element *child : children) {
    ElementRenderContext childContext(context);
    sf::Vector2f childMinSize = child->getMiniminumSize();

    // Allocate correct amount of space to child
    childContext.size.x = childMinSize.x;
    childContext.size.x += portionedLeftoverSpace * child->spacePriority;
    childContext.size.y = perpendicularMinSize;

    switch (child->perpendicularSizing) {
    case Shrink:
      childContext.topLeft.y +=
          (childContext.size.y - childMinSize.y) * child->perpendicularAlign;
      childContext.size.y = childMinSize.y;
      break;
    case Stretch:
      childContext.size.y = perpendicularMinSize;
      break;
    case Expand:
      // no operation needed
      break;
    }

    // Draw child
    child->drawToWindow(window, childContext);
    child->renderChildren(window, childContext);

    // Track current X position
    context.topLeft.x += childContext.size.x;
  }

  // Clears cached minimum size after rendering.
  clearCachedMinSize();
}

/// Gets the minimum size needed for this element to render correctly.
sf::Vector2f ui::Element::getMiniminumSize() {
  if (cachedMinSize.has_value()) {
    return cachedMinSize.value();
  } else {
    cachedMinSize.emplace(recalculateMinimumSize());
    return cachedMinSize.value();
  }
}

/// Recalculates the minimum size needed to render this element.
sf::Vector2f ui::Element::recalculateMinimumSize() {
  // TODO: support vertical orientation
  sf::Vector2f childrenMinSize = sf::Vector2f(0, 0);
  for (Element *child : children) {
    sf::Vector2f childMinSize = child->getMiniminumSize();
    childrenMinSize.x += childMinSize.x;
    childrenMinSize.y = std::max(childrenMinSize.y, childMinSize.y);
  }
  childrenMinSize.x += inset.left + inset.right;
  childrenMinSize.y += inset.top + inset.bottom;
  return sf::Vector2f(std::max(childrenMinSize.x, minSize.x),
                      std::max(childrenMinSize.y, minSize.y));
}

void ui::Element::clearCachedMinSize() { cachedMinSize.reset(); }
