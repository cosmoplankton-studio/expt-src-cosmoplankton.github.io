import '../static/common.css';
import store from './app-ux/store'
import * as appUx from './app-ux/App-ux'
const appCore = import('../app-core/pkg/app-core');

// debug messages for store import
// store.dispatch({ type: 'global/log', payload: '' })
// store.dispatch({ type: 'toolbar/log', payload: '' })

class WebApp {
    title = '';
    overlayMount;
    canvas;
    glCtx;

    constructor(title) {
        this.title = title;
    }

    initCanvas() {
        // get canvas for wasm app
        this.canvas = document.getElementById('appCanvas');
        this.glCtx = this.canvas.getContext('webgl', { antialias: true });
        console.log('init done')
    }

    initOverlay() {
        this.overlayMount = document.getElementById('reactApp');
    }

    runCanvas() {

        // init the render-loop
        appCore.then(m => {
            if (!this.glCtx) {
                alert('Failed to initialize WebGL');
                return;
            }
            
            console.log('run start')

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
    }

    runOverlay() {
        // start react app
        appUx.run(store, this.overlayMount);
    }

    pauseUpdate(pause) {
        appCore.then(m => { m.pause_update(pause) });
    }
}
const webApp = new WebApp('cosmoplankton');

export default webApp;
