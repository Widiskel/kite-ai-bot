use rand::{rng, seq::IndexedRandom, Rng};
use serde_json::Value;
use std::{fs::File, io::Read, time::Duration};

pub struct Helper;

impl Helper {
    pub fn read_data_from_file(path: &str) -> Option<Vec<String>> {
        let mut file: File = File::open(path).ok()?;
        let mut data: String = String::new();
        file.read_to_string(&mut data).ok()?;
        let json_data: Value = serde_json::from_str(&data).ok()?;

        json_data
            .as_array()?
            .iter()
            .filter_map(|val| val.as_str().map(String::from))
            .collect::<Vec<String>>()
            .into()
    }

    pub fn random_user_agent() -> String {
        let ua = [
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/125.0.6422.80 Mobile/15E148 Safari/604.1",
      "Mozilla/5.0 (iPhone; CPU iPhone OS 17_5_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 EdgiOS/125.2535.60 Mobile/15E148 Safari/605.1.15",
      "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.6422.113 Mobile Safari/537.36 EdgA/124.0.2478.104",
      "Mozilla/5.0 (Linux; Android 10; Pixel 3 XL) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.6422.113 Mobile Safari/537.36 EdgA/124.0.2478.104",
      "Mozilla/5.0 (Linux; Android 10; VOG-L29) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.6422.113 Mobile Safari/537.36 OPR/76.2.4027.73374",
      "Mozilla/5.0 (Linux; Android 10; SM-N975F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.6422.113 Mobile Safari/537.36 OPR/76.2.4027.73374",
        ];

        let mut rng = rand::rng();
        ua.choose(&mut rng).unwrap().to_string()
    }

    pub fn pick_random_from_arr<T: Clone>(items: &[T]) -> Option<T> {
        let mut rng = rng();
        items.choose(&mut rng).cloned()
    }

    pub fn pick_random_set<'a>(items: &'a [(&'a str, &'a str)]) -> (&'a str, &'a str) {
        let mut rng = rand::rng();
        items[rng.random_range(0..items.len())]
    }

    pub fn ms_to_time(ms: u64) -> String {
        let duration = Duration::from_millis(ms);
        let hours = duration.as_secs() / 3600;
        let minutes = (duration.as_secs() % 3600) / 60;
        let seconds = duration.as_secs() % 60;
        format!("{} Hours {} Min {} Sec", hours, minutes, seconds)
    }

    pub fn get_data_index_from_file(acc: &str, path: &str) -> Option<i32> {
        if let Some(data) = Self::read_data_from_file(path) {
            data.iter().position(|x| x == acc).map(|index| index as i32)
        } else {
            None
        }
    }

    pub fn is_valid_proxy_format(proxy_url: &str) -> bool {
        let parts: Vec<&str> = proxy_url.split("://").collect();
        if parts.len() != 2 {
            return false;
        }

        let protocol = parts[0];
        if !["http", "https", "socks5"].contains(&protocol) {
            return false;
        }

        let rest = parts[1];
        let (auth_part, host_port_part) = match rest.split_once('@') {
            Some((auth, host_port)) => (Some(auth), host_port),
            None => (None, rest),
        };

        if let Some(auth) = auth_part {
            let auth_parts: Vec<&str> = auth.split(':').collect();
            if auth_parts.len() < 1 || auth_parts.len() > 2 {
                return false;
            }
            if auth_parts[0].is_empty() {
                return false;
            }
        }

        let host_port_parts: Vec<&str> = host_port_part.split(':').collect();
        if host_port_parts.len() != 2 {
            return false;
        }

        let host = host_port_parts[0];
        let port = host_port_parts[1];

        !host.is_empty() && port.parse::<u16>().is_ok()
    }

    pub fn show_skel_logo() -> &'static str {
        let logo: &'static str = "   
                      ...                                                     
                     .;:.                                 
                    .;ol,.                                
                   .;ooc:'                                
            ..    .;ooccc:'.    ..                        
          .',....'cdxlccccc;.....,'.                      
         .;;..'';clolccccccc:,''..;;.                     
        ':c'..':cccccccccccccc;...'c:.                    
       ':cc,.'ccccccccccccccccc:..;cc:'                   
    ...:cc;.':cccccccccccccccccc:..:cc:...                
   .;';cc;.':;;:cccccccccccccc:;;;'.;cc,,;.               
  .cc':c:.',.....;cccccccccc;.....,..:c:'c:               
  ,x:'cc;.,'     .':cccccc:'.     ',.;cc':x'              
  lO,'cc;.;,       .;cccc:.       ,;.;cc';0l              
 .o0;.;c;.,:'......',''''''......':,.;c;.:0l.             
 .lxl,.;,..;c::::;:,.    .,:;::::c;..,;.,oxl.             
 .lkxOl..  ..'..;::'..''..'::;..'..  ..c0xkl.             
  .cKMx.        .;c:;:cc:;:c:.        .xMKc.              
    ;KX:         ;o::l:;cc;o:.        ;KK;                
     :KK:.       ,d,cd,'ol'o:       .:0K:                 
      ;0NOl:;:loo;. ... .. .;ldlc::lkN0:                  
       .lONNNKOx0Xd,;;'.,:,lKKkk0XNN0o.                   
         .','.. .lX0doooodOXd.  .','.                     
                 .,okkddxkd;.                             
                    'oxxd;.                               
   ........................................                              
   .OWo  xNd lox  xxl Ald   xoc dakkkkkxsx.              
   .OWo  o0W cXW  dM0 MMN   lNK laddKMNkso.               
   .kMKoxsNN oWX  dW0 MMMWO lWK    axM0   .                
   .OMWXNaMX dM0  kM0 MMKxNXKW0    axMk   .                 
   .OMk  dWK oWX XWdx Mxx  XMMO    akMx   .                 
   'OWo  dM0 'kNNXNNd DMD   OWk    aoWd   .                 
   ........................................                 
";

        return logo;
    }
}
