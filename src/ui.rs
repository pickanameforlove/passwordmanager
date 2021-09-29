use std::borrow::BorrowMut;

use mysql::*;
use mysql::prelude::*;
use eframe::{egui, epi};
use eframe::egui::Id;

mod font;
use font::install_fonts;
mod p;
use crate::ui::list::Password;
use eframe::egui::{containers::*, *};
pub mod dialog;
pub mod list;



//遗留下一个问题，如何在password上实现From_Row接口
pub struct MyApp {
    pub data: Vec<Password>,
    pub w1:list::list,
    pub w2:dialog::Dialog,
    pub w1_id:Id,
    pub w2_id: Id,
    pub w1_vis: bool,
    pub w2_vis: bool,
}

//这里不需要实现new方法，data需要在setup方法中获取到，进而更新。

pub fn getData()->Vec<Password>{
    let url = "mysql://root:123456@localhost:3306/passworddb";
        let opts = Opts::from_url(url).expect("mysql initialize Error!");

        let pool = Pool::new(opts).unwrap();

        let mut conn = pool.get_conn().unwrap();
        conn.query_map("select * from passwords",|(id,description,user,password)|Password{
            id,description,user,password
        }).expect("Query Failed")
}



impl epi::App for MyApp {
    fn name(&self) -> &str {
        "password manager"
    }
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        install_fonts(_ctx);
        
        self.data = getData();
        _ctx.memory().id_data_temp.insert(self.w1_id,true);
        _ctx.memory().id_data_temp.insert(self.w2_id,true);
        
        
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.checkbox(&mut self.w1_vis, "password list".to_string());
            ui.checkbox(&mut self.w2_vis, "add password".to_string());
         });
        self.w1.show(ctx,&self.data,self.w1_id,&mut self.w1_vis);
        self.w2.show(ctx,&mut self.data,&mut self.w2_vis);
    }
}