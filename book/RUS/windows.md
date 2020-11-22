# Введение

### Определения

"Страница" - типы, которые реализуют типаж `WindowPage`.


Всего два вида окон. Отличаются они по скорости работы и возможностям (ниже будут рассмотрены по отдельности):
- `PagedWindow` - обрабатывает события быстрее `DynamicWindow`, но долго переключает "страницы", имеет два способа работы, которые можно чередовать
- `DynamicWindow` - немного медленней, чем `PagedWindow`, но гораздо быстрее переключает страницы

### Создание

Для создания окон используются две функции:
 - с замыканием - выдаёт настройки по умолчанию, а также даёт список мониторов (для установки полноэкранного режима)
```
let mut window=PagedWindow::new(|monitors,settings|{
    let monitor=monitors.remove(0);
    let fullscreen=cat_engine::glium::glutin::window::Fullscreen::Borderless(monitor);
    window_settings.window_attributes.fullscreen=Some(fullscreen);
}).unwrap();
```

 - с ручной настройкой
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

### События

Окна поддерживают следующие события:
 - запрос на закрытие окна
 - изменение размера и перемещение окна
 - обновление (только при `feature != "lazy"`)
 - перерисовка окна
 - приостановка и возобновление приложения
 - остановка цикла событий (когда нужно закрыть "страницу")
 - окно получило/потеряло фокус
 - события мышки и клавиатуры
 - изменение модификаторов (Shift, Ctrl, Alt, Logo)
 - перенос файлов в окно (только при `feature = "file_drop"`)



# WindowBase

Основа для окон, включает в себя само окно, графические функции,
цикл событий и генератор пользовательских событий.

С помощью её можно создать своё окно.
Все поля основы доступны.

Также в её включены многие "фичи".



# PagedWindow

### Работа с помощью "страниц"

Все события прописываются с помощь типажа `WindowPage`
и обработываются сразу же после их появления.

Является самым производительным методом, но имеет серьёзные ограничения в возможностях.

```
pub struct Page;

impl WindowPage<'static> for Page{
    type Window=PagedWindow;
    type Output=();

    fn on_window_close_requested(&mut self,_window:&mut PagedWindow){
        // Автоматически выходит из цикла
    }

    #[cfg(not(feature="lazy"))]
    fn on_update_requested(&mut self,_window:&mut PagedWindow){
        // Какие-то действия
    }

    fn on_redraw_requested(&mut self,_window:&mut PagedWindow){
        // Рендеринг
    }

    fn on_mouse_pressed(&mut self,_window:&mut PagedWindow,_button:MouseButton){}
    fn on_mouse_released(&mut self,_window:&mut PagedWindow,_button:MouseButton){}
    fn on_mouse_moved(&mut self,_window:&mut PagedWindow,_:[f32;2]){}
    fn on_mouse_scrolled(&mut self,_window:&mut PagedWindow,_:MouseScrollDelta){}

    fn on_keyboard_pressed(&mut self,_window:&mut PagedWindow,button:KeyboardButton){}

    fn on_keyboard_released(&mut self,_window:&mut PagedWindow,_button:KeyboardButton){}

    fn on_character_recieved(&mut self,_window:&mut PagedWindow,_character:char){}

    fn on_window_resized(&mut self,_window:&mut PagedWindow,_new_size:[u32;2]){}

    fn on_suspended(&mut self,_window:&mut PagedWindow){}
    fn on_resumed(&mut self,_window:&mut PagedWindow){}

    fn on_window_moved(&mut self,_window:&mut PagedWindow,_:[i32;2]){}

    fn on_window_focused(&mut self,_window:&mut PagedWindow,_:bool){}

    fn on_modifiers_changed(&mut self,_window:&mut PagedWindow,_modifiers:ModifiersState){}

    #[cfg(feature="file_drop")]
    fn on_file_dropped(&mut self,_:&mut PagedWindow,_:PathBuf){}
    #[cfg(feature="file_drop")]
    fn on_file_hovered(&mut self,_:&mut PagedWindow,_:PathBuf){}
    #[cfg(feature="file_drop")]
    fn on_file_hovered_canceled(&mut self,_:&mut PagedWindow){}

    fn on_event_loop_closed(&mut self,_:&mut PagedWindow){}
}


fn main(){
    let mut window=PagedWindow::new(|_,_|{}).unwrap();

    let mut page=Page;

    window.run_page(&mut page);
}
```

