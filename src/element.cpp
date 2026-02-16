#pragma once
#include <SFML/Graphics/RectangleShape.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/System/Vector2.hpp>
#include <SFML/Window/Keyboard.hpp>
#include <UILib/element.hpp>
#include <UILib/util.hpp>
#include <algorithm>
#include <iostream>

class ui::ElementRenderContext {
public:
  sf::Vector2f topLeft;
  sf::Vector2f size;
  sf::RenderWindow *window;
};

/// Draws this element to the screen.
void ui::Element::drawToWindow(ElementRenderContext context) {
  // no op
}

/// Draws all children of this element.
void ui::Element::renderChildren(ElementRenderContext context) {
  float usedSpace = 0;
  float perpendicularMinSize = 0;
  unsigned int totalAxisSplit = 0;

  // Flips vectors if children are vertical
  if (verticalChildren) {
    context.size = sf::Vector2f(context.size.y, context.size.x);
    context.topLeft = sf::Vector2f(context.topLeft.y, context.topLeft.x);
  }

  // Move in to allow for padding
  context.topLeft += sf::Vector2f(inset.left, inset.top);
  context.size -= sf::Vector2f(inset.left, inset.top);
  context.size -= sf::Vector2f(inset.right, inset.bottom);

  // Divide space among children
  for (Element *child : children) {
    ElementSizeCache childMinSizeFromCache = child->getMiniminumSize();
    sf::Vector2f childMinSize =
        sf::Vector2f(childMinSizeFromCache.x, childMinSizeFromCache.y);
    if (verticalChildren) {
      childMinSize = sf::Vector2f(childMinSize.y, childMinSize.x);
    }

    perpendicularMinSize = std::max(perpendicularMinSize, childMinSize.y);
    usedSpace += childMinSize.x;
    if (child->perpendicularSizing == PerpendicularSizing::Expand) {
      perpendicularMinSize = context.size.y;
    }
    totalAxisSplit += child->spacePriority;
  }

  // Align along parallel axis
  context.topLeft.y +=
      (context.size.y - perpendicularMinSize) * childPerpendicularAlign;

  float portionedLeftoverSpace;
  if (totalAxisSplit > 0) {
    portionedLeftoverSpace = context.size.x - usedSpace;
    portionedLeftoverSpace /= totalAxisSplit;
  } else {
    portionedLeftoverSpace = 0.;
    context.topLeft.x += (context.size.x - usedSpace) * childParallelAlign;
  }

  // Render each child
  for (Element *child : children) {
    ElementRenderContext childContext(context);
    ElementSizeCache childMinSizeFromCache = child->getMiniminumSize();
    sf::Vector2f childMinSize =
        sf::Vector2f(childMinSizeFromCache.x, childMinSizeFromCache.y);
    if (verticalChildren) {
      childMinSize = sf::Vector2f(childMinSize.y, childMinSize.x);
    }

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

    // Track current X position
    context.topLeft.x += childContext.size.x;

    // Undo vector flip if needed
    if (verticalChildren) {
      childContext.size =
          sf::Vector2f(childContext.size.y, childContext.size.x);
      childContext.topLeft =
          sf::Vector2f(childContext.topLeft.y, childContext.topLeft.x);
    }

    // Draw child
    child->drawToWindow(childContext);
    child->renderChildren(childContext);
  }

  // Clears cached minimum size after rendering.
  sizeCache.reset();
}

/// Gets the minimum size needed for this element to render correctly.
ui::ElementSizeCache ui::Element::getMiniminumSize() {
  if (!sizeCache.has_value()) {
    recalculateMinimumSize();
  }
  return sizeCache.value();
}

/// Recalculates the minimum size needed to render this element.
void ui::Element::recalculateMinimumSize() {
  sf::Vector2f childrenMinSize = sf::Vector2f(0, 0);
  for (Element *child : children) {
    ElementSizeCache childMinSize = child->getMiniminumSize();
    if (verticalChildren) {
      childrenMinSize.y += childMinSize.y;
      childrenMinSize.x = std::max(childrenMinSize.x, childMinSize.x);
    } else {
      childrenMinSize.x += childMinSize.x;
      childrenMinSize.y = std::max(childrenMinSize.y, childMinSize.y);
    }
  }
  childrenMinSize.x += inset.left + inset.right;
  childrenMinSize.y += inset.top + inset.bottom;
  ElementSizeCache cache;
  cache.x = std::max(childrenMinSize.x, minWidth);
  cache.y = std::max(childrenMinSize.y, minHeight);
  sizeCache.emplace(cache);
}
