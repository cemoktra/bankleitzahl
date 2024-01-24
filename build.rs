use std::io::BufRead;

pub fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=data.txt");

    let file = std::fs::File::open("data.txt").expect("data.txt exists");
    let reader = std::io::BufReader::new(file);

    let mut blz_structs = quote::quote! {};

    for line in reader.lines() {
        let line = line.expect("is valid UTF-8");
        let chars = line.chars().collect::<Vec<_>>();

        let blz: String = chars[0..8].iter().collect();
        let name: String = chars[9..67].iter().collect();
        let plz: String = chars[67..72].iter().collect();
        let city: String = chars[72..107].iter().collect();
        let bic: String = chars[139..150].iter().collect();
        let del: String = chars[159..160].iter().collect();
        let new_blz: String = chars[160..168].iter().collect();

        let name = name.trim();
        let bic = bic.trim();
        let city = city.trim();

        let bic_token = if bic.is_empty() {
            quote::quote! {
                None
            }
        } else {
            quote::quote! {
                Some(#bic)
            }
        };

        let deleted = del.as_str() == "1";

        let new_blz_code = if deleted {
            quote::quote! { Some(#new_blz) }
        } else {
            quote::quote! { None }
        };

        blz_structs.extend(quote::quote! {
            Blz {
                blz: #blz,
                bic: #bic_token,
                name: #name,
                post_code: #plz,
                city: #city,
                deleted: #deleted,
                new_blz: #new_blz_code
            },
        });
    }

    let generated_code = quote::quote! {
        pub struct Blz {
            blz: &'static str,
            bic: Option<&'static str>,
            name: &'static str,
            post_code: &'static str,
            city: &'static str,
            deleted: bool,
            new_blz: Option<&'static str>,
        }

        const DATA: &[Blz] = &[
            #blz_structs
        ];

        impl Blz {
            pub fn from_blz(blz: &str) -> Vec<&'static Self> {
                DATA.iter().filter(|item| item.blz == blz).collect()
            }

            pub fn from_bic(bic: &str) -> Vec<&'static Self> {
                DATA.iter().filter(|item| item.bic == Some(bic)).collect()
            }

            pub fn blz(&self) -> &'static str {
                self.blz
            }

            pub fn new_blz(&self) -> Option<&'static str> {
                self.new_blz
            }

            pub fn is_deleted(&self) -> bool {
                self.deleted
            }

            pub fn bic(&self) -> Option<&'static str> {
                self.bic
            }

            pub fn name(&self) -> &'static str {
                self.name
            }

            pub fn post_code(&self) -> &'static str {
                self.post_code
            }

            pub fn city(&self) -> &'static str {
                self.city
            }
        }
    };

    std::fs::write("src/data.rs", generated_code.to_string()).expect("generate code");

    std::process::Command::new("rustfmt")
        .arg("src/data.rs")
        .output()
        .expect("Unable to handle process");
}
