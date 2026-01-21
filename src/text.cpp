#include "UILib/element.hpp"
#include "element.cpp"
#include "util.cpp"
#include <SFML/Graphics/Font.hpp>
#include <SFML/Graphics/Text.hpp>
#include <SFML/System/Vector2.hpp>
#include <UILib/text.hpp>
#include <exception>
#include <stdexcept>
#include <string>
#include <unordered_map>

std::unordered_map<std::string, sf::Font> loadedFonts;
sf::Font *defaultFont = nullptr;

class MissingFontException : public std::exception {
private:
  std::string missingFont;

public:
  MissingFontException(std::string missingFont) : missingFont(missingFont) {}

  const char *what() const noexcept override {
    std::string msg = "Tried to find font \"" + missingFont +
                      "\", but no such font was loaded. Try loading it first "
                      "using `loadFont`.";
    const char *buf = msg.c_str();
    return buf;
  }
};

class FontNotFoundException : public std::exception {
private:
  std::string filePath;

public:
  FontNotFoundException(std::string filePath) : filePath(filePath) {}

  const char *what() const noexcept override {
    std::string msg = ("Failed to load font from file \"" + filePath +
                       "\", does the file exist?");
    const char *buf = msg.c_str();
    return buf;
  }
};

class DefaultFontMissingException : public std::exception {
public:
  const char *what() const noexcept override {
    return "No default font loaded - cannot display text without a default "
           "font! Please set the font using `setDefaultFont`";
  }
};

sf::Text createTextObject(ui::Text *text) {
  // Tries to get the set font, if it fails or none is set, use default font.
  // If neither are set, throws an exception.
  sf::Font *f = nullptr;
  if (text->font.has_value()) {
    try {
      f = &loadedFonts[text->font.value()];
    } catch (std::out_of_range e) {
      throw MissingFontException(text->font.value());
    }
  } else {
    f = defaultFont;
  }
  if (f == nullptr) {
    throw DefaultFontMissingException();
  }
  sf::Text t = sf::Text(*f, text->text);
  t.setCharacterSize(text->fontSize);
  return t;
}

void ui::Text::drawToWindow(ElementRenderContext context) {
  sf::Text t = createTextObject(this);
  t.setPosition(context.topLeft - t.getLocalBounds().position);
  t.setFillColor(toSFColor(textColor));
  context.window->draw(t);
}

void ui::Text::recalculateMinimumSize() {
  // Uses this to get the minimum size of children as a normal element would
  // The maximum between that size and the actual text minimum size is used.
  Element::recalculateMinimumSize();
  ElementSizeCache childrenMinSize = sizeCache.value();
  sf::Text t = createTextObject(this);
  sf::FloatRect bounds = t.getLocalBounds();

  childrenMinSize.x = std::max(childrenMinSize.x, bounds.size.x);
  childrenMinSize.y = std::max(childrenMinSize.y, bounds.size.y);
  sizeCache.emplace(childrenMinSize);
}

void ui::loadFont(std::string name, std::string filePath) {
  try {
    loadedFonts.insert_or_assign(name, sf::Font(filePath));
  } catch (std::exception e) {
    throw FontNotFoundException(filePath);
  }
}

void ui::setDefaultFont(std::string fontName) {
  try {
    defaultFont = &loadedFonts.at(fontName);
  } catch (std::out_of_range e) {
    throw MissingFontException(fontName);
  }
}
