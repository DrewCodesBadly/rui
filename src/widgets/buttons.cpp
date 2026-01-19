#include "UILib/element.hpp"
#include "UILib/shapes.hpp"
#include "UILib/util.hpp"
#include <UILib/widgets/buttons.hpp>
#include <initializer_list>

bool ui::Button::onLeftClickReleased() { return onPressed(); }

ui::Button::Button(ui::Color color, std::function<bool()> onPressed,
                   std::initializer_list<Element *> children)
    : color(color), onPressed(onPressed) {
  backgroundBox.background = color;
  this->children.push_back(&backgroundBox);
  for (Element *child : children) {
    backgroundBox.children.push_back(child);
  }
}
