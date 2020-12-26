// изменение размера окна
macro_rules! window_resized{
    ($size:expr,$window:expr)=>{
        unsafe{
            window_width=$size.width as f32;
            window_height=$size.height as f32;
            window_center=[window_width/2f32,window_height/2f32];

            Resized([$size.width,$size.height])
        }
    };

    ($size:expr,$page:expr,$window:expr)=>{
        unsafe{
            window_width=$size.width as f32;
            window_height=$size.height as f32;
            window_center=[window_width/2f32,window_height/2f32];

            $page.on_window_resized($window,[$size.width,$size.height])
        }
    }
}


// Сдвиг мыши (сдвиг за пределы окна игнорируется)
macro_rules! cursor_moved{
    ($position:expr)=>{
        unsafe{
            let last_position=mouse_cursor.position();

            let position=[$position.x as f32,$position.y as f32];

            let dx=position[0]-last_position[0];
            let dy=position[1]-last_position[1];

            mouse_cursor.set_position(position);

            WindowEvent::MouseMovementDelta([dx,dy])
        }
    };

    ($position:expr,$page:expr,$window:expr)=>{
        unsafe{
            let last_position=mouse_cursor.position();

            let position=[$position.x as f32,$position.y as f32];

            let dx=position[0]-last_position[0];
            let dy=position[1]-last_position[1];

            mouse_cursor.set_position(position);

            $page.on_mouse_moved($window,[dx,dy])
        }
    }
}

// Обработка действий с кнопками мыши
macro_rules! mouse_input{
    ($button:expr,$state:expr,$window:expr)=>{
        if $state==ElementState::Pressed{
            MousePressed($button)
        }
        else{
            MouseReleased($button)
        }
    };

    ($button:expr,$state:expr,$page:expr,$window:expr)=>{
        if $state==ElementState::Pressed{
            $page.on_mouse_pressed($window,$button)
        }
        else{
            $page.on_mouse_released($window,$button)
        }
    }
}

// Обработка действий с клавишами клавиатуры
macro_rules! keyboard_input{
    ($input:expr)=>{
        {
            let key=if let Some(key)=$input.virtual_keycode{
                unsafe{std::mem::transmute(key)}
            }
            else{
                KeyboardButton::Unknown
            };

            if $input.state==ElementState::Pressed{
                KeyboardPressed(key)
            }
            else{
                KeyboardReleased(key)
            }
        }
    };

    ($input:expr,$page:expr,$window:expr)=>{
        {
            let key=if let Some(key)=$input.virtual_keycode{
                unsafe{std::mem::transmute(key)}
            }
            else{
                KeyboardButton::Unknown
            };

            if $input.state==ElementState::Pressed{
                $page.on_keyboard_pressed($window,key)
            }
            else{
                $page.on_keyboard_released($window,key)
            }
        }
    }
}

