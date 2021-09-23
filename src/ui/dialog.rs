use eframe::egui::{containers::*, *};
use crate::ui::list::Password;
use crate::add_password::insert_password;
use crate::add_password::gen_traditional_password;
// #[derive(Default)]
pub struct Dialog{
    p:Password,
    open:bool,
}
impl Default for Dialog {
    fn default () -> Dialog {
        Dialog{p : Password{id: 0, description: String::new(), user:String::new(),password:String::new()},
                open : true}
    }
}
impl Dialog {
    pub fn show(&mut self, ctx: &CtxRef){
        let mut open = self.open;
        // Window::new("adding password").open(&mut open).show(ctx,|ui| self.ui(ui));
        Window::new("adding password").open(&mut open).show(ctx,|ui| self.ui(ui));
    }
    fn ui(&mut self, ui: &mut Ui) {
        let Self{p,open} = self;
        Frame::dark_canvas(ui.style()).show(ui,|ui|{
            ui.horizontal(|ui| {
                ui.label("网站描述：");
                ui.text_edit_singleline(&mut p.description);
            });
            ui.horizontal(|ui| {
                ui.label("网站用户：");
                ui.text_edit_singleline(&mut p.user);
            });
            ui.horizontal(|ui| {
                ui.label("网站密码：");
                ui.text_edit_singleline(&mut p.password);
                if ui.button("generate").clicked(){
                    p.password = gen_traditional_password(18);
                }
            });
            if ui.button("submit").clicked(){
                // println!("d:{},u:{},p:{}",p.description,p.user,p.password);
                insert_password(p);
                *open = false;
            }
        });
    }
}
