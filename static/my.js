function addEditEvent() {
    let edit = document.getElementById("modal_load");

    edit.addEventListener('click', function (e) {
            if (e.target.id === "modal_load") {
                edit.classList.remove('in');
                edit.setAttribute("aria-hidden", "true");
                setTimeout(() => {
                    edit.setAttribute("style", "display: none")
                }, 150)
                deleteBackDrop();
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
    }, 150);
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

function login_btn_disable() {
    let btn = document.getElementById("login_btn");
    btn.disabled = true;
    btn.innerText = "登录中";
}

function login_btn_enable() {
    let btn = document.getElementById("login_btn");
    btn.disabled = false;
    btn.innerText = "登录";
}

function register_btn_disable() {
    let btn = document.getElementById("register_btn");
    btn.disabled = true;
    btn.innerText = "注册中";
}

function register_btn_enable() {
    let btn = document.getElementById("register_btn");
    btn.disabled = false;
    btn.innerText = "注册";
}

function active_btn_disable() {
    let btn = document.getElementById("active_btn");
    btn.disabled = true;
    btn.innerText = "激活中";
}

function active_btn_enable() {
    let btn = document.getElementById("active_btn");
    btn.disabled = false;
    btn.innerText = "激活";
}

function send_btn_disable() {
    let btn = document.getElementById("send_btn");
    btn.disabled = true;
    btn.innerText = "发送中";
}

function send_btn_enable() {
    let btn = document.getElementById("send_btn");
    btn.disabled = false;
    btn.innerText = "发送";
}

function change_btn_disable() {
    let btn = document.getElementById("change_btn");
    btn.disabled = true;
    btn.innerText = "修改中";
}

function change_btn_enable() {
    let btn = document.getElementById("change_btn");
    btn.disabled = false;
    btn.innerText = "修改";
}
