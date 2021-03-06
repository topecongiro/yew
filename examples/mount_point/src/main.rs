#[macro_use]
extern crate stdweb;
extern crate yew;
extern crate mount_point;

use yew::prelude::*;
use stdweb::web::{IElement, INode, IParentNode, document};
use mount_point::Model;

fn main() {
    yew::initialize();
    let body = document().query_selector("body").unwrap().unwrap();

    // This canvas won't be overwritten by yew!
    let canvas = document().create_element("canvas").unwrap();
    body.append_child(&canvas);

    js! {
        const canvas = document.querySelector("canvas");
        canvas.width = 100;
        canvas.height = 100;
        const ctx = canvas.getContext("2d");
        ctx.fillStyle = "green";
        ctx.fillRect(10, 10, 50, 50);
    };

    let mount_class = "mount-point";
    let mount_point = document().create_element("div").unwrap();
    mount_point.class_list().add(mount_class).unwrap();
    body.append_child(&mount_point);

    let app: App<_, Model> = App::new(());
    app.mount(mount_point);
    yew::run_loop();
}
