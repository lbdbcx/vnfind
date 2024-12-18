var current_search = { rev: false, columns: "类型|剧情|角色|感情|画面|结束时间" };
const INIT_GAME = {
    property: {
        "标题": "",
        "结束时间": "",
        "类型": "",
        "剧情": "",
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

function send_search_game(data) {
    return $.ajax({
        url: "/search",
        data,
        callback: function (_, statusText) {
            if (statusText == "success") {
                current_search = data;
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

async function show_game_detail(id) {
    let game = await get_game(id);
    let main = $("<div></div>");
    let title_container = $(`<div class="flex-row" id="game-title-container"></div>`);
    let return_button = $("<button>←</button>").click(function () {
        $("#right-container").empty().addClass("hidden");
    });
    let edit_button = $("<button>编辑</button>").click(function () {
        change_game(id);
    });
    title_container.append(return_button).append(`<h1>${game.property["标题"]}</h1> </div>`).append(edit_button);
    main.append(title_container);
    for (x in game.property) {
        if (x == "标题") continue;
        main.append(`${x} : ${game.property[x]}<br />`);
    }
    let tags = $(`<div id="tag-container"></div>`);
    for (x of game.tag) {
        tags.append($(`<span class="tag">${x}</span>`));
    }
    main.append(tags);
    $("#right-container").html(main).removeClass("hidden");
}

async function show_game_table(data) {
    data = await send_search_game(data);

    let table = $("<table></table>");
    let head = $("<tr></tr>");
    for (x of data.column) {
        let y = $("<th></th>").text(x);
        if (current_search.key == x) {
            y.addClass(current_search.rev ? "sortup" : "sortdown");
        }
        y.on({
            click: function () {
                let key = $(this).text();
                let d = current_search;
                if (d.key == key) {
                    d.rev = !d.rev;
                } else {
                    d.key = key;
                    d.rev = false;
                }
                show_game_table(d);
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
        s.on({
            click: function () {
                show_game_detail(id);
            },
            dblclick: function () {
                change_game(id);
            },
        });
        table.append(s);
    }

    $("#table-container").html(table);
}

function remove_value() {
    $(this).parent().remove();
}

function add_value(form, k = "", v = "") {
    let new_value = `<input type="text" class="form-k" value="${k}"/><input type="text" class="form-v" value="${v}" />`;
    let del_button = $(`<button type="button" class="in-form">X</button>`).click(remove_value);
    $("#new-value-button").before($("<div></div>").append(new_value).append(del_button).append("<br />"));
}

function send_add_game(data) {
    $.post("/add_game", JSON.stringify(data), function (res) {
        show_game_table(current_search);
        show_game_detail(res);
    })
}
function send_modify_game(data) {
    $.post(`/edit_game?id=${data.id}`, JSON.stringify(data), function () {
        show_game_table(current_search);
        show_game_detail(data.id);
    })
}
function click_add_game(id) {
    let c = id > 0;
    let keys = $("#add-form .form-k");
    let values = $("#add-form .form-v");
    let tag = [], prop = {};
    for (let i = 0; i < keys.length; ++i) {
        let k = $(keys[i]).val().trim(), v = $(values[i]).val().trim();
        // v = is_num(v) ? parseFloat(v) : v;
        if (v == "") {
            continue;
        }
        if (k == "tag" || k == "") {
            tag = tag.concat(v.split(/,|，/).map((x) => x.trim()));
        }
        else {
            prop[k] = v;
        }
    }
    let game_data = { tag, property: prop, id: c ? id : 0 };
    c ? send_modify_game(game_data) : send_add_game(game_data);
}

function show_form(data) {
    let is_modify = data.id > 0;
    let form = $(`<form id="add-form"></form>`);
    let cancel_button = $(`<button type="button">取消</button>`);
    if (is_modify) {
        cancel_button.click(function () {
            $("#add-form").remove();
            show_game_detail(data.id);
        });
    } else {
        cancel_button.click(function () {
            $("#right-container").empty().addClass("hidden");
        });
    }
    let submit_button = $(`<button type="button" class="primary">提交</button>`).click(function () {
        click_add_game(data.id);
    });
    let new_button = $(`<button type="button" id="new-value-button">+</button>`).click(function () {
        add_value(form);
    });
    form.append(new_button);
    form.append(cancel_button);
    form.append(submit_button);
    $("#right-container").html(form).removeClass("hidden");
    for (k in data.property) {
        add_value(form, k, data.property[k]);
    }
    // for (k in data.num_property) {
    //     add_value(form, k, data.num_property[k]);
    // }
    if (data.tag) { add_value(form, "tag", data.tag.join(', ')); }
    if (is_modify) {
        $("#submit-form").text("修改");
    }
}

window.onload = async function () {
    show_game_table(current_search);
    $("#add-btn").click(function () {
        show_form(INIT_GAME);
    });
    $("#columns-input").val(current_search.columns).on('keypress', function (e) {
        if (e.which === 13) {
            let d = current_search;
            d.columns = $(this).val();
            show_game_table(d);
        }
    });
    $("#search-input").on({
        keypress: function (e) {
            if (e.which === 13) {
                let d = current_search;
                d.query = $(this).val();
                show_game_table(d);
            }
        },
        input: function () {
            console.log($(this).val());
            let d = current_search;
            d.query = $(this).val();
            show_game_table(d);
        }
    });
}