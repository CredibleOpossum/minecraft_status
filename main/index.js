function handle_server_json(json) {
    // Reset to inital state
    icon.src = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII="
    players.style = "";
    players.innerText = "[N/A]";
    list.innerHTML = "";

    // Update text and style
    icon.src = json.favicon
    if (json.players.online < json.players.max) {
        players.style = "color:rgb(0, 255, 0)";
    } else {
        players.style = "color:rgb(255, 0, 0)";
    }
    players.innerText = `[${json.players.online}/${json.players.max}]`

    if (json["players"].hasOwnProperty("sample")) {
        json["players"]["sample"].forEach(element => {
            var li = document.createElement("li");
            li.appendChild(document.createTextNode(element.name))
            list.appendChild(li);
        });
    }
}

async function get_server_data(hostname, port) {
    await fetch(`get/${hostname}/${port}`).then(response => response.json()).then(data => handle_server_json(data)).catch((error) => { })
}

document.getElementById("address_input").addEventListener("keyup", function (event) {
    if (event.key === "Enter") {
        input = document.getElementById("address_input").value.split(":");
        if (input.length == 2) {
            get_server_data(input[0], input[1]);
        } else if (input.length == 1) {
            get_server_data(input[0], "25565");
        }
    }
});
