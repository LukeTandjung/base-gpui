use crate::dialog::{
    create_dialog_handle, DialogClose, DialogDescription, DialogHandle, DialogTitle, DialogTrigger,
};
use crate::drawer::{
    create_drawer_handle, DrawerClose, DrawerDescription, DrawerHandle, DrawerTitle, DrawerTrigger,
};

#[test]
fn reexported_parts_are_the_dialog_types() {
    // Compile-level alias check: each Drawer re-export is the Dialog type.
    let _trigger: fn() -> DialogTrigger<()> = DrawerTrigger::<()>::new;
    let _title: fn() -> DialogTitle<()> = DrawerTitle::<()>::new;
    let _description: fn() -> DialogDescription<()> = DrawerDescription::<()>::new;
    let _close: fn() -> DialogClose<()> = DrawerClose::<()>::new;

    // A Drawer intentionally shares the Dialog handle type.
    let drawer_handle: DrawerHandle<()> = create_drawer_handle::<()>();
    let dialog_handle: DialogHandle<()> = drawer_handle;
    let _also_dialog: DialogHandle<()> = create_dialog_handle::<()>();
    drop(dialog_handle);
}
