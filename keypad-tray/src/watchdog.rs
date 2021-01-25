use std::{
    sync::mpsc::{channel, Sender},
    thread::{sleep, spawn},
    time::Duration,
};

use native_windows_gui::NoticeSender;
use sysinfo::{RefreshKind, System, SystemExt};

use crate::models::Profile;

pub struct WatchDog {
    shutdown: Sender<()>,
}

const WATCH_INTERVAL_SECONDS: u64 = 1;

impl WatchDog {
    pub fn start(profiles: Vec<Profile>, notice: NoticeSender) -> Self {
        let (tx, rx) = channel::<()>();
        spawn(move || {
            let mut system = System::new_with_specifics(RefreshKind::new().with_processes());
            let mut shutdown = rx.try_iter();
            let mut switcher = match AutoSwitcher::new(profiles) {
                Some(switcher) => switcher,
                None => return (),
            };

            loop {
                if shutdown.next().is_some() {
                    break;
                }

                system.refresh_processes();
                match switcher.next_profile(&system) {
                    Some(profile) => {
                        apply_profile(profile);
                        notice.notice();
                    }
                    None => {}
                }

                sleep(Duration::from_secs(WATCH_INTERVAL_SECONDS));
            }
        });

        Self { shutdown: tx }
    }

    pub fn stop(self) {
        let _ = self.shutdown.send(());
    }
}

fn apply_profile(profile: Profile) {
    let _ = keypad::Keypad::auto_detect().and_then(|mut k| k.send_combos_to_device(profile.combos));
}

struct AutoSwitcher {
    default: Profile,
    auto_profiles: Vec<Profile>,
    state: State,
}

impl AutoSwitcher {
    fn new(mut profiles: Vec<Profile>) -> Option<Self> {
        if profiles.len() == 0 {
            return None;
        }

        let default = profiles.remove(0);
        let auto_profiles = profiles
            .into_iter()
            .filter(|p| {
                p.auto_launch_program.is_some()
                    && p.auto_launch_program.as_ref().unwrap().trim().len() > 0
            })
            .collect();
        Some(Self {
            state: State::Default,
            default,
            auto_profiles,
        })
    }

    fn next_profile(&mut self, system: &System) -> Option<Profile> {
        match self.state {
            State::Default => {
                let profile = self
                    .auto_profiles
                    .iter()
                    .find(|p| {
                        let name = p.auto_launch_program.as_ref().unwrap();
                        program_is_running(name.as_ref(), system)
                    })
                    .map(|p| p.clone());
                if profile.is_some() {
                    self.state = State::InProgram(profile.clone().unwrap());
                }
                profile
            }
            State::InProgram(ref p) => {
                let name = p.auto_launch_program.as_ref().unwrap();
                if !program_is_running(name.as_ref(), system) {
                    // program has stopped, set to default and run again
                    // to check if other programs are running
                    self.state = State::Default;
                    self.next_profile(system).or(Some(self.default.clone()))
                } else {
                    None
                }
            }
        }
    }
}

fn program_is_running(name: &str, system: &System) -> bool {
    system.get_process_by_name(name).len() > 0
}

enum State {
    Default,
    InProgram(Profile),
}
