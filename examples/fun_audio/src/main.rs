pub use makepad_component::{self, *};
pub use makepad_platform::{self, *, audio::*, midi::*};

mod piano;
mod audio;
use crate::piano::*;
use crate::audio::*;

live_register!{
    use AudioComponent::*;
    use FrameComponent::*;
    use makepad_component::theme::*;
    use makepad_platform::shader::std::*;
    App: {{App}} {
        window: {pass: {clear_color: (COLOR_BG_APP)}}
        audio_graph: {
            root: Mixer {
                /*c0: BasicSynth {
                    plugin: "AUMIDISynth"
                    preset_data: "21adslkfjalkwqwe"
                }*/
                c1: Instrument {
                    key_range: {start: 34, end: 47 shift: 30}
                    PluginEffect {
                        plugin: "AUReverb2"
                    }
                    PluginMusicDevice {
                        plugin: "AUMIDISynth"
                    }
                }
            }
        }
        
        frame: {
            color: (COLOR_BG_APP)
            walk: {width: Size::Fill, height: Size::Fill}
            layout: {
                padding: 8
                align: {x: 0.0, y: 0.0}
                spacing: 30.,
                flow: Flow::Down
            },
            Frame {
                layout: {flow: Flow::Right, spacing: 5.0}
                walk: {margin: {left: 60}, height: Size::Fit}
                Button {label: "+  Band"}
                Button {label: "<"}
                Button {label: ">"}
                Frame {
                    walk: {width: Size::Fill, height: Size::Fixed(36)}
                    color: #f00
                    bg_quad: {
                        const WAVE_HEIGHT: 0.15
                        const WAVE_FREQ: 0.2
                        fn pixel(self) -> vec4 {
                            let offset_y = 1.5;
                            let pos2 = vec2(self.pos.x, self.pos.y + WAVE_HEIGHT * sin(WAVE_FREQ * self.pos.x * self.rect_size.x));
                            let sdf = Sdf2d::viewport(pos2 * self.rect_size);
                            sdf.clear(#2f)
                            sdf.move_to(0., self.rect_size.y * 0.5);
                            sdf.line_to(self.rect_size.x, self.rect_size.y * 0.5);
                            return sdf.stroke(#f, 1.0)
                        }
                    }
                }
            }
            piano: = Piano {}
            Splitter {
                walk: {width: Size::Fill, height: 100}
                a: Frame {
                    color: #f00
                }
                b: Frame {
                    color:#fff
                }
            }
            /*Frame {
                layout: {flow: Flow::Right, spacing: 30.}
                Frame {color: #0f0, width: Size::Fill, height: 40}
                Frame {
                    color: #0ff
                    layout: {
                        padding: 10
                        flow: Flow::Down
                        spacing: 10
                    }
                    width: Size::Fixed(100)
                    height: 300
                    Frame {color: #00f, width: 40, height: Size::Fill}
                    Frame {color: #f00, width: 40, height: 40}
                    Frame {color: #00f, width: 40, height: 40}
                }
                Frame {color: #f00, width: 40, height: 40}
                Frame {color: #f0f, width: Size::Fill, height: 60}
                Frame {color: #f00, width: 40, height: 40}
            }*/
        }
        
        scroll_view: {
            h_show: true,
            v_show: true,
            view: {}
        }
    }
}
main_app!(App);

#[derive(Live, LiveHook)]
pub struct App {
    frame: Frame,
    audio_graph: AudioGraph,
    window: BareWindow,
    scroll_view: ScrollView,
}

impl App {
    pub fn live_register(cx: &mut Cx) {
        makepad_component::live_register(cx);
        crate::audio::live_register(cx);
        crate::piano::live_register(cx);
    }
    
    pub fn new_app(cx: &mut Cx) -> Self {
        Self::new_as_main_module(cx, &module_path!(), id!(App)).unwrap()
    }
    
    pub fn handle_event(&mut self, cx: &mut Cx, event: &mut Event) {
        
        //self.desktop_window.handle_event(cx, event);
        self.scroll_view.handle_event(cx, event);
        
        for item in self.frame.handle_event(cx, event) {
            match item.id {
                id!(piano) => if let PianoAction::Note {is_on, note_number, velocity} = item.action.cast() {
                    self.audio_graph.send_midi_1_data(Midi1Note {
                        is_on,
                        note_number,
                        channel: 0,
                        velocity
                    }.into());
                }
                _ => ()
            }
        }
        
        for action in self.audio_graph.handle_event(cx, event) {
            match action {
                AudioGraphAction::Midi1Data(data) => if let Midi1Event::Note(note) = data.decode() {
                    let piano = self.frame.child_mut::<Piano>(id!(piano)).unwrap();
                    piano.set_note(cx, note.is_on, note.note_number)
                }
            }
        };
        
        //let instrument = self.instrument.clone();
        /*
        for action in self.piano.handle_event(cx, event) {
            match action {
                PianoAction::Note {is_on, note_number, velocity} => {
                    self.audio_graph.send_midi_1_data(Midi1Note {
                        is_on,
                        note_number,
                        channel: 0,
                        velocity
                    }.into());
                }
            }
        };*/
        
        match event {
            Event::KeyDown(ke) => {
                if let KeyCode::F1 = ke.key_code {
                }
                if let KeyCode::Escape = ke.key_code {
                }
                
            }
            Event::Draw(draw_event) => {
                self.draw(&mut Cx2d::new(cx, draw_event));
                //self.piano.set_key_focus(cx);
            }
            _ => ()
        }
    }
    
    pub fn draw(&mut self, cx: &mut Cx2d) {
        if self.window.begin(cx).is_err() {
            return;
        }
        
        //self.piano.draw(cx);
        while self.frame.draw(cx).is_err() {};
        /*
        if self.scroll_view.begin(cx).is_ok() {
            self.scroll_view.end(cx);
        }*/
        
        
        self.window.end(cx);
    }
}