
#[cfg(test)]
mod rust_multidownloader {
    use curl::easy::Easy;
    use std::fs::File;
    use std::thread;
    use threadpool::ThreadPool;
    use std::io::{self, Read, Write, BufReader,BufWriter};
    use std::sync::{Arc, Barrier};
    use regex::Regex;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let download_lskun=[    "https://cdn.kernel.org/pub/linux/kernel/v5.x/linux-5.8.2.tar.xz".to_string(),
        "https://cdn.kernel.org/pub/linux/kernel/v5.x/linux-5.7.16.tar.xz".to_string(),
        "https://cdn.kernel.org/pub/linux/kernel/v5.x/linux-5.4.59.tar.xz".to_string(),
        "https://cdn.kernel.org/pub/linux/kernel/v4.x/linux-4.19.141.tar.xz".to_string()
        ];
        download(&download_lskun,8);
    }
    fn download_child(urlkun:&String){
        let url_re = Regex::new(r".+/(.+?)([\?#;].*)?$").unwrap();
        let urlkun_clone=urlkun.clone();
        let capskun=url_re.captures(&urlkun_clone).unwrap();
        let mut writer = BufWriter::new(File::create(&capskun[1]).unwrap());
        let mut easy = Easy::new();
        easy.url(&urlkun.clone()).unwrap();
        let mut transfer = easy.transfer();
        println!("{}",urlkun.clone());
        transfer.write_function(|data| {
            writer.write_all(&data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    pub fn download(url_list:&[String],max_threads:usize){
        let mut listkun=url_list.clone();
        let pool = ThreadPool::new(max_threads);
        let barrier = Arc::new(Barrier::new(url_list.len() + 1));
        for(dl_url) in listkun{
            let barrier = barrier.clone();
            let dl_url2=dl_url.clone();
            pool.execute(move || {
                download_child(&dl_url2);
                barrier.wait();
            });
        }
        barrier.wait();
    }
}
