use mysql::*;
use mysql::prelude::*;
use eframe::{egui, epi};
use crossclip::{Clipboard, SystemClipboard};
mod font;


pub struct password{
    id: i32,
    description: String,
    user: String,
    password: String,
}
//遗留下一个问题，如何在password上实现From_Row接口
pub struct MyApp {
    pub data: Vec<password>,
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
        font::install_fonts(_ctx);
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
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("some_unique_id").show(ui, |ui| {
                ui.label("description");
                ui.label("user");
                ui.label("password");
                ui.end_row();
                
                let mut vec_iter = self.data.iter();
                loop {
                    match vec_iter.next(){
                        Some(password) => {
                            ui.label(&password.description);
                            ui.label(&password.user);
                            if ui.add(egui::Button::new("copy password").frame(false)).clicked() {
                                let pass_sec = (&password.password).clone();
                                let clipboard =  SystemClipboard::new().unwrap();
                                clipboard.set_string_contents(pass_sec).unwrap();
                            };
                            ui.end_row();
                        },
                        None => break,
                    }
                }
            });

        });
    }
}