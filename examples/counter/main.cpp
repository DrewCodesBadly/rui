#include "UILib/element.hpp"
#include "UILib/text.hpp"
#include "UILib/util.hpp"
#include "UILib/widgets/buttons.hpp"
#include <SFML/Graphics/Font.hpp>
#include <SFML/System/Vector2.hpp>
#include <UILib/lib.hpp>
#include <UILib/shapes.hpp>
#include <functional>
#include <string>

class CounterRoot : public ui::Element {
private:
  int count = 0;
  // Defining the children of this element
  ui::Text counterText =
      ui::Text(sf::Font("assets/Roboto-Regular.ttf"), "Counter: 0");
  ui::Text titleText =
      ui::Text(sf::Font("assets/Roboto-Regular.ttf"), "Counter App");
  ui::Text buttonText =
      ui::Text(sf::Font("assets/Roboto-Regular.ttf"), "Increment Counter");
  ui::Button button = ui::Button(
      ui::Color(210, 50, 50),
      // Lambda callback when the button is pressed
      // Returning `true` indicates that the window needs to be redrawn.
      [this] {
        count++;
        counterText.text = "Counter: " + std::to_string(count);
        return true;
      },
      {&buttonText});

public:
  CounterRoot() {
    children.push_back(&titleText);
    children.push_back(&counterText);
    children.push_back(&button);
  };
};

int main() {
  ui::App app = ui::App("Hello World Example")
                    .withVsync(true)
                    .withBackgroundColor(ui::Color(20, 20, 20));

  CounterRoot root;
  app.setRootElement(&root);
  app.openWindow();
  return 0;
}
