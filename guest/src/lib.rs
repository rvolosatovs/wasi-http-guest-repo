wit_bindgen::generate!("guest");

struct Guest;

impl http::Http for Guest {
    fn handle(_req: types::IncomingRequest, _res: types::ResponseOutparam) {
        todo!()
    }
}

export_guest!(Guest);
