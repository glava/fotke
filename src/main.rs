extern crate gdk_pixbuf;
extern crate gio;
extern crate glib;
extern crate gtk;

use gdk_pixbuf::{Pixbuf, InterpType};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    ApplicationWindow, CellRendererText, Orientation, TreeStore, TreeView, TreeViewColumn, WindowPosition, Image,
};

use std::env::args;

macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
                move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
                move |$(clone!(@param $p),)+| $body
        }
    );
}

#[macro_export]
macro_rules! upgrade_weak {
    ($x:ident, $r:expr) => {{
        match $x.upgrade() {
            Some(o) => o,
            None => return $r,
        }
    }};
    ($x:ident) => {
        upgrade_weak!($x, ())
    };
}

fn append_text_column(tree: &TreeView) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}

fn load_image(path: &str) -> Option<Pixbuf> {
    return Pixbuf::new_from_file(path)
    .ok()
    .map(|i| i.scale_simple(640, 480, InterpType::Bilinear))
    .unwrap();
}

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("Fotke");
    window.set_position(WindowPosition::Center);

    // left pane
    let left_tree = TreeView::new();
    let left_store = TreeStore::new(&[String::static_type()]);

    left_tree.set_model(Some(&left_store));
    left_tree.set_headers_visible(false);
    append_text_column(&left_tree);

    let images = fotke::image_paths("/Users/goran/Documents/xa2");
    for image in images.clone() {
        // insert_with_values takes two slices: column indices and ToValue
        // trait objects. ToValue is implemented for strings, numeric types,
        // bool and Object descendants
        left_store.insert_with_values(None, None, &[0], &[&image.as_path().to_str().unwrap()]);
    }


    let image = Image::new_from_pixbuf(Some(&load_image("/Users/goran/Documents/xa2/24A_01150.jpg").unwrap()));

    let left_selection = left_tree.get_selection();
    let split_pane = gtk::Box::new(Orientation::Horizontal, 10);

    split_pane.set_size_request(-1, -1);
    split_pane.add(&left_tree);
    split_pane.add(&image);
    window.add(&split_pane);
    window.show_all();

    left_selection.connect_changed(move |tree_selection| {
        let (left_model, iter) = tree_selection.get_selected().expect("Couldn't get selected");
        
        let selected_path = left_model.get_value(&iter, 0)
                        .get::<String>()
                        .expect("Couldn't get string value");
        let widgets = split_pane.get_children();
        let selected_image = widgets[1].downcast_ref::<gtk::Image>().unwrap();
        
        let image_pix = load_image(&selected_path).unwrap();
        selected_image.set_from_pixbuf(Some(&image_pix));
        selected_image.show_all();
    });
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.treeview"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
