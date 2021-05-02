use cat_engine_basement::windows::{
    Window,
    WindowAttributes,
    WindowClass,
    WindowSubclassArguments,
};

use cat_engine_basement::windows::winapi::{
    shared::{
        windef::{
            HWND,
        },
        ntdef::LARGE_INTEGER
    },
    um::{
        winuser::{
            MSG,
            GetMessageW,
            PeekMessageW,
            SendMessageW,
            PostMessageW,
            TranslateMessage,
            DispatchMessageW,
            DestroyWindow,
            SetTimer,

            PM_REMOVE,

            // window messages
            WM_QUIT,
            WM_PAINT,
            WM_USER,
            WM_APP,
        },
        errhandlingapi::GetLastError,
        profileapi::{
            QueryPerformanceCounter,
            QueryPerformanceFrequency,
        },
    },
};

use std::{
    ptr::null_mut,
    mem::{
        transmute,
        zeroed
    },
    thread::{
        spawn,
        JoinHandle,
    },
    sync::mpsc::{
        Receiver,
        Sender,
        channel,
    },
};

pub fn create_windows_thread(channel:CommandReceiver)->JoinHandle<()>{
    spawn(move||{
        unsafe{
            let mut message:MSG=zeroed();

            loop{
                if let Ok(command)=channel.command.try_recv(){
                    match command{
                        Command::CreateWindow{
                            window_class,
                            window_subclass_args,
                            window_attributes,
                        }=>{
                            let class=&*window_class;
                            let window=Window::new(class,window_attributes,&*window_subclass_args).unwrap();
                            channel.result.send(CommandResult::Window{
                                window,
                            });
                        }

                        Command::Break=>break,
                    }
                }

                match GetMessageW(&mut message,null_mut(),0,0){
                    -1=>{
                        println!("Error");
                        break
                    }

                    0=>break,

                    _=>match message.message{
                        _=>{
                            TranslateMessage(&message);
                            DispatchMessageW(&message);
                        }
                    }
                }
            }
        }
    })
}

pub enum Command{
    CreateWindow{
        window_class:*const WindowClass,
        window_subclass_args:*const WindowSubclassArguments,
        window_attributes:WindowAttributes,
    },

    Break,
}

unsafe impl Send for Command{}

pub enum CommandResult{
    None,
    Window{
        window:Window,
    }
}

unsafe impl Send for CommandResult{}

pub struct CommandSender{
    command:Sender<Command>,
    result:Receiver<CommandResult>,
}

impl CommandSender{
    pub fn create_window(&self,window_class:&WindowClass,window_subclass_args:&WindowSubclassArguments,window_attributes:WindowAttributes)->CommandResult{
        if self.command.send(Command::CreateWindow{
            window_class:window_class as *const WindowClass,
            window_subclass_args:window_subclass_args as *const WindowSubclassArguments,
            window_attributes,
        }).is_ok(){
            if let Ok(window)=self.result.recv(){
                window
            }
            else{
                CommandResult::None
            }
        }
        else{
            CommandResult::None
        }
    }

    pub fn call_break(&self){
        self.command.send(Command::Break);
    }
}

pub struct CommandReceiver{
    command:Receiver<Command>,
    result:Sender<CommandResult>,
}

pub fn command_channel()->(CommandSender,CommandReceiver){
    let (command_send,command_receive)=channel();
    let (result_send,result_receive)=channel();
    (
        CommandSender{
            command:command_send,
            result:result_receive,
        },
        CommandReceiver{
            command:command_receive,
            result:result_send,
        }
    )
}