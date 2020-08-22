
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
mod rust_multidownloader {
    use curl::easy::Easy;
    use std::fs::File;
    use std::thread;
    use threadpool::ThreadPool;
    use std::io::{self, Read, Write, BufReader,BufWriter};
    use std::sync::{Arc, Barrier};
    use regex::Regex;
    use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
    lazy_static!{
        static ref multi_prog:MultiProgress = {
            let mut mp=MultiProgress::new();
            mp
        };
    }
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let download_lskun=[    "https://cdn.kernel.org/pub/linux/kernel/v5.x/linux-5.8.2.tar.xz".to_string(),
        "https://cdn.kernel.org/pub/linux/kernel/v5.x/linux-5.7.16.tar.xz".to_string(),
        "https://cdn.kernel.org/pub/linux/kernel/v5.x/linux-5.4.59.tar.xz".to_string(),
        "https://cdn.kernel.org/pub/linux/kernel/v4.x/linux-4.19.141.tar.xz".to_string()
        ];
        download(&download_lskun);
    }
    fn download_child(urlkun:&String,barrier:&Arc<Barrier>){
        let url_re = Regex::new(r".+/(.+?)([\?#;].*)?$").unwrap();
        let urlkun_clone=urlkun.clone();
        let capskun=url_re.captures(&urlkun_clone).unwrap();
        let mut writer = BufWriter::new(File::create(&capskun[1]).unwrap());
        let mut easy = Easy::new();    
        let sty = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:50.cyan/blue} {pos:>7}/{len:7} {msg}")
        .progress_chars("##-");
        easy.url(&urlkun.clone()).unwrap();
        easy.progress(true); 
        let mut islongkun:bool=false;
        let mut progbarkun=multi_prog.add(ProgressBar::new(0));
        progbarkun.set_style(sty.clone());
        progbarkun.set_message(&capskun[1]);      
        easy.progress_function(move|dl_all,dl_now,up_all,up_now| {
            progbarkun.set_length(dl_all as u64);
            progbarkun.set_position(dl_now as u64);
            return true;
        }).unwrap();
        let mut transfer = easy.transfer();  
        transfer.write_function(|data| {
            writer.write_all(&data);
            Ok(data.len())
        }).unwrap();
        barrier.wait();
        transfer.perform().unwrap();
    }
    pub fn download(url_list:&[String]){
        let mut listkun=url_list.clone();
        let pool = ThreadPool::new(url_list.len() + 2);
        let barrier = Arc::new(Barrier::new(url_list.len() + 1));
        let barrier2 = Arc::new(Barrier::new(url_list.len() + 1));
        for(dl_url) in listkun{
            let barrier = barrier.clone();
            let barrier2=barrier2.clone();
            let dl_url2=dl_url.clone();
            pool.execute(move || {
                download_child(&dl_url2,&barrier2);
                barrier.wait();
            });
        }
        pool.execute(move || {
            barrier2.wait();
            multi_prog.join_and_clear();
        });
        barrier.wait();
    }
}
