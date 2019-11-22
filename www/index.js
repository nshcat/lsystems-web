
import { LSystemInterface, DrawOperation, DrawingParameters } from "lsystems-web";
import { memory } from "lsystems-web/lsystems_web_bg";
import * as three from "three";
import {TrackballControls} from "three/examples/jsm/controls/TrackballControls";
import "jquery";
import "popper.js";
import "bootstrap-select";
import "bootstrap-slider"
import "bootstrap-colorpicker"
import "bootstrap-colorpicker/dist/css/bootstrap-colorpicker.min.css"

var $ = require("jquery");

var ruleDivCounter = 0;
var interpDivCounter = 0;

var startX = 0;
var startY = 0;

var colorPalette = [ new three.Color(255, 255, 255), new three.Color(255, 255, 255), new three.Color(255, 255, 255) ];

var drawWireframe = false;

$('#slide-StartAngle').on('input',function () {
    $("#input-StartAngle").val($(this).val());
    refreshDrawingParameters();
});
$('#input-StartAngle').on('input', function(){
    $('#slide-StartAngle').val($(this).val());
    refreshDrawingParameters();
});
$('#slide-AngleDelta').on('input',function () {
    $("#input-AngleDelta").val($(this).val());
    refreshDrawingParameters();
});
$('#input-AngleDelta').on('input', function(){
    $('#slide-AngleDelta').val($(this).val());
    refreshDrawingParameters();
});
$('#slide-Step').on('input',function () {
    $("#input-Step").val($(this).val());
    refreshDrawingParameters();
});
$('#input-Step').on('input', function(){
    $('#slide-Step').val($(this).val());
    refreshDrawingParameters();
});
$('#slide-Iters').on('input',function () {
    $("#input-Iters").val($(this).val());
    refreshDrawingParameters();
});
$('#input-Iters').on('input', function(){
    $('#slide-Iters').val($(this).val());
    refreshDrawingParameters();
});


$('#addRuleDivBtt').on('click', function(){
   addRuleDiv();
});

$('#addInterpDivBtt').on('click', function(){
    addInterpDiv();
});

$('#refreshBtt').on('click', function(){
    extractLSystemConfig();
});

$('#inputAxiom').on('input', function () {
    extractLSystemConfig();
});

$('#input-Iters').on('input', function () {
    extractLSystemConfig();
});
$('#slide-Iters').on('input', function () {
    extractLSystemConfig();
});

$('#preset0').on('click', function () {
    loadPreset(0);
    extractLSystemConfig();
});

$('#preset1').on('click', function () {
    loadPreset(1);
    extractLSystemConfig();
});

$('#preset2').on('click', function () {
    loadPreset(2);
    extractLSystemConfig();
});

$('#preset3').on('click', function () {
    loadPreset(3);
    extractLSystemConfig();
});

$('#preset4').on('click', function () {
    loadPreset(4);
    extractLSystemConfig();
});

$('#preset5').on('click', function () {
    loadPreset(5);
    extractLSystemConfig();
});

$('#preset6').on('click', function () {
    loadPreset(6);
    extractLSystemConfig();
});

$('#chkBoxWireframe').on('click', function () {
    drawWireframe = $('#chkBoxWireframe').is(":checked");
    refreshScene();
});

$('#color1').colorpicker();

$('#color1').on('changeColor', function(event) {
    $('#color1').css('background-color', event.color.toString());
    const clr = event.color.toRGB();
    colorPalette[0] = new three.Color(clr.r/255, clr.g/255, clr.b/255);
    refreshScene();
});

$('#color2').colorpicker();

$('#color2').on('changeColor', function(event) {
    $('#color2').css('background-color', event.color.toString());
    const clr = event.color.toRGB();
    colorPalette[1] = new three.Color(clr.r/255, clr.g/255, clr.b/255);
    refreshScene();
});

$('#color3').colorpicker();

$('#color3').on('changeColor', function(event) {
    $('#color3').css('background-color', event.color.toString());
    const clr = event.color.toRGB();
    colorPalette[2] = new three.Color(clr.r/255, clr.g/255, clr.b/255);
    refreshScene();
});


var canvas = document.getElementById('rendertarget');
//canvas.style.width ='100%';
//canvas.style.height='100%';
// ...then set the internal size to match
canvas.width  = canvas.offsetWidth;
canvas.height = canvas.offsetHeight;

