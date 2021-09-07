import store from './app/store'
import * as reactApp from './app/App'
export const wasm = import('../app-core/pkg/app-core');

// debug messages for store import
// store.dispatch({ type: 'global/log', payload: '' })
// store.dispatch({ type: 'toolbar/log', payload: '' })

// start react app
reactApp.run();

// get canvas for wasm app
const canvas = document.getElementById('appCanvas');
const gl = canvas.getContext('webgl', { antialias: true });

// init the render-loop
wasm.then(m => {
    if (!gl) {
        alert('Failed to initialize WebGL');
        return;
    }
    
    const FPS_THROTTLE = 1000.0 / 120.0; // milliseconds / frames
    const initialTime = Date.now();
    let lastDrawTime = -1; // milliseconds

    function tick() {
        window.requestAnimationFrame(tick);
        const currTime = Date.now();

        if (currTime >= lastDrawTime + FPS_THROTTLE) {
            lastDrawTime = currTime;
            let elapsedTime = currTime - initialTime;
            m.tick(elapsedTime, window.innerHeight, window.innerWidth);
        }
    }

    tick(); // Let the Games Begin
});

