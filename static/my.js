function addEditEvent() {
    let edit = document.getElementById("modal_load");

    edit.addEventListener('click', function (e) {
            console.log(e)
            if (e.target.id === "modal_load") {
                edit.classList.remove('in');
                edit.setAttribute("aria-hidden", "true");
                setTimeout(() => {
                    edit.setAttribute("style", "display: none")
                }, 150)
                deleteBackDrop()
            }
        }
    )
}

function unShow() {
    let edit = document.getElementById("modal_load");
    edit.click();
}

function show() {
    let edit = document.getElementById("modal_load");
    addBackDrop();
    edit.setAttribute("style", "display: block")
    edit.setAttribute("aria-hidden", "false");
    setTimeout(() => {
        edit.classList.add('in');
    }, 150)
}

function addBackDrop() {
    let backdrop = document.createElement("div");
    backdrop.setAttribute("id", "backdrop");
    backdrop.classList.add("modal-backdrop");
    backdrop.classList.add("fade");
    backdrop.classList.add("in");
    document.body.appendChild(backdrop);
}

function deleteBackDrop() {
    let backdrop = document.getElementById("backdrop");
    if (backdrop === null) return;
    document.body.removeChild(backdrop);
}