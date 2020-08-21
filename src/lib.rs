#[cfg(test)]
mod rust_multidownloader {
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
    pub fn download(url_list:&[String]){
        let mut listkun=url_list.clone();
        for(dl_url) in listkun{
            println!("{}",&dl_url);
        }
    }
}
