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
        for (x of row) {
            if (id == null) {
                id = x;
            }
            s.append("<td>" + x + "</td>");
        }
        s.dblclick(function () {
            change_game(id);
        });
        table.append(s);
    }

    $("#table_container").html(table);
}

function remove_value() {
    let node = $(this);
    console.log(node.prev());
    node.prev().remove();
    node.prev().remove();
    node.next().remove();
    node.remove();
}

function add_value(form, k = "", v = "") {
    let new_value = `<input type="text" class="form_k" value="${k}"/><input type="text" class="form_v" value="${v}" />`;
    let del_button = $(`<button type="button"></button>`).text("X");
    del_button.click(remove_value);
    form.children().last().prev().before(new_value).before(del_button).before($("<br />"));
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
function click_add_game(e, id) {
    let c = id > 0;
    let children = $(e.target).parent().children();
    let keys = children.filter((_, x) => { return $(x).hasClass("form_k"); });
    let values = children.filter((_, x) => { return $(x).hasClass("form_v"); });
    let tag = [], prop = {}, num_prop = {};
    for (let i = 0; i < keys.length; ++i) {
        let k = $(keys[i]).val().trim(), v = $(values[i]).val().trim();
        v = is_num(v) ? parseFloat(v) : v;
        if (v == "") {
            continue;
        }
        if (k == "tag" || k == "") {
            tag = v.split(/,|，/).map((x) => x.trim());
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
    let c = data.id > 0;
    let form = $(`<form id="add_form">
        <button type="button" id="submit_form">提交</button>
    </form>`);
    let new_button = $(`<button type="button"></button>`).text("+").click(function () {
        add_value(form);
    });
    form.children().last().before(new_button);
    for (k in data.property) {
        add_value(form, k, data.property[k]);
    }
    for (k in data.num_property) {
        add_value(form, k, data.num_property[k]);
    }
    if (data.tag) { add_value(form, "tag", data.tag.join(',')); }
    $("#form_container").html(form);
    if (c) {
        $("#submit_form").text("修改");
    }
    $("#submit_form").click(function (e) {
        click_add_game(e, data.id);
    });
}

window.onload = async function () {
    show_game();
    $("#add_btn").click(function () {
        show_form(INIT_GAME);
    });
}