import init , { WebAppHandle } from './pkg/culsynth_web_ui.js';

const ctx = new AudioContext()
await ctx.audioWorklet.addModule('./processor.js')
const node = new AudioWorkletNode(ctx, 'synth-processor', {
    numberOfInputs: 0,
    numberOfOutputs: 1,
    outputChannelCount: [2],
});
node.connect(ctx.destination)

fetch('./pkg/culsynth_web_audioworklet_bg.wasm').then(r => r.arrayBuffer()).then(r => node.port.postMessage({
    type: 'loadWasm',
    data: r,
}))

let wasm;
wasm = await init();
let app_handle;
app_handle = new WebAppHandle();
app_handle.start(node, ctx);
