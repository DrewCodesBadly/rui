#pragma once
#include "element.hpp"
#include <vector>
namespace ui {

class MouseListener : public ui::Element {
private:
  float currentX, currentY, currentRight, currentBottom;

public:
  bool contains(int x, int y);
  void drawToWindow(sf::RenderWindow *window,
                    ElementRenderContext context) override;

  // Virtual functions to override to handle mouse events.
  // The return value decides if the window needs to be redrawn.
  virtual bool onLeftClickPressed() { return false; };
  virtual bool onLeftClickReleased() { return false; };
  virtual bool onRightClickPressed() { return false; };
  virtual bool onRightClickReleased() { return false; };
  virtual bool onMouseEntered() { return false; };
  virtual bool onMouseExited() { return false; };

  MouseListener() {};
  MouseListener(std::initializer_list<Element *> children) {
    this->children = children;
  };
  ~MouseListener() {};
};
} // namespace ui