var scene = new three.Scene();
var camera = new three.PerspectiveCamera(75, canvas.width / canvas.height, 0.1, 1000);

camera.position.set(0, 0, 1);

var renderer = new three.WebGLRenderer({ canvas: canvas });
//document.body.appendChild(renderer.domElement);

var controls = new TrackballControls(camera, renderer.domElement);
controls.rotateSpeed = 1.0;
controls.zoomSpeed = 1.2;
controls.panSpeed = 0.8;
controls.staticMoving = false;
controls.dynamicDampingFactor = 0.3;
controls.keys = [ 65, 83, 68 ];

camera.position.z = 5;

var drawingParms = DrawingParameters.new();
drawingParms.set_angle_delta_degrees(60.0);

//loadDefaultRulesAndInterp();

var lsystem = LSystemInterface.new();

loadPreset(3);

window.addEventListener('resize', onWindowResize, false);

function onWindowResize() {
    canvas.width  = canvas.offsetWidth;
    canvas.height = canvas.offsetHeight;

    renderer.setSize(canvas.width, canvas.height);

    camera.aspect = canvas.width / canvas.height;
    camera.updateProjectionMatrix();
}


function drawPolygons(polygons) {
    var i = 0;

    while(i < polygons.length) {
        // Read number of vertices
        const vertexCount = polygons[i];
        i++;

        // Read color
        const colorIndex = polygons[i];
        const color = colorPalette[colorIndex];
        i++;

        console.log("Color Index:", colorIndex, "Color:", color);

        // Read vertices
        var vertices = new Array(vertexCount);

        for(var j = 0; j < vertexCount; ++j) {
            var vec = new three.Vector3(
                polygons[ i + (j*3) + 0],
                polygons[ i + (j*3) + 1],
                polygons[ i + (j*3) + 2],
            );

            vertices[j] = vec;
        }

        var geometry = new three.BufferGeometry();
        var rawVertices = [];

        for(var j = 0; j < vertexCount; ++j) {
            var vertex = vertices[j];

            rawVertices.push(vertex.x);
            rawVertices.push(-vertex.y);
            rawVertices.push(vertex.z);
        }

        geometry.setAttribute('position', new three.Float32BufferAttribute(rawVertices, 3));

        geometry.computeVertexNormals();

        if(drawWireframe) {
            var material = new three.MeshPhongMaterial( {
                color: color,
                polygonOffset: true,
                polygonOffsetFactor: 1, // positive value pushes polygon further away
                polygonOffsetUnits: 1
            } );

            material.side = three.DoubleSide;
            var mesh = new three.Mesh(geometry, material);
            scene.add(mesh);

            var geo = new three.EdgesGeometry( mesh.geometry ); // or WireframeGeometry
            var mat = new three.LineBasicMaterial( { color: 0xffffff, linewidth: 2 } );
            var wireframe = new three.LineSegments( geo, mat );
            mesh.add( wireframe );

        } else {
            var material = new three.MeshPhongMaterial({ ambient: color, color: color, specular: color, shininess: 30 });
            material.side = three.DoubleSide;
            var mesh = new three.Mesh(geometry, material);
            mesh.drawMode = three.TriangleFanDrawMode;

            scene.add(mesh);
        }

        i += vertexCount * 3;
    }
}

function refreshScene() {
    scene = new three.Scene();

    var light = new three.AmbientLight(0x404040);
    var light2 = new three.DirectionalLight(0x404040);
    light2.position.set( 0, 1, 1 ).normalize();

    scene.add(light);
    scene.add(light2);


    retrieveLines();
    retrievePolygons();
}

function retrievePolygons() {
    const polyPtr = lsystem.retrieve_polygons();
    const polyLen = lsystem.retrieve_polygons_length();
    const buffer = new Float64Array(memory.buffer, polyPtr, polyLen);
    drawPolygons(buffer)
}


function retrieveLines() {
    const linePtr = lsystem.retrieve_lines();
    const lineLen = lsystem.retrieve_lines_length();
    const buffer = new Float64Array(memory.buffer, linePtr, lineLen);
    drawLines(buffer)
}

