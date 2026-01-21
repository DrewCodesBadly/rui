#pragma once
#include "util.hpp"
#include <initializer_list>
#include <optional>
#include <vector>

namespace ui {
enum PerpendicularSizing { Shrink, Stretch, Expand };

class ElementRenderContext;
struct ElementSizeCache {
  float x, y;
};

/// Base class for UI elements.
class Element {
private:
protected:
  std::optional<ElementSizeCache> sizeCache;

public:
  float minWidth = 0., minHeight = 0.;
  ui::RectBorders inset;
  std::vector<Element *> children;
  bool verticalChildren = false;
  // Alignment of children along the parallel axis.
  float childParallelAlign = 0.5;
  float childPerpendicularAlign = 0.5;

  // Info for placement inside of its parent.

  // Determines the proportion of leftover space given to this element.
  unsigned int spacePriority = 0;
  PerpendicularSizing perpendicularSizing = PerpendicularSizing::Shrink;

  // Determines this widget's alignment within allotted space
  // on a scale of [0,1], like UV coordinates.
  float perpendicularAlign = 0.5;

  virtual void drawToWindow(ElementRenderContext context);
  void renderChildren(ElementRenderContext context);

  ElementSizeCache getMiniminumSize();
  virtual void recalculateMinimumSize();

  Element() {};
  Element(std::initializer_list<Element *> children) : children(children) {};

  virtual ~Element() {};
};
} // namespace ui
