#include "UILib/element.hpp"
#include <UILib/lib.hpp>
int main() {
  App app = App("Hello World Example").withVsync(true);
  app.setRootElement(ui::Element());
  app.openWindow();
  return 0;
}
