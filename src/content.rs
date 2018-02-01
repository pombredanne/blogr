
use super::{COMRAK_OPTIONS, BASE};
use accept::*;
use templates::TemplateMenu;
use xpress::*;

use std::fmt::Display;
use std::{env, str, thread};
use std::fs::{self, File, DirEntry};
use std::io::prelude::*;
use std::io::{self, Cursor, Read};
use std::path::{Path, PathBuf};
use std::time::{self, Instant, Duration};
use std::prelude::*;
use std::ffi::OsStr;
use std::collections::HashMap;
use std::sync::{Mutex, Arc, RwLock};
use std::sync::atomic::AtomicUsize;
// use rocket;
use rocket::http::Status;
use rocket::State;
use rocket_contrib::Template;
use rocket::response::{self, Response, Responder};
use rocket::request::{FromRequest, Request};
use rocket::Outcome;
use rocket::Outcome::Success;
use rocket::response::NamedFile;
use rocket::http::{ContentType, Header, HeaderMap};
// use ::rocket::request::{FromRequest, FromForm, FormItems, FromFormValue, FromParam};
// use ::rocket::outcome::Outcome;
// use rocket::http::RawStr;
// use rocket::response::{content, NamedFile, Redirect, Flash};

use comrak::{markdown_to_html, ComrakOptions};
use twoway;
use brotli;
use libflate::gzip;
use libflate::deflate;

use ::serde::{Deserialize, Serialize};
use serde_json::{Value, Error};



pub const DEFAULT_TEMPLATE: &'static str = "static-default.html.hbs";

pub const SEPARATOR: &[u8] = b"
-----";


// How to load all the pages and get the contents of the Templates on app start?
//   it requires a Request either through a FromRequest or a Responder
//   - maybe pass the PageContext and other info to a Responder
//      Responder:
//          encoding
//          &ContentCacheLock (don't need &mut as the AtomicUsize and RwLock have interior mutability)
//          
//          
//          
//      maybe add a Template in the ContentContext because the Template can be used without a Responder directly
//      then use the cache structure to store the bytes and compressed bytes generated by the respond_to
//      only store the bytes or compressed versions when actually used
//          or maybe: only generate a ContentCached when used but generate all compressed versions at same time
//      
//      
//      

pub struct ContentContext {
    // pub pages: RwLock<HashMap<String, ContentCached>>,
    pub pages: HashMap<String, PageContext>,
    pub size: AtomicUsize,
}

pub struct ContentCacheLock {
    pub pages: RwLock<HashMap<String, ContentCached>>,
    pub size: AtomicUsize,
}

// pub struct ContentRequest<'c, 'u> {
// pub struct ContentRequest<'c, 'p, 'u> {
//     pub encoding: AcceptCompression,
//     pub cache: &'c ContentCacheLock,
//     // pub contexts: &'c ContentContext,
//     pub route: &'u str,
//     pub context: &'p PageContext,
// }
pub struct ContentRequest {
    pub encoding: AcceptCompression,
    // pub cache: ContentCacheLock,
    pub route: String,
    // pub context: PageContext,
    // pub contexts: &'c ContentContext,
}
// pub struct ContentCacheMap {
//     pub pages: HashMap<String, ContentCached>,
//     // pub size: AtomicUsize,
//     // pub size: u64,
// }

#[derive(Debug, Clone)]
pub struct ContentCached {
    // uses: u64,
    // pub size: u64, // size of uncomprsesed
    // pub total: u64, // total size of uncompressed plus all compressed versions
    // pub gzip: Option<Vec<u8>>,
    // pub br: Option<Vec<u8>>,
    // pub deflate: Option<Vec<u8>>,
    pub page: Vec<u8>,
    pub gzip: Vec<u8>,
    pub br: Vec<u8>,
    pub deflate: Vec<u8>,
    pub size: usize,
}

// pub struct PageCached
#[derive(Debug, Clone, Serialize)]
pub struct PageContext {
    pub uri: String,
    pub title: String,
    pub body: String,
    pub template: String,
    pub js: Option<String>,
    pub description: Option<String>,
    pub gentime: String,
    pub base_url: String,
    pub admin: bool,
    pub user: bool,
    pub menu: Option<Vec<TemplateMenu>>,
    pub menu_dropdown: Option<Vec<TemplateMenu>>,
    // pub info: TemplatePageInfo,
}

