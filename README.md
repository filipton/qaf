<div align="center">
  <h1>QAF</h1>

  <!-- Version -->
  <a href="https://crates.io/crates/qaf">
    <img src="https://img.shields.io/crates/v/qaf.svg?style=flat-square"
        alt="Crates.io version" />
  </a>

  <!-- Docs -->
  <a href="https://docs.rs/planetscale-driver">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
        alt="docs.rs docs" />
  </a>
  
  Create web api's at ease!
  
  ***IF YOU WANT TO SEE TEMPLATES CLICK [HERE](https://github.com/filipton/qaf-templates).***
</div>

## Why and how?
Main reason why creating web api's in rust is slow (not really) is the thing with routing. <br/>
In every framework you must to set route handlers by hand, so i've made simple build.rs scripts
to speed up this process.

Routes are stored in `src/pages` dir, they are automatically scanned and added to router 
(Actix, Axum, Cloudflare and event Vercel with custom router implementation).

## Compatibility
I'm not creating own Responses so your function declarations are the same. 
Only Vercel have different implementation, because it doesn't have bullt in router.

## Installation
```bash
cargo install qaf
```
## Usage
[![asciicast](https://asciinema.org/a/xJpfVUr6hlhD8SqPaDYqiTUPm.svg)](https://asciinema.org/a/xJpfVUr6hlhD8SqPaDYqiTUPm)

## Support

|      #      |           Actix           |            Axum           |                       Cloudflare Workers                      |                          Vercel Edge                          |
|:-----------:|:-------------------------:|:-------------------------:|:-------------------------------------------------------------:|:-------------------------------------------------------------:|
|   Routing   |         Dir based         |         Dir based         |                           Dir based                           |                           Dir based                           |
|   Database  | Mysql and Postgres (SQLX) | Mysql and Postgres (SQLX) | [Planetscale](https://github.com/filipton/planetscale-driver) | [Planetscale](https://github.com/filipton/planetscale-driver) |
|  Websockets |        Tungstenite        |        Tungstenite        |                              Yes                              |                               No                              |
|    Docker   |            Yes            |            Yes            |                               No                              |                               No                              |
| Dev command |            Yes            |            Yes            |                              Yes                              |                              Yes                              |