function refreshDrawingParameters() {
    var drawingParams = DrawingParameters.new();

    drawingParams.set_angle_delta_degrees($('#slide-AngleDelta').val());
    drawingParams.set_start_angle_degrees($('#slide-StartAngle').val());
    drawingParams.set_step($('#slide-Step').val());
    drawingParams.set_start_position(startX, startY);
    drawingParams.set_color_palette_size(3);

    lsystem.set_draw_parameters(drawingParams);

    // We only need to reinterpret the already iterated axiom, since the changes only affect displaying.
    lsystem.interpret();

    refreshScene();
}

/**
 * handle changes that require full reiteration of the lsystem
 */
function handleRuleInterpChange() {
    extractLSystemConfig();
}


function handleRemoveRule(id) {
    $('#'+id).remove();
    extractLSystemConfig();
}

function addRuleDiv() {
    var id = 'rule' + ruleDivCounter;
    var bttid = 'removerule' + ruleDivCounter;

    var leftsideid = 'ruleleftside' + ruleDivCounter;
    var rightsideid = 'rulerightside' + ruleDivCounter;

    var div = $('<div></div>')
        .prop('class', 'ruleline')
        .prop('id', id)
        .html(`
                 <input id=${leftsideid} class="ruleboxleft" maxlength="1" type="text"/>
                 <span width="40%">-></span>
                 <input id=${rightsideid} class="ruleboxright" type="text"/>
                 <button id=${bttid} type="button" class="btn btn-danger rightbutton">-</button>
            `);

    $('#rules-div').append(div);

    $('#' + leftsideid).on('input', function(){
        extractLSystemConfig();
    });

    $('#' + rightsideid).on('input', function(){
        extractLSystemConfig();
    });

    $('#' + bttid).on('click', function(){
        handleRemoveRule(id);
    });

    ruleDivCounter++;
}



function handleRemoveInterp(id) {
    $('#'+id).remove();
    extractLSystemConfig();
}

function addInterpDiv() {
    var id = 'interp' + interpDivCounter;
    var bttid = 'removeinterp' + interpDivCounter;

    var leftsideid = 'interpleftside' + interpDivCounter;
    var rightsideid = 'interprightside' + interpDivCounter;

    var div = $('<div></div>')
        .prop('class', 'line')
        .prop('id', id)
        .html(`
                 <input id=${leftsideid} class="ruleboxleft" maxlength="1" type="text"/>
                 <span>-></span>
                 <select id=${rightsideid} data-dropup-auto="false">
                    <optgroup label="Movement">
                        <option value="0">Forward</option>
                        <option value="1">Forward (no draw)</option>
                        <option value="7">Forward (contracting)</option>
                        <option value="2">Turn Right</option>
                        <option value="3">Turn Left</option>
                        <option value="9">Pitch Up</option>
                        <option value="8">Pitch Down</option>               
                        <option value="10">Roll Left</option>
                        <option value="11">Roll Right</option>
                        <option value="12">Turn Around</option>
                    </optgroup>
                    <optgroup label="State Handling">
                        <option value="4">Save State</option>
                        <option value="5">Load State</option>    
                    </optgroup>         
                    <optgroup label="Polygons">
                        <option value="13">Begin Polygon</option>
                        <option value="15">Submit Vertex</option>
                        <option value="14">End Polygon</option>       
                    </optgroup>      
                    <optgroup label="Special">       
                        <option value="6">Ignore</option>   
                        <option value="16">Increment Color</option>   
                        <option value="17">Decrement Color</option>   
                    </optgroup>         
                 </select>
                 <button id=${bttid} type="button" class="btn btn-danger rightbutton">-</button>
            `);

    $('#interp-div').append(div);

    $('#' + leftsideid).on('input', function(){
        extractLSystemConfig();
    });

    $('#' + rightsideid).on('change', function(){
        extractLSystemConfig();
    });

    $('#' + bttid).on('click', function(){
        handleRemoveRule(id);
    });

    $('select').selectpicker();

    interpDivCounter++;
}

function setDelta(x) {
    $('#input-AngleDelta').val(x);
    $('#slide-AngleDelta').val(x);
}

function setStart(x) {
    $('#input-StartAngle').val(x);
    $('#slide-StartAngle').val(x);
}

function setStep(x) {
    $('#input-Step').val(x);
    $('#slide-Step').val(x);
}

function setIterations(x) {
    $('#input-Iters').val(x);
    $('#slide-Iters').val(x);
}

