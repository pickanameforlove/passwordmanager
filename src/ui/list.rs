use eframe::egui::{containers::*, *};
use eframe::{egui, epi};
use crate::ui::dialog;
use crate::ui::p;


pub struct password{
    pub id: i32,
    pub description: String,
    pub user: String,
    pub password: String,
}
//如何实现一个空构造方法呀！
impl Default for password {
    fn default () -> password {
        password{id: 0, description: String::new(), user:String::new(),password:String::new()}
    }
}

// #[derive(Default)]
pub struct list{
    open: bool,
}
impl Default for list{
    fn default() -> list{
        list{open : true}
    }
}
impl list {
    pub fn show(&mut self, ctx: &CtxRef , data : &Vec<password>){
        // let mut open = true;
        let mut open = self.open;
        Window::new("password list").open(&mut open).show(ctx,|ui| self.ui(ui,data));
    }
    fn ui(&mut self, ui: &mut Ui,data : &Vec<password>) {
        Frame::dark_canvas(ui.style()).show(ui,|ui|{
            egui::Grid::new("list").min_col_width(180.0).show(ui, |ui| {
                ui.label("description");
                ui.label("user");
                ui.label("password");

                ui.end_row();
                
                let mut vec_iter = (*data).iter();
                //和if let一脉相承
                let mut index = 1;
                while let Some(password) = vec_iter.next(){
                    ui.label(&password.description);
                    ui.label(&password.user);
                    let mut p_clone = password.password.clone();
                    let id_str = index.to_string();
                    ui.add(p::password(&mut p_clone,id_str));
                    // if ui.button("test").clicked(){
                    //     let mut dialog = dialog::dialog::default();
                    //     dialog.show(ctx);
                    // }
                    ui.end_row();
                    index += 1;
                }
            });
        });
    }
}