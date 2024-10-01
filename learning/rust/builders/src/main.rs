use derive_builder::Builder;

#[derive(Clone, Debug)]
struct TLSCert {
    key: String,
    cert: String,
}

type Ms = u32;

#[derive(Debug, Builder)]
struct Server {
    host: String,
    port: u16,
    tls: Option<TLSCert>,
    hot_reload: bool,
    timeout: Ms,
}

// impl Server {
//     fn build(host: String, port: u16) -> Builder {
//         Builder {
//             host,
//             port,
//             tls: None,
//             hot_reload: None,
//             timeout: None,
//         }
//     }
// }

pub struct Builder {
    host: String,
    port: u16,
    tls: Option<TLSCert>,
    hot_reload: Option<bool>,
    timeout: Option<Ms>,
}

impl Builder {
    fn tls(mut self, tls: TLSCert) -> Self {
        self.tls = Some(tls);
        self
    }
    fn hot_reload(mut self, hot_reload: bool) -> Self {
        self.hot_reload = Some(hot_reload);
        self
    }
    fn timeout(mut self, timeout: ms) -> Self {
        self.timeout = Some(timeout);
        self
    }
    fn build(self) -> Server {
        Server {
            host: self.host,
            port: self.port,
            tls: self.tls,
            hot_reload: self.hot_reload.unwrap_or_default(),
            timeout: self.timeout.unwrap_or(2000),
        }
    }
}

fn main() {
    let host = "localhost".to_owned();
    let port = 8080;

    let cert = TLSCert {
        key: "...".to_owned(),
        cert: "...".to_owned(),
    };

    // let basic_server = Server::build(host.clone(), port).build();
    // println!("{basic_server:?}");
    // 
    // let tls_server = Server::build(host.clone(), port).tls(cert.clone()).build();
    // println!("{tls_server:?}");
    // 
    // let server = Server::build(host.clone(), port)
    //     .tls(cert.clone())
    //     .hot_reload(true)
    //     .timeout(5000)
    //     .build();
    // println!("{server:?}");

    let server = ServerBuilder::default()
        .host(host.clone())
        .port(port.clone())
        .build()
        .unwrap();
}