function setPosition(x, y) {
    startX = x;
    startY = y;
}

function setAxiom(x) {
    $("#inputAxiom").val(x);
}

function loadPenrose() {
    setStart(55);
    setDelta(36);
    setStep(0.468);
    setIterations(3);
    setPosition(-1, 0);

    setAxiom("[7]++[7]++[7]++[7]++[7]");

    addRule("6", "81++91----71[-81----61]++");
    addRule("7", "+81--91[---61--71]+");
    addRule("8", "-61++71[+++81++91]-");
    addRule("9", "--81++++61[+91++++71]--71");
    addRule("1", "");

    addInterp("6", "0");
    addInterp("7", "0");
    addInterp("8", "0");
    addInterp("9", "0");
    addInterp("-", "3");
    addInterp("+", "2");
    addInterp("[", "4");
    addInterp("]", "5");
}

function loadSirpinski() {
    setStart(60);
    setDelta(120);
    setStep(0.45);
    setIterations(4);
    setPosition(0, -3);

    setAxiom("F-G-G");

    addRule("F", "F-G+F+G-F");
    addRule("G", "GG");

    addInterp("F", "0");
    addInterp("G", "0");
    addInterp("-", "2");
    addInterp("+", "3");
}

function loadLeaf() {
    setStart(60);
    setDelta(15);
    setStep(0.45);
    setIterations(9);
    setPosition(-2, -2);

    setAxiom("G+GG+G[A][B]");

    addRule("A", "[+A{.].C.}");
    addRule("B", "[-B{.].C.}");
    addRule("C", "^FvC");

    addInterp("G", "0");
    addInterp("F", "1");
    addInterp("-", "2");
    addInterp("+", "3");
    addInterp("[", "4");
    addInterp("]", "5");
    addInterp("{", "13");
    addInterp("}", "14");
    addInterp(".", "15");

    addInterp("^", "9");
    addInterp("v", "8");
}

function loadSirpinski2() {
    setStart(60);
    setDelta(120);
    setStep(0.5);
    setIterations(4);
    setPosition(-1, 0);

    setAxiom("F-G-G");

    addRule("F", "F-G+F+G-F");
    addRule("G", "GG");

    addInterp("F", "7");
    addInterp("G", "7");
    addInterp("-", "2");
    addInterp("+", "3");
}

function loadFlower() {
    setStart(270);
    setDelta(23);
    setStep(0.03);
    setIterations(6);
    setPosition(0, 2);

    setAxiom("X");

    addRule("X", "F+[[X]-X]-F[-FX]+X");
    addRule("F", "FF");

    addInterp("F", "0");
    addInterp("X", "6");
    addInterp("-", "3");
    addInterp("+", "2");
    addInterp("[", "4");
    addInterp("]", "5");
}

function loadKoch2() {
    setStart(0);
    setDelta(60);
    setStep(0.33333);
    setIterations(3);
    setPosition(-1, -1);

    setAxiom("F--F--F");

    addRule("F", "F+F--F+F");

    addInterp("F", "7");
    addInterp("-", "2");
    addInterp("+", "3");
}

function loadKoch() {
    setStart(0);
    setDelta(60);
    setStep(0.2);
    setIterations(3);
    setPosition(-3, -2);

    setAxiom("F--F--F");

    addRule("F", "F+F--F+F");

    addInterp("F", "0");
    addInterp("-", "2");
    addInterp("+", "3");
}

function loadPreset(idx) {
    $('#interp-div').empty();
    $('#rules-div').empty();

    interpDivCounter = 0;
    ruleDivCounter = 0;


    switch(idx) {
        case 0:
            loadFlower();
            break;
        case 1:
            loadKoch();
            break;
        case 4:
            loadKoch2();
            break;
        case 2:
            loadSirpinski();
            break;
        case 5:
            loadSirpinski2();
            break;
        case 3:
            loadPenrose();
            break;
        case 6:
            loadLeaf();
            break;
    }
}





/*function loadDefaultRulesAndInterp() {
    $('#interp-div').empty();
    $('#rules-div').empty();

    interpDivCounter = 0;
    ruleDivCounter = 0;

    $("#inputAxiom").val("X");

    addRule("X", "F+[[X]-X]-F[-FX]+X");
    addRule("F", "FF");

    addInterp("F", "0");
    addInterp("X", "6");
    addInterp("-", "3");
    addInterp("+", "2");
    addInterp("[", "4");
    addInterp("]", "5");
}*/

