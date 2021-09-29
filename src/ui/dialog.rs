use eframe::egui::{containers::*, *};
use crate::ui::list::Password;
use crate::add_password::insert_password;
use crate::add_password::gen_traditional_password;
use crate::ui::getData;

use super::MyApp;
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
    pub fn show(&mut self, ctx: &CtxRef,data: &mut Vec<Password>,vis:&mut bool){
        
        // let  open =ctx.memory().id_data_temp.get_mut_or_default::<bool>(id);
        // let mut opening = open;
        Window::new("adding password").open(vis)
        // .open(ctx.memory().id_data_temp.get_mut_or_default::<bool>(id))
        .show(ctx,|ui| self.ui(ui,data));
    }
    fn ui(&mut self, ui: &mut Ui, data: &mut Vec<Password>) {
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
                p.description = String::new();
                p.user = String::new();
                p.password = String::new();
                // *open = false;
                *data = getData();
                
                
            }
        });
    }
}
