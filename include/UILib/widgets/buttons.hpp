#pragma once
#include "UILib/element.hpp"
#include "UILib/util.hpp"
#include <UILib/listeners.hpp>
#include <algorithm>
#include <functional>
#include <initializer_list>

namespace ui {
class Button : public ui::MouseListener {

private:
  std::function<bool()> onPressed;
  ui::Color activeColor;

public:
  ui::Color color;

  // Some default color schemes.
  ui::Color hoverColor =
      ui::Color(std::min(color.r + 25, 255), std::min(color.g + 25, 255),
                std::min(color.b + 25, 255));
  ui::Color pressedColor =
      ui::Color(std::max(color.r - 25, 0), std::max(color.g - 25, 0),
                std::max(color.b - 25, 0));

  virtual bool onLeftClickReleased() override;
  virtual bool onMouseEntered() override;
  virtual bool onMouseExited() override;
  virtual bool onLeftClickPressed() override;
  virtual void drawToWindow(ElementRenderContext context) override;

  Button(ui::Color color, std::function<bool()> onPressed,
         std::initializer_list<Element *> children);
  ~Button() {};
};
} // namespace ui