function addRule(left, right) {
    addRuleDiv();
    var idx = ruleDivCounter - 1;

    $('#ruleleftside' + idx).val(left);
    $('#rulerightside' + idx).val(right);
}

function addInterp(left, right) {
    addInterpDiv();
    var idx = interpDivCounter - 1;

    $('#interpleftside' + idx).val(left);
    $('#interprightside' + idx).val(right);
    $('#interprightside' + idx).selectpicker('render');
}

function retrieveDrawOperation(input) {
    switch (input) {
        case "0":
            return DrawOperation.Forward;
        case "1":
            return DrawOperation.ForwardNoDraw;
        case "2":
            return DrawOperation.TurnRight;
        case "3":
            return DrawOperation.TurnLeft;
        case "4":
            return DrawOperation.SaveState;
        case "5":
            return DrawOperation.LoadState;
        case "6":
            return DrawOperation.Ignore;
        case "7":
            return DrawOperation.ForwardContracting;
        case "8":
            return DrawOperation.PitchDown;
        case "9":
            return DrawOperation.PitchUp;
        case "10":
            return DrawOperation.RollLeft;
        case "11":
            return DrawOperation.RollRight;
        case "12":
            return DrawOperation.TurnAround;
        case "13":
            return DrawOperation.BeginPolygon;
        case "14":
            return DrawOperation.EndPolygon;
        case "15":
            return DrawOperation.SubmitVertex;
        case "16":
            return DrawOperation.IncrementColor;
        case "17":
            return DrawOperation.DecrementColor;
    }
}



function extractLSystemConfig() {
    lsystem.clear();

    // Retrieve axiom
    lsystem.set_axiom($('#inputAxiom').val());

    // Retrieve iteration count
    lsystem.set_iterations($('#slide-Iters').val());

    // Retrieve rules
    var rulesDiv = document.getElementById('rules-div');
    var numRules = rulesDiv.childNodes.length;

    for(var ix = 0; ix < numRules; ++ix) {
        var childDiv = rulesDiv.childNodes[ix];
        var realIndex = childDiv.id.substr(4);

        var left = $('#ruleleftside' + realIndex).val();
        var right = $('#rulerightside' + realIndex).val();

        lsystem.set_rule(left, right);
    }

    // Retrieve interpretations
    var interpDiv = document.getElementById('interp-div');
    var numInterp = interpDiv.childNodes.length;

    for(var ix = 0; ix < numInterp; ++ix) {
        var childDiv = interpDiv.childNodes[ix];
        var realIndex = childDiv.id.substr(6);

        var left = $('#interpleftside' + realIndex).val();
        var right = $('#interprightside' + realIndex).val();

        lsystem.set_interpretation(left, right);
    }

    lsystem.iterate();

    refreshDrawingParameters();
}

extractLSystemConfig();

/**
 * Draw a list of line segments.
 *
 * @param lines An array of float values. three values make up a vertex, and two vertices make up a line.
 * @param color The line color, white per default.
 */
function drawLines(lines) {
    var i = 0;

    var geometry = new three.Geometry();

    var clrs = [];

    while(i < lines.length)
    {
        const clrIdx = lines[i];
        i++;
        const color = colorPalette[clrIdx];

        var beginVertex = new three.Vector3(
            lines[i],
            -lines[i+1],
            lines[i+2]
        );

        i = i + 3;

        var endVertex = new three.Vector3(
            lines[i],
            -lines[i+1],
            lines[i+2]
        );

        i = i + 3;

        clrs.push(color);
        clrs.push(color);

        geometry.vertices.push(beginVertex, endVertex);
    }

    for(var i=0; i<geometry.vertices.length; i++) {
        geometry.colors[i] = clrs[i];
    }

    var material = new three.LineBasicMaterial({
        color: 0xffffff,
        vertexColors: three.VertexColors
    });

    var line = new three.LineSegments(geometry, material);
    scene.add(line);
}

$(function () {
    $('select').selectpicker();
});

function animate()
{
    requestAnimationFrame(animate);
    controls.update();
    renderer.render(scene, camera);
}

animate();

