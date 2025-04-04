#[cfg(debug_assertions)] #[rustfmt::skip] mod _0 { const __: &str = "zellij -l zellij.kdl"; }
use std::{collections::BTreeMap, io, path::Path};
use zellij_tile::prelude::*;
use zellij_utils::consts::ZELLIJ_SOCK_DIR;

#[derive(Default)]
struct State {}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        // runs once on plugin load, provides the configuration with which this plugin was loaded
        // (if any)
        //
        // this is a good place to `subscribe` (https://docs.rs/zellij-tile/latest/zellij_tile/shim/fn.subscribe.html)
        // to `Event`s (https://docs.rs/zellij-tile/latest/zellij_tile/prelude/enum.Event.html)
        // and `request_permissions` (https://docs.rs/zellij-tile/latest/zellij_tile/shim/fn.request_permission.html)

        // Assuming things do run only once, per zellij process, ...
        fn ping_sock(p: &Path) -> Result<(), std::io::Error> {
            std::os::unix::net::UnixStream::connect(p)?;
            Ok(())
        }

        fn p2_spawn_bg() {}
    }
    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::ModeUpdate(_mi) => false,
            Event::TabUpdate(_upvec) => false,
            Event::PaneUpdate(_manifest) => false,
            Event::Key(_key) => false,
            Event::Mouse(_mouse) => false,
            Event::Timer(_dur) => false,
            Event::CopyToClipboard(_buff) => false,
            Event::SystemClipboardFailure => false,
            Event::InputReceived => false,
            Event::Visible(_isvis) => false,
            Event::CustomMessage(_src, _ctx) => false,
            Event::FileSystemCreate(_iovec /* : Vec<(PathBuf, Option<FileMetadata>)> */) => false,
            Event::FileSystemRead(_iovec /* : Vec<(PathBuf, Option<FileMetadata>)> */) => false,
            Event::FileSystemUpdate(_iovec /* : Vec<(PathBuf, Option<FileMetadata>)> */) => false,
            Event::FileSystemDelete(_iovec /* : Vec<(PathBuf, Option<FileMetadata>)> */) => false,
            Event::PermissionRequestResult(_res /* : PermissionStatus */) => false,
            Event::SessionUpdate(
                _seshvec, /* : Vec<SessionInfo> */
                _datavec, /* : Vec<(String, Duration)> */
            ) => false,
            Event::RunCommandResult(
                _opc, /* : Option<i32> */
                _out, /* : Vec<u8> */
                _err, /* : Vec<u8> */
                _unk, /* : BTreeMap<String, String> */
            ) => false,
            Event::WebRequestResult(
                _opc, /* : u16*/
                _btm, /* : BTreeMap<String, String>*/
                _dat, /* : Vec<u8> */
                _unk, /* : BTreeMap<String, String>*/
            ) => false,
            Event::CommandPaneOpened(_unk0, _unk1) => false,
            Event::CommandPaneExited(_op, _unk1, _unk2) => false,
            Event::PaneClosed(p) => match p {
                PaneId::Plugin(_pid) => false,
                PaneId::Terminal(_tid) => false,
            },
            Event::EditPaneOpened(_unk0, _unk1) => false,
            Event::EditPaneExited(_unk0, _unk1, _unk2) => false,
            Event::CommandPaneReRun(_timeout_mayb, _unk1) => false,
            Event::FailedToWriteConfigToDisk(_reason_str) => false,
            Event::ListClients(_clientvec) => false,
            _ => panic!("Uncovered scenario, check zellij-tile version against variants"),
        }
    }
    fn pipe(&mut self, pm: PipeMessage) -> bool {
        match pm.source {
            PipeSource::Cli(_id_cli_pipe) => false,
            PipeSource::Plugin(_id_source_plugin) => false,
            _ => false,
        }
    }
    fn render(&mut self, rows: usize, cols: usize) {
        println!("Hi there! I have {rows} rows and {cols} columns");
    }
}
