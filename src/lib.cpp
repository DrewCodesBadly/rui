#include "UILib/element.hpp"
#include "util.cpp"
#include <SFML/Graphics.hpp>
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/System/String.hpp>
#include <SFML/Window/ContextSettings.hpp>
#include <SFML/Window/Event.hpp>
#include <SFML/Window/WindowEnums.hpp>
#include <UILib/lib.hpp>

/// Starts running this app in a window.
void App::openWindow(bool fullscreen, unsigned int sizeX, unsigned int sizeY) {
  sf::ContextSettings settings;
  settings.antiAliasingLevel = antiAliasingLevel;
  sf::RenderWindow window(
      sf::VideoMode({sizeX, sizeY}), appName, sf::Style::Default,
      fullscreen ? sf::State::Fullscreen : sf::State::Windowed, settings);
  window.setVerticalSyncEnabled(true);
  bool needsRedraw = true;
  while (window.isOpen()) {

    while (const std::optional event = window.pollEvent()) {
      // Example: listening to different types of events and "handling" them
      if (event->is<sf::Event::Closed>()) {
        window.close();
      } else if (const auto *resized = event->getIf<sf::Event::Resized>()) {
      } else if (const auto *keyPressed =
                     event->getIf<sf::Event::KeyPressed>()) {
      } else if (const auto *keyRelease =
                     event->getIf<sf::Event::KeyReleased>()) {
      } else if (const auto *mousePressed =
                     event->getIf<sf::Event::MouseButtonPressed>()) {
      } else if (const auto *mouseReleased =
                     event->getIf<sf::Event::MouseButtonReleased>()) {
      } else if (const auto *mouseMoved =
                     event->getIf<sf::Event::MouseMoved>()) {
      } else if (const auto *mouseScrolled =
                     event->getIf<sf::Event::MouseWheelScrolled>()) {
      } else if (const auto *textEntered =
                     event->getIf<sf::Event::TextEntered>()) {
      } else if (const auto *resized = event->getIf<sf::Event::Resized>()) {
      }
    }

    if (needsRedraw) {
      window.clear(toSFColor(backgroundColor));

      // draw...

      window.display();
      needsRedraw = false;
    }
  }
}

// App builder methods

/// Sets whether or not vsync is enabled on the window.
App App::withVsync(bool vsync) {
  this->useVsync = vsync;
  return *this;
}

App App::withAntiAliasing(int antiAliasingLevel) {
  this->antiAliasingLevel = antiAliasingLevel;
  return *this;
}

App App::withBackgroundColor(ui::Color color) {
  this->backgroundColor = color;
  return *this;
}

void App::setRootElement(ui::Element element) { rootElement = element; }