### Работа с помощью замыканий

Все события обратываются в замыкании.

Этот метод медленнее первого но имеет больше возможностей.

```
let mut window=PagedWindow::new(|_,_|{}).unwrap();

window.run(|window,event|{
    match event{
        WindowEvent::CloseRequested=>{
            // Автоматически выходит из цикла
        }

        WindowEvent::Update=>{
            // Какие-то действия
        }

        WindowEvent::RedrawRequested=>{
            // Рендеринг
        }
        _=>{}
    }
});
```



# DynamicWindow

Все события прописываются с помощь типажа `WindowPage`
и обработываются сразу же после их появления.

Это окно использует страницы как типажи-объекты, поэтому их можно менять на ходу.

```
pub struct Page;

impl<'a> WindowPage<'a> for Page{
    type Window=DynamicWindow<'a>;
    type Output=();

    fn on_window_close_requested(&mut self,_window:&mut DynamicWindow<'a>){
        // Автоматически выходит из цикла
    }

    #[cfg(not(feature="lazy"))]
    fn on_update_requested(&mut self,_window:&mut DynamicWindow<'a>){
        // Какие-то действия
    }

    fn on_redraw_requested(&mut self,_window:&mut DynamicWindow<'a>){
        // Рендеринг
    }

    fn on_mouse_pressed(&mut self,_window:&mut DynamicWindow<'a>,_button:MouseButton){}
    fn on_mouse_released(&mut self,_window:&mut DynamicWindow<'a>,_button:MouseButton){}
    fn on_mouse_moved(&mut self,_window:&mut DynamicWindow<'a>,_:[f32;2]){}
    fn on_mouse_scrolled(&mut self,_window:&mut DynamicWindow<'a>,_:MouseScrollDelta){}

    fn on_keyboard_pressed(&mut self,_window:&mut DynamicWindow<'a>,button:KeyboardButton){}

    fn on_keyboard_released(&mut self,_window:&mut DynamicWindow<'a>,_button:KeyboardButton){}

    fn on_character_recieved(&mut self,_window:&mut DynamicWindow<'a>,_character:char){}

    fn on_window_resized(&mut self,_window:&mut DynamicWindow<'a>,_new_size:[u32;2]){}

    fn on_suspended(&mut self,_window:&mut DynamicWindow<'a>){}
    fn on_resumed(&mut self,_window:&mut DynamicWindow<'a>){}

    fn on_window_moved(&mut self,_window:&mut DynamicWindow<'a>,_:[i32;2]){}

    fn on_window_focused(&mut self,_window:&mut DynamicWindow<'a>,_:bool){}

    fn on_modifiers_changed(&mut self,_window:&mut DynamicWindow<'a>,_modifiers:ModifiersState){}

    #[cfg(feature="file_drop")]
    fn on_file_dropped(&mut self,_:&mut DynamicWindow<'a>,_:PathBuf){}
    #[cfg(feature="file_drop")]
    fn on_file_hovered(&mut self,_:&mut DynamicWindow<'a>,_:PathBuf){}
    #[cfg(feature="file_drop")]
    fn on_file_hovered_canceled(&mut self,_:&mut DynamicWindow<'a>){}

    fn on_event_loop_closed(&mut self,_:&mut DynamicWindow<'a>){}
}

fn main(){
    let mut window=DynamicWindow::new(|_,_|{}).unwrap();

    let mut page=Page;

    window.run(&mut page);
}
```