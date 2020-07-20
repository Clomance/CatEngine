# Общее

Всего три вида окон. Отличаются они по скорости работы и возможностям (ниже будут рассмотрены по отдельности).

Для создания окон используются две функции:
 * с замыканием - выдаёт настройки по умолчанию, а также даёт список мониторов (для установки полноэкранного режима)
```
let mut window=DefaultWindow::new(|monitors,settings|{
    let monitor=monitors.remove(0);
    let fullscreen=cat_engine::glium::glutin::window::Fullscreen::Borderless(monitor);
    window_settings.window_attributes.fullscreen=Some(fullscreen);
}).unwrap();
```

* с ручной настройкой
```
let graphics_settings=GraphicsSettings::new();
let general_settings=GeneralSettings::new();
let context_builder=ContextBuilder::new();
let window_builder=WindowBuilder::default();
let event_loop=EventLoop::<InnerWindowEvent>::with_user_event();

let mut window=PagedWindow::raw(
    window_builder,
    context_builder,
    graphics_settings,
    event_loop,
    general_settings,
).unwrap();
```

Окна поддерживают следующие собития:
 - запрос на закрытие окна
 - изменение рамера и перемещение окна
 - обновление (только при `feature != "lazy"`)
 - перерисовка окна
 - приостановка и возобновление приложения
 - остановка цикла событий (когда нужно закрыть "страницу" для `PagedWindow` и `DynamicWindow`)
 - окно получило/потеряло фокус
 - события мышки и клавиатуры
 - изменение модификаторов (Shift,Ctrl,Alt,Logo)
 - перенос файлов в окно (только при `feature = "file_drop"`)



# WindowBase

Основа для окон, включает в себя само окно, графические функции,
цикл событий и генератор пользовательских событий.

С помощью её можно создать своё окно.
Все поля основы доступны.

Также в её включены многие "фичи".

# DefaultWindow

Все события обрабатываются и добавляются в очередь внешней обработки (Window.events)
для работы с ними вне структуры окна.

Имеет самый широкий спектр возможностей, но является самым медленным и имеет много проблем.

```
let mut window=DefaultWindow::new(|_,_|{}).unwrap();

while let Some(event)=window.next_event(){
    match event{
        WindowEvent::CloseRequested=>{
            break
        }

        WindowEvent::Update=>{
            
        }

        WindowEvent::RedrawRequested=>{
            // Рендеринг
        }
        _=>{}
    }
}
```

# PagedWindow

### Работа с помощью "страниц"

Все события прописываются с помощь типажа `WindowPage`
и обработываются сразу же после их появления.

Является самым производительным методом, но имеет серьёзные ограничения в возможностях.

### Работа с помощью замыканий

Все события обратываются в замыкании.

Этот метод совмещает удобство `Обычной структуры окна` и скорость типажа `WindowPage`, но так же, как и последний ограничивает некоторые возможности.


# DynamicWindow

Все события прописываются с помощь типажа `WindowPage`
и обработываются сразу же после их появления.

Это окно использует страницы как типажи-объекты, поэтому их сможно менять на ходу.