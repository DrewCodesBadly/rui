#include "element.cpp"
#include "listeners.cpp"
#include "shapes.cpp"
#include "text.cpp"
#include "util.cpp"
#include "widgets/buttons.cpp"
#include <SFML/Graphics.hpp>
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/Rect.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Graphics/View.hpp>
#include <SFML/System/String.hpp>
#include <SFML/System/Vector2.hpp>
#include <SFML/Window/ContextSettings.hpp>
#include <SFML/Window/Event.hpp>
#include <SFML/Window/WindowEnums.hpp>
#include <UILib/element.hpp>
#include <UILib/lib.hpp>

/// Starts running this ui::App in a window.
void ui::App::openWindow(bool fullscreen) {
  sf::ContextSettings settings;
  settings.antiAliasingLevel = antiAliasingLevel;
  sf::RenderWindow window(
      sf::VideoMode({sizeX, sizeY}), appName, sf::Style::Default,
      fullscreen ? sf::State::Fullscreen : sf::State::Windowed, settings);
  window.setVerticalSyncEnabled(true);
  bool needsRedraw = true;
  sf::Vector2i lastMousePos = sf::Vector2i(-1, -1);
  while (window.isOpen()) {

    while (const std::optional event = window.pollEvent()) {
      // Example: listening to different types of events and "handling" them
      if (event->is<sf::Event::Closed>()) {
        window.close();
      } else if (const auto *resized = event->getIf<sf::Event::Resized>()) {
        sizeX = resized->size.x;
        sizeY = resized->size.y;
        // resets the window view to handle size change correctly.
        window.setView(
            sf::View(sf::FloatRect({0., 0.}, sf::Vector2f(resized->size))));
        needsRedraw = true;
      } else if (const auto *keyPressed =
                     event->getIf<sf::Event::KeyPressed>()) {
        // todo
      } else if (const auto *keyRelease =
                     event->getIf<sf::Event::KeyReleased>()) {
        // todo
      } else if (const auto *mousePressed =
                     event->getIf<sf::Event::MouseButtonPressed>()) {
        for (MouseListener *listener : activeListeners) {
          if (listener->contains(mousePressed->position.x,
                                 mousePressed->position.y)) {
            switch (mousePressed->button) {
            case sf::Mouse::Button::Left:
              if (listener->onLeftClickPressed()) {
                needsRedraw = true;
              }
              break;
            case sf::Mouse::Button::Right:
              if (listener->onRightClickPressed()) {
                needsRedraw = true;
              }
              break;
            case sf::Mouse::Button::Middle:
            case sf::Mouse::Button::Extra1:
            case sf::Mouse::Button::Extra2:
              break;
            }
          }
        }
      } else if (const auto *mouseReleased =
                     event->getIf<sf::Event::MouseButtonReleased>()) {
        for (MouseListener *listener : activeListeners) {
          if (listener->contains(mouseReleased->position.x,
                                 mouseReleased->position.y)) {
            switch (mouseReleased->button) {
            case sf::Mouse::Button::Left:
              if (listener->onLeftClickReleased()) {
                needsRedraw = true;
              }
              break;
            case sf::Mouse::Button::Right:
              if (listener->onRightClickReleased()) {
                needsRedraw = true;
              }
              break;
            case sf::Mouse::Button::Middle:
            case sf::Mouse::Button::Extra1:
            case sf::Mouse::Button::Extra2:
              break;
            }
          }
        }
      } else if (const auto *mouseMoved =
                     event->getIf<sf::Event::MouseMoved>()) {
        for (MouseListener *listener : activeListeners) {
          if (listener->contains(mouseMoved->position.x,
                                 mouseMoved->position.y)) {
            if (!listener->contains(lastMousePos.x, lastMousePos.y)) {
              if (listener->onMouseEntered()) {
                needsRedraw = true;
              }
            }
          } else if (listener->contains(lastMousePos.x, lastMousePos.y)) {
            if (listener->onMouseExited()) {
              needsRedraw = true;
            }
          }
        }

        lastMousePos = mouseMoved->position;
      } else if (const auto *mouseScrolled =
                     event->getIf<sf::Event::MouseWheelScrolled>()) {
        // todo
      } else if (const auto *textEntered =
                     event->getIf<sf::Event::TextEntered>()) {
        // todo
      }
    }

    if (needsRedraw) {
      window.clear(toSFColor(backgroundColor));
      activeListeners.clear();

      ui::ElementRenderContext context;
      context.size = sf::Vector2f(sizeX, sizeY);
      rootElement->drawToWindow(&window, context);
      rootElement->renderChildren(&window, context);

      window.display();
      needsRedraw = false;
    }
  }
}

void ui::App::updateMouseListeners(int oldX, int oldY, int newX, int newY) {}

// App builder methods

/// Sets whether or not vsync is enabled on the window.
ui::App ui::App::withVsync(bool vsync) {
  this->useVsync = vsync;
  return *this;
}

ui::App ui::App::withAntiAliasing(int antiAliasingLevel) {
  this->antiAliasingLevel = antiAliasingLevel;
  return *this;
}

ui::App ui::App::withBackgroundColor(ui::Color color) {
  this->backgroundColor = color;
  return *this;
}

void ui::App::setRootElement(ui::Element *element) { rootElement = element; }