/// Used to retrieve html and metadata from the page
pub struct PageFormat {
    yaml: Vec<u8>,
    html: Vec<u8>,
}

// Used for the yaml deserialization method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
    pub uri: String,
    pub title: String,
    pub template: String,
    #[serde(default)]
    pub markdown: bool,
    #[serde(default)]
    pub js: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

impl ContentContext {
    pub fn load(dir: &str) -> ContentContext {
        unimplemented!()
    }
    
    pub fn retrieve(&self, uri: &str) -> Option<&PageContext> {
        unimplemented!()
    }
}


impl ContentCacheLock {
    pub fn new() -> ContentCacheLock {
        ContentCacheLock {
            pages: RwLock::new( HashMap::new() ),
            size: AtomicUsize::new( 0 ),
        }
    }
    
    // pub fn retrieve() -> 
    
    
}

// impl<'a, 'c, 'u> Responder<'a> for ContentRequest<'c, 'u> {
impl<'a> Responder<'a> for ContentRequest 
{
    fn respond_to(self, req: &Request) -> response::Result<'a> 
    {
        // get body contents then build an Express instance
        
        // look for the uri entry in self.cache
        // if exists use self.cache.pages.read() to read from the RwLock in the ContentCache
        //   and pull the compression method/original (specified by encoding.preferred()) from the cache
        // if not then create the new cache entry
        
        // let cache: Result<_, _>;
        
        //     cache = self.cache.pages.read();
        //     if let Ok(cache) = self.cache.pages.read() {
        //         // cache_entry = cache.get(self.route);
        //         // cache_entry = cache.get(self.route).map(|r| *r);
        //         cache_entry = cache.get(self.route);
        //     } else {
        //         cache_entry = None;
        //     }
        // }
        
        // Replacing self.cache and self.context
        
        // let content_context_map = req.guard::<ContentContext>();
        // let content_cache = req.guard::<ContentCacheLock>();
        let content_context_map_rst =  req.guard::<State<ContentContext>>();
        let content_cache_rst =  req.guard::<State<ContentCacheLock>>();
        // if content_context_map_rst == 0usize {}
        if let Success(content_context) = content_context_map_rst {
            if let Success(content_cache2) = content_cache_rst {
                let content_cache = content_cache2.inner();
                // let content_context_opt = content_context_map.get(&self.route);
                
                // cache_entry = self.cache.pages.read().unwrap().get(self.route);
                // if let Some(content_context) = content_context_opt {
                    if let Ok(cache) = content_cache.cache.pages.read() 
                    {
                    // if let Ok(cache) = self.cache.pages.read() {
                        let cache_entry: Option<&ContentCached>;
                        cache_entry = cache.get(&self.route);
                        // if let Some(cache_entry) = cache.get(&self.route) {
                        // output_contents is used as a variable to reference in output_bytes
                        //   when there is no existing cache entry for the uri
                        let mut output_contents: Vec<u8> = Vec::new();
                        
                        if let Some(entry) = cache_entry {
                            let entry = cache_entry.unwrap(); // ok because this is guaranteed to be something
                            let mut output_bytes: &Vec<u8> = &Vec::new();
                            match self.encoding.preferred() 
                            {
                                CompressionEncoding::Uncompressed => { output_bytes = &entry.page; },
                                CompressionEncoding::Brotli => { output_bytes = &entry.br; },
                                CompressionEncoding::Gzip => { output_bytes = &entry.gzip; },
                                CompressionEncoding::Deflate => { output_bytes = &entry.deflate; },
                            }
                            
                            // ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- -----
                            // To improve performance: instead of cloning the vector:
                            //   make a String::new() that is converted .into() an Express instance
                            //   then do xresp.streamed_body( Cursor::new(output_bytes) )
                            // ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- -----
                            
                            output_contents = output_bytes.clone();
                            let express: Express = output_contents.into();
                            express.respond_to(req) // do not use express.compress(encoding) as the contents are already compressed!!!
                            
                            // let mut xresp = express.respond_to(req).unwrap_or_default();
                            // xresp.streamed_body(Cursor::new( output_bytes ))
                                
                        }
                         else 
                        {
                            
                            
                            // let template: Template = Template::render(self.context.template.clone(), &self.context);
                            let template: Template = Template::render(content_context.template.clone(), &content_context);
                            
                            // let bytes: Vec<u8> = Vec::new();
                            let express: Express = template.into();
                            
                            let mut xresp = express.respond_to(req).unwrap_or_default();
                            // let mut tresp = template.respond_to(req).unwrap_or_default();
                            // xresp.set_streamed_body(Cursor::new( output_bytes )); // need to use set_streamed_body() when not using a response builder
                            
                            
                            // Performance:
                            // make the if let Some(body) for the xresp.body_bytes() not assign to a variable
                            // instead move all of the compression and the entry: ContentCached struct initialization
                            //   into the if statement, and make an else statement that creates a BLANK entry (otherwise it will have to create the blank entry over and over again)
                            
                            // output_contents = if let Some(body) = xresp.body_bytes() {
                            //     body
                            // } else {
                            //     Vec::new()
                            // };
                            let mut output_contents: Vec<u8> = Vec::new();
                            let entry: ContentCached;
                            if let Some(body) = xresp.body_bytes() 
                            {
                                output_contents = body;
                                let gzip: Vec<u8>;
                                {
                                    let mut buffer = Vec::with_capacity(output_contents.len() + 200);
                                    let mut gzip_encoder = gzip::Encoder::new(buffer).unwrap();
                                    gzip_encoder.write_all(&output_contents).expect("hi gzip"); // .expect("Gzip compression failed.");
                                    gzip = gzip_encoder.finish().into_result().unwrap_or(Vec::new());
                                }
                                
                                let br: Vec<u8>;
                                {
                                    let length = output_contents.len()+200;
                                    let mut buffer = Vec::with_capacity(length);
                                    // let mut compressor = ::brotli::CompressorReader::new(Cursor::new(data), 10*1024, 9, 22);
                                    let mut compressor = ::brotli::CompressorReader::new(Cursor::new(&output_contents), length, 9, 22);
                                    let _ = compressor.read_to_end(&mut buffer);
                                    br = buffer;
                                }
                                
                                let deflate: Vec<u8>;
                                {
                                    let mut buffer = Vec::with_capacity(output_contents.len()+200);
                                    let mut encoder = deflate::Encoder::new(buffer);
                                    encoder.write_all(&output_contents); //.expect("Deflate compression failed.");
                                    deflate = encoder.finish().into_result().unwrap_or(Vec::new());
                                    
                                }
                                
                                let output_length = output_contents.len();
                                let gzip_length = gzip.len();
                                let br_length = br.len();
                                let deflate_length = deflate.len();
                                
                                entry = ContentCached 
                                {
                                    page: output_contents,
                                    gzip,
                                    br,
                                    deflate,
                                    // size: output_contents.len() + gzip.len() + br.len() + deflate.len(),
                                    size: output_length + gzip_length + br_length + deflate_length,
                                };
                            } 
                            else 
                            {
                                let output_length = output_contents.len();
                                output_contents = Vec::new();
                                entry = ContentCached 
                                {
                                    page: output_contents,
                                    gzip: Vec::new(),
                                    br: Vec::new(),
                                    deflate: Vec::new(),
                                    size: output_length,
                                };
                            }
                            {
                                // let map_rst = self.cache.pages.write();
                                let map_rst = content_cache.pages.write();
                                if let Ok(mut map) = map_rst 
                                {
                                    map.insert(self.route.to_owned(), entry.clone());
                                }
                            }
                            
                            // Add entry
                            // Add total size to the cache.size:
                            //   br.len() + gzip.len() + deflate.len() + output_contents.len()
                            
                            // need to set the body because the body_bytes() method consumes it
                            // which is ok because it gets 
                            
                            let output = match self.encoding.preferred() 
                            {
                                CompressionEncoding::Uncompressed => { entry.page },
                                CompressionEncoding::Brotli => { entry.br },
                                CompressionEncoding::Gzip => { entry.gzip },
                                CompressionEncoding::Deflate => { entry.deflate },
                            };
                            
                            xresp.set_streamed_body(  Cursor::new( output )  );
                            Ok(xresp)
                        }
                    } else {
                        Err(Status::BadRequest)
                    }
                // } else {
                //     Err(Status::BadRequest)
                // }
            } else {
                Err(Status::ImATeapot)
            }
        } else {
            Err(Status::ImATeapot)
        }
    }
}

















