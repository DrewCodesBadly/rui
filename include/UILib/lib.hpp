#pragma once
#include "element.hpp"
#include "util.hpp"
#include <string>

namespace ui {
/// Base class for an application.
class App {
private:
  bool needsRedraw = true;
  bool useVsync = true;
  int antiAliasingLevel = 4;
  ui::Color backgroundColor = ui::Color(0, 0, 0, 255);
  std::string appName;
  ui::Element *rootElement;
  unsigned int sizeX = 512;
  unsigned int sizeY = 512;

  void updateMouseListeners(int oldX, int oldY, int newX, int newY);

public:
  App(std::string name) : appName(name) {};
  App withVsync(bool vsync);
  App withAntiAliasing(int antiAliasingLevel);
  App withBackgroundColor(ui::Color color);
  void setRootElement(ui::Element *element);

  void openWindow(bool fullscreen = false);
};
} // namespace ui
