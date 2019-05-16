use rustler::rustler_export_nifs;
use rustler::schedule::SchedulerFlags;

// пример за нишки, виж threads.rs
mod threads;

// пример за мръсни нифове, виж dirty.rs
mod dirty;

// пример за schedule nif, виж schedule.rs
mod schedule;

mod atoms {
    rustler::rustler_atoms! {
        atom ok;
        atom work;
        atom nil;
    }
}

rustler_export_nifs! {
    "Elixir.Scheduling",
    [
        ("echo_from_thread", 1, threads::echo_from_thread, SchedulerFlags::Normal),
        ("dirty_nif", 0, dirty::dirty_nif, SchedulerFlags::DirtyIo),
        ("do_work", 0, schedule::do_work),
    ],
    None
}
