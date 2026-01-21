#pragma once
#include "UILib/util.hpp"
#include "element.hpp"
#include <optional>
#include <string>
namespace ui {
/// Draws text.
class Text : public Element {
public:
  std::string text;
  std::optional<std::string> font;
  unsigned int fontSize = 24;
  ui::Color textColor = Color(0, 0, 0);
  void drawToWindow(ElementRenderContext context) override;
  void recalculateMinimumSize() override;

  Text(std::string text) : text(text) {};
  Text(std::string text, std::string font) : font(font), text(text) {};
  Text(std::initializer_list<Element *> children) {
    this->children = children;
  };
  ~Text() {};
};

/// Loads a font located at the given file path (relative or absolute.),
/// making it available to use for Text elements. Will throw an
/// exception if the font does not exist.
void loadFont(std::string name, std::string filePath);
/// Sets the default font for any Text elements without a specified font to use.
/// Make sure this font has been loaded first using loadFont.
void setDefaultFont(std::string fontName);
} // namespace ui
