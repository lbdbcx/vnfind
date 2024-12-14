var current_sort = { rev: false };
const INIT_GAME = {
    property: {
        "标题": "",
        "剧情": "",
        "结束时间": "",
        "画面": "",
        "角色": "",
        "感情": "",
        "玩法": "",
        "日常": "",
        "色情": "",
        "声音": "",
    },
};

function is_num(x) {
    return /^-?\d+(.\d+)?$/.test(x);
}

function get_sort_game(data) {
    return $.ajax({
        url: "/sort",
        data,
        callback: function (_, statusText) {
            if (statusText == "success") {
                current_sort = data;
            }
        }
    });
}

function get_game(id) {
    return $.get(`/get_game?id=${id}`);
}

async function change_game(id) {
    let game = await get_game(id);
    show_form(game);
}

async function show_game(data) {
    data = await get_sort_game(data);

    let table = $("<table></table>");
    let head = $("<tr></tr>");
    for (x of data.column) {
        let y = $("<th></th>").text(x);
        if (current_sort.key == x) {
            y.addClass(current_sort.rev ? "sortup" : "sortdown");
        }
        y.on({
            click: function () {
                let key = $(this).text();
                let d = current_sort;
                if (d.key == key) {
                    d.rev = !d.rev;
                } else {
                    d.key = key;
                    d.rev = false;
                }
                show_game(d);
            }
        });
        head.append(y);
    }
    table.append(head);
    let s;
    for (row of data.row) {
        let s = $("<tr></tr>");
        let id = null;
        for (i in row) {
            let x = row[i];
            if (id == null) {
                id = x;
            }
            let td = $(`<td>${x}</td>`);
            if (data.column[i] == "标题") {
                td.addClass("left-align");
            }
            s.append(td);
        }
        s.dblclick(function () {
            change_game(id);
        });
        table.append(s);
    }

    $("#table_container").html(table);
}

function remove_value() {
    $(this).parent().remove();
}

function add_value(form, k = "", v = "") {
    let new_value = `<input type="text" class="form_k" value="${k}"/><input type="text" class="form_v" value="${v}" />`;
    let del_button = $(`<button type="button" class="in-form">X</button>`).click(remove_value);
    $("#new_value_button").before($("<div></div>").append(new_value).append(del_button).append("<br />"));
}

function send_add_game(data) {
    $.post("/add_game", JSON.stringify(data), function () {
        show_game(current_sort);
        $("#add_form").remove();
    })
}
function send_modify_game(data) {
    console.log(data);
    $.post(`/edit_game?id=${data.id}`, JSON.stringify(data), function () {
        show_game(current_sort);
        $("#add_form").remove();
    })
}
function click_add_game(id) {
    let c = id > 0;
    let keys = $("#add_form .form_k");
    let values = $("#add_form .form_v");
    let tag = [], prop = {}, num_prop = {};
    for (let i = 0; i < keys.length; ++i) {
        let k = $(keys[i]).val().trim(), v = $(values[i]).val().trim();
        v = is_num(v) ? parseFloat(v) : v;
        if (v == "") {
            continue;
        }
        if (k == "tag" || k == "") {
            tag = tag.concat(v.split(/,|，/).map((x) => x.trim()));
        }
        else if (is_num(v)) {
            num_prop[k] = v;
        } else {
            prop[k] = v;
        }
    }
    let game_data = { tag, property: prop, num_property: num_prop, id: c ? id : 0 };
    c ? send_modify_game(game_data) : send_add_game(game_data);
}

function show_form(data) {
    let is_modify = data.id > 0;
    let form = $(`<form id="add_form"></form>`);
    let cancel_button = $(`<button type="button">取消</button>`).click(function () {
        $("#add_form").remove();
    });
    let submit_button = $(`<button type="button" class="primary">提交</button>`).click(function () {
        click_add_game(data.id);
    });
    let new_button = $(`<button type="button" id="new_value_button">+</button>`).click(function () {
        add_value(form);
    });
    form.append(new_button);
    form.append(cancel_button);
    form.append(submit_button);
    $("#form_container").html(form);
    for (k in data.property) {
        add_value(form, k, data.property[k]);
    }
    for (k in data.num_property) {
        add_value(form, k, data.num_property[k]);
    }
    if (data.tag) { add_value(form, "tag", data.tag.join(', ')); }
    if (is_modify) {
        $("#submit_form").text("修改");
    }
}

window.onload = async function () {
    show_game();
    $("#add_btn").click(function () {
        show_form(INIT_GAME);
    });
}