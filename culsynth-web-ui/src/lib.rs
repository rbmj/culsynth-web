use culsynth_plugin::editor::Editor;
use culsynth_plugin::{ContextReader, VoiceMode};
use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, AudioWorkletNode};
pub mod webmidihandler;
use webmidihandler::WebMidiHandler;

pub struct WebAudioContext {
    ctx: AudioContext,
    fixed: bool,
    voice_mode: VoiceMode,
}

impl WebAudioContext {
    fn new(ctx: AudioContext) -> Self {
        Self {
            ctx,
            fixed: false,
            voice_mode: VoiceMode::Mono,
        }
    }
    fn resume(&self) {
        let _ = self.ctx.resume();
        log::info!("Starting context");
    }
}

impl ContextReader for WebAudioContext {
    fn sample_rate(&self) -> u32 {
        self.ctx.sample_rate() as u32
    }
    fn is_fixed(&self) -> bool {
        self.fixed
    }
    fn bufsz(&self) -> usize {
        128
    }
    fn voice_mode(&self) -> VoiceMode {
        self.voice_mode
    }
}

pub struct SynthApp {
    audio_context: WebAudioContext,
    editor: Editor,
    midi_handler: WebMidiHandler,
    has_resumed: bool,
}

impl SynthApp {
    /// Called once before the first frame.
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        audioctx: WebAudioContext,
        node: AudioWorkletNode,
    ) -> Self {
        let mut editor = Editor::new();
        editor.initialize(&cc.egui_ctx);
        Self {
            audio_context: audioctx,
            editor,
            midi_handler: WebMidiHandler::new(node),
            has_resumed: false,
        }
    }
}

impl eframe::App for SynthApp {
    /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        self.editor.update(
            ctx,
            &self.audio_context,
            &self.midi_handler.get_params(),
            self.midi_handler.get_tuning(),
            &self.midi_handler.get_matrix(),
            &self.midi_handler,
            Editor::null_sender(),
        );
        if !self.has_resumed && self.midi_handler.has_interacted() {
            self.audio_context.resume();
            self.has_resumed = true;
        }
    }
}

#[wasm_bindgen]
pub struct WebAppHandle {
    runner: eframe::WebRunner,
}

#[wasm_bindgen]
impl WebAppHandle {
    /// Installs a panic hook, then returns.
    #[expect(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Redirect [`log`] message to `console.log` and friends:
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();

        Self {
            runner: eframe::WebRunner::new(),
        }
    }

    /// Call this once from JavaScript to start your app.
    #[wasm_bindgen]
    pub fn start(&self, node: AudioWorkletNode, ctx: AudioContext) {
        let runner = self.runner.clone();
        wasm_bindgen_futures::spawn_local(async move { run_app(runner, node, ctx).await });
    }

    /// Shut down eframe and clean up resources.
    #[wasm_bindgen]
    pub fn destroy(&self) {
        self.runner.destroy();
    }

    /// The JavaScript can check whether or not your app has crashed:
    #[wasm_bindgen]
    pub fn has_panicked(&self) -> bool {
        self.runner.has_panicked()
    }

    #[wasm_bindgen]
    pub fn panic_message(&self) -> Option<String> {
        self.runner.panic_summary().map(|s| s.message())
    }

    #[wasm_bindgen]
    pub fn panic_callstack(&self) -> Option<String> {
        self.runner.panic_summary().map(|s| s.callstack())
    }
}

/// Helper to actually run the app
async fn run_app(runner: eframe::WebRunner, node: AudioWorkletNode, ctx: AudioContext) {
    let web_options = eframe::WebOptions::default();
    let document = web_sys::window().expect("No window").document().expect("No document");
    let canvas = document
        .get_element_by_id("culsynth_canvas")
        .expect("Failed to find drawing canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Canvas was not a HtmlCanvasElement");

    let context = WebAudioContext::new(ctx);

    let start_result = runner
        .start(
            canvas,
            web_options,
            Box::new(move |cc| Ok(Box::new(SynthApp::new(cc, context, node)))),
        )
        .await;
    // Remove the loading text and spinner:
    if let Some(loading_text) = document.get_element_by_id("loading_text") {
        match start_result {
            Ok(_) => {
                loading_text.remove();
            }
            Err(e) => {
                loading_text.set_inner_html(
                    "<p> The app has crashed. See the developer console for details. </p>",
                );
                panic!("Failed to start eframe: {e:?}");
            }
        }
    }
}
