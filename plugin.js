const socket = io("https://clearlang.org/");

register_plugin = function (importObject) {    
    importObject.env._register_name = function (js_object) {
        socket.emit('player_name', consume_js_object(js_object));
    }

    importObject.env._register_time = function (name, time) {
        socket.emit('player_time', [consume_js_object(name), time]);
    }
}

socket.on('update_player', (player) => {
    wasm_exports._update_player(js_object(player[0]), player[1]);
});

miniquad_add_plugin({register_plugin});
