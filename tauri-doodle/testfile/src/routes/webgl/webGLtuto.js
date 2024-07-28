// https://developer.mozilla.org/ko/docs/Web/API/WebGL_API/Tutorial/Getting_started_with_WebGL
// https://developer.mozilla.org/ko/docs/Web/API/WebGL_API/Tutorial/Creating_3D_objects_using_WebGL
let gl; // A global variable for the WebGL context

export function start(){
    let canvas = document.getElementById("glcanvas");

    gl = initWebGL(canvas);

    if(gl) {
        gl.clearColor(0,0,0,1.0);
        gl.enable(gl.DEPTH_TEST);
        gl.depthFunc(gl.LEQUAL);
        gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
    }
}

function initWebGL(canvas) {
    gl = null;

    try {
        // Try to grab the standard context. If it fails, fallback to experimental.
        gl = canvas.getContext("webgl") || canvas.getContext("experimental-webgl");
    } catch(e) {}

    // If we don't have a GL context, give up now
    if (!gl) {
        alert("unable to initialize WebGL. Your browser may not support it.");
        gl = null;
    }

    return gl;
}