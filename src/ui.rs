use mysql::*;
use mysql::prelude::*;
use eframe::{egui, epi};
use crossclip::{Clipboard, SystemClipboard};

mod font;
use font::install_fonts;
use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
mod p;
use list::password;

pub mod dialog;
pub mod list;



//遗留下一个问题，如何在password上实现From_Row接口
pub struct MyApp {
    pub data: Vec<password>,
    pub w1:list::list,
    pub w2:dialog::dialog,
}

//这里不需要实现new方法，data需要在setup方法中获取到，进而更新。

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
        let url = "mysql://root:123456@localhost:3306/passworddb";
        let opts = Opts::from_url(url).expect("mysql initialize Error!");

        let pool = Pool::new(opts).unwrap();

        let mut conn = pool.get_conn().unwrap();
        let result = conn.query_map("select * from passwords",|(id,description,user,password)|password{
            id,description,user,password
        }).expect("Query Failed");
        self.data = result;

        
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        self.w1.show(ctx,&self.data);
        self.w2.show(ctx);
    }
}