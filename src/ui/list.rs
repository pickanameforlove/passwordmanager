use crate::ui::p;
use crossclip::{Clipboard, SystemClipboard};
use eframe::egui;
use eframe::egui::{containers::*, *};

pub struct Password {
    pub id: i32,
    pub description: String,
    pub user: String,
    pub password: String,
}
//如何实现一个空构造方法呀！
impl Default for Password {
    fn default() -> Password {
        Password {
            id: 0,
            description: String::new(),
            user: String::new(),
            password: String::new(),
        }
    }
}

#[derive(Default)]
pub struct list;
// impl Default for list{
//     fn default() -> list{
//         list{open : true}
//     }
// }
impl list {
    pub fn show(&mut self, ctx: &CtxRef, data: &Vec<Password>, id: Id,vis:&mut bool) {
        // let mut memory = ctx.memory();
        // let open = memory.id_data_temp.get_mut_or_default::<bool>(id);
        // // let mut opening = open;
        // println!("{}",open);

        Window::new("password list")
            .open(vis)
            .show(ctx, |ui| self.ui(ui, data));
    }
    fn ui(&mut self, ui: &mut Ui, data: &Vec<Password>) {
        egui::containers::ScrollArea::auto_sized().show(ui, |ui| {
            Frame::dark_canvas(ui.style()).show(ui, |ui| {
                egui::Grid::new("list").min_col_width(180.0).show(ui, |ui| {
                    ui.label("description");
                    ui.label("user");
                    ui.label("password");
                    ui.label("option");

                    ui.end_row();

                    let mut vec_iter = (*data).iter();
                    //和if let一脉相承
                    let mut index = 1;
                    while let Some(password) = vec_iter.next() {
                        ui.label(&password.description);
                        ui.label(&password.user);
                        let mut p_clone = password.password.clone();
                        let id_str = index.to_string();
                        ui.add(p::password(&mut p_clone, id_str));
                        if ui.button("copy password").clicked() {
                            let c = password.password.clone();
                            let clipboard = SystemClipboard::new()
                                .expect("initialize the system clipboard Error");
                            match clipboard.set_string_contents(c) {
                                Ok(_) => println!("success in set clipboard content"),
                                Err(error) => {
                                    panic!("error in setting clipboard content {}", error)
                                }
                            };
                        }
                        ui.end_row();
                        index += 1;
                    }
                });
            })
        });
    }
}
