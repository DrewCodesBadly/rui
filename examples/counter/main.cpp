#include "UILib/element.hpp"
#include "UILib/text.hpp"
#include "UILib/util.hpp"
#include "UILib/widgets/buttons.hpp"
#include <UILib/lib.hpp>
#include <UILib/shapes.hpp>
#include <functional>
#include <string>

class CounterRoot : public ui::Element {
private:
  int count = 0;
  // Defining the children of this element
  ui::Text counterText = ui::Text("Counter: 0");
  ui::Text titleText = ui::Text("Counter App");
  ui::Text buttonText = ui::Text("Increment Counter");
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
  ui::loadFont("roboto", "assets/Roboto-Regular.ttf");
  ui::setDefaultFont("roboto");
  ui::App app = ui::App("Hello World Example")
                    .withVsync(true)
                    .withBackgroundColor(ui::Color(20, 20, 20));
  CounterRoot root;
  app.setRootElement(&root);
  app.openWindow();
  return 0;
}