// Цельный обработчик событий основанный на страницах
macro_rules! paged_event_listener{
    ($window:expr,$event:expr,$control_flow:expr,$page:expr,$state:expr)=>{
        // Проверка, нужно ли событие "обновление"
        #[cfg(not(feature="lazy"))]{
            $window.base.update_check();
            // Endless cycling checking events
            *$control_flow=ControlFlow::Poll;
        }
        
        #[cfg(feature="lazy")]{
            // Waiting for any event except redraw event
            *$control_flow=ControlFlow::Wait;
        }

        match $event{
            Event::UserEvent(event)=>match event{
                // Запрос на закрытие
                InnerWindowEvent::EventLoopCloseRequested=>{
                    *$control_flow=ControlFlow::Exit;
                    // Запрос на закрытие - для получения возвращаемого значения
                    // в LoopDestroyed
                    $state=EventLoopState::CloseRequested;
                    return
                }

                #[cfg(not(feature="lazy"))]
                InnerWindowEvent::Update=>$page.on_update_requested($window),

                #[cfg(feature="lazy")]
                _=>return
            }

            // События окна
            Event::WindowEvent{event,..}=>{
                match event{
                    // Закрытие окна
                    GWindowEvent::CloseRequested=>{
                        *$control_flow=ControlFlow::Exit;
                        $state=EventLoopState::CloseRequested;
                        $page.on_window_close_requested($window);
                    }

                    // Изменение размера окна
                    GWindowEvent::Resized(size)=>window_resized!(size,$page,$window),

                    // Сдвиг окна
                    GWindowEvent::Moved(pos)=>$page.on_window_moved($window,[pos.x,pos.y]),

                    // Сдвиг мыши (сдвиг за пределы окна игнорируется)
                    GWindowEvent::CursorMoved{position,..}=>cursor_moved!(position,$page,$window),

                    // Прокрутка колёсика мыши
                    GWindowEvent::MouseWheel{delta,..}=>$page.on_mouse_scrolled($window,delta),

                    // Обработка действий с кнопками мыши
                    GWindowEvent::MouseInput{button,state,..}=>mouse_input!(button,state,$page,$window),

                    // Обработка действий с клавишами клавиатуры
                    GWindowEvent::KeyboardInput{input,..}=>keyboard_input!(input,$page,$window),

                    // Получение вводимых букв
                    GWindowEvent::ReceivedCharacter(character)
                            if !character.is_ascii_control()=>$page.on_character_recieved($window,character),

                    // При потере фокуса
                    #[cfg(feature="auto_hide")]
                    GWindowEvent::Focused(f)=>if !f{
                        *$control_flow=ControlFlow::Exit;
                        $window.on_window_hidden();
                        $window.display().gl_window().window().set_minimized(true); // Сворацивание окна
                        $page.on_window_focused($window,f);
                    }

                    #[cfg(not(feature="auto_hide"))]
                    GWindowEvent::Focused(f)=>$page.on_window_focused($window,f),

                    GWindowEvent::ModifiersChanged(modifier)=>$page.on_modifiers_changed($window,modifier),

                    #[cfg(feature="file_drop")]
                    GWindowEvent::DroppedFile(path)=>$page.on_file_dropped($window,path),
                    #[cfg(feature="file_drop")]
                    GWindowEvent::HoveredFile(path)=>$page.on_file_hovered($window,path),
                    #[cfg(feature="file_drop")]
                    GWindowEvent::HoveredFileCancelled=>$page.on_file_hovered_canceled($window),

                    _=>{} // Игнорирование остальных событий
                }
            }

            Event::Suspended=>$page.on_suspended($window),
            Event::Resumed=>$page.on_resumed($window),

            // Запрос на рендеринг
            Event::MainEventsCleared=>{
                $window.display().gl_window().window().request_redraw();
            }

            // Рендеринг
            Event::RedrawRequested(_)=>{
                #[cfg(feature="fps_counter")]
                $window.base.count_fps();

                $page.on_redraw_requested($window);
            }

            Event::LoopDestroyed=>{
                if EventLoopState::CloseRequested==$state{
                    $state=EventLoopState::Closed($page.on_event_loop_closed($window))
                }
            }

            _=>{}
        }
    }
}

// Цельный обработчик событий основанный на страницах
// (для ожидания получения фокуса)
macro_rules! paged_wait_until_focused{
    ($window:expr,$event:expr,$control_flow:expr,$page:expr,$state:expr)=>{
        *$control_flow=ControlFlow::Wait;

        match $event{
            Event::UserEvent(event)=>match event{
                InnerWindowEvent::EventLoopCloseRequested=>{
                    *$control_flow=ControlFlow::Exit;
                    $state=EventLoopState::CloseRequested;
                    return
                }
                _=>return
            }

            Event::WindowEvent{event,..}=>{
                match event{
                    // Остановка цикла обработки событий
                    GWindowEvent::CloseRequested=>{
                        *$control_flow=ControlFlow::Exit;
                        $state=EventLoopState::CloseRequested;
                        $page.on_window_close_requested($window)
                    }

                    // Изменение размера окна
                    GWindowEvent::Resized(size)=>window_resized!(size,$page,$window),

                    // При получении фокуса
                    GWindowEvent::Focused(f)=>{
                        *$control_flow=ControlFlow::Exit;
                        $window.base.display.gl_window().window().set_minimized(false);
                        $window.on_window_unhidden();
                        $page.on_window_focused($window,f);
                    }

                    _=>return
                }
            }

            Event::Suspended=>$page.on_suspended($window),
            Event::Resumed=>$page.on_resumed($window),

            Event::LoopDestroyed=>{
                if EventLoopState::CloseRequested==$state{
                    $state=EventLoopState::Closed($page.on_event_loop_closed($window))
                }
            }

            _=>return
        }
    }
}