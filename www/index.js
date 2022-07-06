const input = document.getElementById("input_holder");
const id = document.getElementById("id_holder");

async function pushUrl() {
    console.log("pushing...");
    const url = input.value.trim();
    if(url) {
        const data = {
            url: url,
        }
        const resp = await postUserForm(window.location.href, data);
        if (resp.ok) {
            const json = await resp.json();
            id.innerHTML = json.id;
            id.title = json.id;
            id.href = json.id;
        } else {
            id.innerHTML = "Oops fatal error hehe";
        }
    }
}

async function postUserForm(url, data) {
    const resp = await fetch(url, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(data)
    });
    return resp;
}

function keyPressed(event) {
    if (event.keyCode == 13) {
        pushUrl();
    }
}
