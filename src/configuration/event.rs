use notify::{Watcher, RecommendedWatcher, RecursiveMode, Result, Config, EventKind, INotifyWatcher};
use notify::event::AccessKind::Close;
use notify::event::AccessMode::Write;

pub fn listen(path: &str) -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher: INotifyWatcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                if event.kind == EventKind::Access(Close(Write)) {
                    println!("event: {:?}", event);
                }
            },
            Err(error) => {
                println!("watch error: {:?}", error);
            }
        }
    }

    Ok(())
}
