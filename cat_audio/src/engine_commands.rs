use super::{MonoTrack,TrackSet};

/// Команды аудио системы.
/// 
/// Audio system commands.
pub enum AudioEngineCommand{
// ХРАНИЛИЩЕ \\
    /// Добавляет одноканальный трек в ячейку хранилища.
    /// 
    /// Если нет такой ячейки, то ничего не происходит.
    /// 
    /// Adds a mono-channel track to a storage slot.
    /// 
    /// If there is no such slot, nothing happens.
    AddMono(MonoTrack,usize),

    /// Добавляет несколько одноканальных треков в ячейки хранилища.
    /// 
    /// Если нет таких ячеек, то ничего не происходит.
    /// 
    /// Adds some mono-channel tracks to the storage.
    /// 
    /// If there is no such slots, nothing happens.
    AddMonos(Vec<(MonoTrack,usize)>),

    /// Убирает одноканальный трек из хранилища.
    /// 
    /// Если нет такого трека, то ничего не происходит.
    /// 
    /// Removes a mono-channel track from the storage.
    /// 
    /// If there is no such track, nothing happens.
    RemoveMono(usize),

    /// Убирает несколько одноканальных треков из хранилища.
    /// 
    /// Если нет таких треков, то ничего не происходит.
    /// 
    /// Removes some mono-channel tracks from the storage.
    /// 
    /// If there are no such tracks, nothing happens.
    RemoveMonos(Vec<usize>),

    /// Снимает паузу с треков из плейлиста,
    /// привязанных к треку из хранилища.
    /// 
    /// Unpauses tracks from the playlist
    /// attached to a track from the storage.
    UnpauseMonoFromStorage(usize),

    /// Снимает паузу с треков из плейлиста,
    /// привязанных к трекам из хранилища.
    /// 
    /// Unpauses tracks from the playlist
    /// attached to tracks from the storage.
    UnpauseMonosFromStorage(Vec<usize>),

    /// Ставит на паузу треки из плейлиста,
    /// привязанные к треку из хранилища.
    /// 
    /// Pauses tracks from the playlist
    /// attached to a track from the storage.
    PauseMonoFromStorage(usize),

    /// Ставит на паузу треки из плейлиста,
    /// привязанные к трекам из хранилища.
    /// 
    /// Stops tracks from the playlist
    /// attached to tracks from the storage.
    PauseMonosFromStorage(Vec<usize>),

    /// Останавливает треки из плейлиста,
    /// привязанные к треку из хранилища.
    /// 
    /// Stops tracks from the playlist
    /// attached to a track from the storage.
    StopMonoFromStorage(usize),

    /// Останавливает треки из плейлиста,
    /// привязанные к трекам из хранилища.
    /// 
    /// Stops tracks from the playlist
    /// attached to tracks from the storage.
    StopMonosFromStorage(Vec<usize>),

    /// Устанавливает громкость треков из плейлиста,
    /// привязанных к треку из хранилища.
    /// 
    /// Sets a volume of tracks from the playlist
    /// attached to a track from the storage.
    SetMonoVolumeFromStorage(usize,f32),

    /// Устанавливает громкость треков из плейлиста,
    /// привязанных к трекам из хранилища.
    /// 
    /// Sets a volume of tracks from the playlist
    /// attached to tracks from the storage.
    SetMonosVolumeFromStorage(Vec<usize>,f32),

    /// Устанавливает громкости треков из плейлиста,
    /// привязанных к трекам из хранилища.
    /// 
    /// Sets volumes of tracks from the playlist
    /// attached to tracks from the storage.
    SetMonosVolumesFromStorage(Vec<(usize,f32)>),

// ПЛЕЙЛИСТ \\
    /// Проигрывает одноканальный трек на данных каналах.
    /// 
    /// Plays a mono-channel track on the given channels.
    PlayMonoOnChannels(TrackSet),

    /// Проигрывает несколько одноканальных треков на данных каналах.
    /// 
    /// Plays some mono-channel tracks on the given channels.
    PlayMonosOnChannels(Vec<TrackSet>),

    /// Снимает с паузы трек из плейлиста.
    /// 
    /// Если уже проигрывается, ничего не происходит.
    UnpauseMonoFromPlaylist(usize),

    /// Снимает с паузы трек из плейлиста.
    /// 
    /// Если уже проигрывается, ничего не происходит.
    UnpauseMonosFromPlaylist(Vec<usize>),

    /// Ставит трек из плейлиста на паузу.
    /// 
    /// Если уже на паузе, ничего не происходит.
    PauseMonoFromPlaylist(usize),

    PauseMonosFromPlaylist(Vec<usize>),

    /// Убирает одноканальный трек из плейлиста.
    /// 
    /// Removes a mono-channel track from the playlist.
    RemoveMonoFromPlaylist(usize),

    RemoveMonosFromPlaylist(Vec<usize>),

    /// Отчищает плейлист (список текущих играющих треков).
    /// 
    /// Clears a playlist (the list of currently playing tracks).
    ClearPlaylist,

// Параметры \\
    /// Устанавливает громкость трека в плейлисте.
    /// 
    /// Sets a volume to a track in the playlist.
    SetMonoVolume(usize,f32),

    /// Устанавливает громкость треков в плейлисте.
    /// 
    /// Sets a volume to tracks in the playlist.
    SetMonosVolume(Vec<usize>,f32),

    /// Устанавливает громкости треков в плейлисте.
    /// 
    /// Sets volumes to tracks in the playlist.
    SetMonosVolumes(Vec<(usize,f32)>),

    /// Устанавливает общую громкость.
    /// 
    /// Sets the general volume.
    SetGeneralVolume(f32),

// Остальное \\
    /// Закрывает аудио поток.
    /// 
    /// Closes the audio thead.
    Close,
}

unsafe impl std::marker::Sync for AudioEngineCommand{}
unsafe impl std::marker::Send for AudioEngineCommand{}

/// Результат выполнения команды. The result of command accomplishing.
#[derive(Clone,Debug,PartialEq)]
pub enum AudioCommandResult{
    /// Команда отправлена.
    /// 
    /// A command is sent.
    Sent,

    Index(usize),
    Indices(Vec<usize>),

    /// Аудио поток остановлен.
    /// 
    /// The audio thread is closed.
    ThreadClosed,
    /// Хранилище треков переполнено.
    /// 
    /// The track storage is overflown.
    StorageOverflow,
    // /// Хранилище треков пусто.
    // /// 
    // /// The track storage is empty.
    // StorageIsEmpty,

    /// Нет такого трека (в хранилище или плейлисте).
    /// 
    /// No such track (in the storage or playlist).
    NoSuchTrack,
}


impl AudioCommandResult{
    /// Паникует, если результат не `Ok`.
    /// 
    /// Panics if the result isn't `Ok`.
    pub fn unwrap(self){
        match self{
            AudioCommandResult::ThreadClosed |
                AudioCommandResult::StorageOverflow |
                    AudioCommandResult::NoSuchTrack=>
                        panic!("{:?}",self),
            _=>{}
        }
    }

    /// Паникует и выводит сообщение, если результат не `Ok`.
    /// 
    /// Panics и prints the message if the result isn't `Ok`.
    pub fn expect(self,msg:&str){
        if self!=AudioCommandResult::Sent{
            panic!("{} {:?}",msg,self)
        }
    }
}