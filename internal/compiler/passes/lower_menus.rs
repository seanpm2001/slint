// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0

//! Passe lower the `MenuBar` and `ContextMenu` as well as all their contents
//!
//! Must be done before inlining and many other passes because the lowered code must
//! be further inlined as it may expends to native widget that needs inlining

use crate::diagnostics::BuildDiagnostics;
use crate::expression_tree::NamedReference;
use crate::langtype::ElementType;
use crate::object_tree::*;

struct UsefulMenuComponents {
    menubar_impl: ElementType,
    vertical_layout: ElementType,
    empty: ElementType,
}

pub async fn lower_menus(
    doc: &Document,
    type_loader: &mut crate::typeloader::TypeLoader,
    diag: &mut BuildDiagnostics,
) {
    // Ignore import errors
    let mut build_diags_to_ignore = BuildDiagnostics::default();
    let useful_menu_component = UsefulMenuComponents {
        menubar_impl: type_loader
            .import_component("std-widgets.slint", "MenuBarImpl", &mut build_diags_to_ignore)
            .await
            .expect("MenuBarImpl should be in std-widgets.slint")
            .into(),
        vertical_layout: type_loader
            .global_type_registry
            .borrow()
            .lookup_builtin_element("VerticalLayout")
            .expect("VerticalLayout is a builtin type"),
        empty: type_loader.global_type_registry.borrow().empty_type(),
    };

    doc.visit_all_used_components(|component| {
        recurse_elem_including_sub_components_no_borrow(component, &(), &mut |elem, _| {
            if matches!(&elem.borrow().builtin_type(), Some(b) if b.name == "Window") {
                process_window(elem, &useful_menu_component, diag);
            }
        })
    });
}

fn process_window(win: &ElementRc, components: &UsefulMenuComponents, diag: &mut BuildDiagnostics) {
    /*  if matches!(&elem.borrow_mut().base_type, ElementType::Builtin(_)) {
        // That's the TabWidget re-exported from the style, it doesn't need to be processed
        return;
    }*/

    let mut window = win.borrow_mut();
    let mut menu_bar = None;
    window.children.retain(|x| {
        if matches!(&x.borrow().base_type, ElementType::Builtin(b) if b.name == "MenuBar") {
            if menu_bar.is_some() {
                diag.push_error("Only one MenuBar is allowed in a Window".into(), &*x.borrow());
            } else {
                menu_bar = Some(x.clone());
            }
            false
        } else {
            true
        }
    });

    let Some(menu_bar) = menu_bar else {
        return;
    };
    menu_bar.borrow_mut().base_type = components.menubar_impl.clone();

    // Create a child that contains all the child but the menubar
    let child = Element {
        id: window.id.clone() + "-child",
        base_type: components.empty.clone(),
        enclosing_component: window.enclosing_component.clone(),
        children: std::mem::take(&mut window.children),
        ..Default::default()
    }
    .make_rc();

    const HEIGHT: &str = "height";
    let child_height = NamedReference::new(&child, HEIGHT);

    // Create a layout
    let layout = Element {
        id: window.id.clone() + "-menulayout",
        base_type: components.vertical_layout.clone(),
        enclosing_component: window.enclosing_component.clone(),
        children: vec![menu_bar, child],
        ..Default::default()
    }
    .make_rc();

    window.children.push(layout);
    let component = window.enclosing_component.upgrade().unwrap();

    drop(window);

    // Rename every access to `root.height` into `child.height`
    let win_height = NamedReference::new(win, HEIGHT);
    crate::object_tree::visit_all_named_references(&component, &mut |nr| {
        if nr == &win_height {
            *nr = child_height.clone()
        }
    });
    // except for the actual geometry
    win.borrow_mut().geometry_props.as_mut().unwrap().height = win_height;
}
