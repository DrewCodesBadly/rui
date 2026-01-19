#pragma once
#include "UILib/element.hpp"
#include "UILib/shapes.hpp"
#include "UILib/util.hpp"
#include <UILib/listeners.hpp>
#include <functional>
#include <initializer_list>

namespace ui {
class Button : public ui::MouseListener {

private:
  ui::Color color;
  std::function<bool()> onPressed;
  ui::Box backgroundBox;

public:
  virtual bool onLeftClickReleased() override;

  Button(ui::Color color, std::function<bool()> onPressed,
         std::initializer_list<Element *> children);
  ~Button() {};
};
} // namespace ui
