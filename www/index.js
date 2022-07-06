const input = document.getElementById("input_holder");
const id = document.getElementById("id_holder");

async function pushUrl() {
    const url = input.value.trim();
    if(url) {
        const data = {
            url: url,
        }
    }
}

function keyPressed(event) {
    if (event.keyCode == 13) {
        pushUrl();
    }
}
