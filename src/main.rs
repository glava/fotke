//! # TreeView Sample
//!
//! This sample demonstrates how to create a `TreeView` with either a `ListStore` or `TreeStore`.

extern crate gdk_pixbuf;
extern crate gio;
extern crate glib;
extern crate gtk;

use gdk_pixbuf::{Pixbuf, InterpType};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    ApplicationWindow, ButtonsType, CellRendererPixbuf, CellRendererText, DialogFlags,
    MessageDialog, MessageType, Orientation, TreeStore, TreeView, TreeViewColumn, WindowPosition,
};

use std::env::args;

// make moving clones into closures more convenient
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
    let first = images.first().unwrap().as_path().to_str().unwrap();
    let image = load_image(&first);
    for image in images.clone() {
        // insert_with_values takes two slices: column indices and ToValue
        // trait objects. ToValue is implemented for strings, numeric types,
        // bool and Object descendants
        left_store.insert_with_values(None, None, &[0], &[&image.as_path().to_str().unwrap()]);

        // for _ in 0..i {
        //     left_store.insert_with_values(Some(&iter), None, &[0], &[&"I'm a child node"]);
        // }
    }

    // right pane
    let right_tree = TreeView::new();
    let right_column_types = [Pixbuf::static_type()];
    let right_store = TreeStore::new(&right_column_types);
    let renderer = CellRendererPixbuf::new();
    let col = TreeViewColumn::new();

    col.set_title("Picture");
    col.pack_start(&renderer, false);

    col.add_attribute(&renderer, "pixbuf", 0);
    right_tree.append_column(&col);
    right_tree.set_model(Some(&right_store));
    right_tree.set_headers_visible(true);

    
    right_store.insert_with_values(
        None,
        None,
        &[0],
        &[&image],
    );

    // selection and path manipulation

    let left_selection = left_tree.get_selection();
    left_selection.connect_changed(clone!(right_tree => move |tree_selection| {
        let f = images.first();
        let (left_model, iter) = tree_selection.get_selected().expect("Couldn't get selected");
        
     let mut path = left_model.get_path(&iter).expect("Couldn't get path");
        println!("{:?}", left_model.get_value(&iter, 0)
                    .get::<String>()
                    .expect("Couldn't get string value"));
        // get the top-level element path
        while path.get_depth() > 1 {
            path.up();
        }
        right_tree.get_selection().select_path(&path);
    }));

    // display the panes

    let split_pane = gtk::Box::new(Orientation::Horizontal, 10);

    split_pane.set_size_request(-1, -1);
    split_pane.add(&left_tree);
    split_pane.add(&right_tree);

    window.add(&split_pane);
    window.show_all();
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