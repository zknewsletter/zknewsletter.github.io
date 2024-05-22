use std::{fs, path::PathBuf};

fn main() {
    let paths = list_all_md_files_from_issues_directory();
    let paths = order_paths_by_date(paths);

    for (i, path) in paths.iter().enumerate() {
        process_issue_html(&path, i == paths.len() - 1);
    }
}

fn list_all_md_files_from_issues_directory() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for entry in fs::read_dir(PathBuf::from("../issues")).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().unwrap() == "md" {
            paths.push(path);
        }
    }
    paths
}

fn order_paths_by_date(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut paths = paths;
    paths.sort_by(|a, b| a.cmp(b));
    paths
}

fn read_path_to_html(path: &PathBuf) -> String {
    let issue = fs::read_to_string(path).unwrap();
    markdown::to_html(&issue)
}

fn get_date_from_path(path: &PathBuf) -> (String, String, String) {
    let path = path.file_name().unwrap().to_str().unwrap();
    let path = path.split('.').collect::<Vec<&str>>();
    let path = path[0].split('-').collect::<Vec<&str>>();
    let year = path[0].to_string();
    let month = path[1].to_string();
    let day = path[2].to_string();
    (year, month, day)
}

fn process_issue_html(path: &PathBuf, as_index: bool) {
    let issue = read_path_to_html(&path);
    let (year, month, day) = get_date_from_path(&path);
    let mut html = TEMPLATE.to_string();
    html = html.replace("{{YEAR}}", &year);
    html = html.replace("{{MONTH}}", &month);
    html = html.replace("{{DAY}}", &day);
    html = html.replace("{{POST_CONTENT}}", &issue);

    let html_file_name = format!("{}-{}-{}.html", year, month, day);
    fs::write(PathBuf::from("../issues/").join(html_file_name), html.clone()).unwrap();

    if as_index {
        fs::write(PathBuf::from("../index.html"), html).unwrap();
    }
}

const TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en-US">
    <head>
        <meta charset="utf-8" />
        <title>ZKNewsletter</title>
        <link rel="shortcut icon" type="image/png" href="/favicon.png" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />

        <!-- Twitter -->
        <meta name="twitter:card" content="summary_large_image" />
        <meta name="twitter:site" content="@odradev" />
        <meta name="twitter:creator" content="@odradev" />
        <meta property="og:url" content="https://zknewsletter.com" />
        <meta property="og:title" content="ZK Newsletter for {{YEAR}}/{{MONTH}}/{{DAY}}" />
        <meta property="og:description" content="Summary of the most important things, which happened in Zero Knowledge world, in the previous week." />
        <meta property="og:image" content="https://zknewsletter.com/issues/{{YEAR}}-{{MONTH}}-{{DAY}}-cover.png" />

        <style>
            @import url('https://fonts.googleapis.com/css2?family=Open+Sans:wght@380&family=Roboto+Mono:wght@400&display=swap');

            :root {
               --font-color: rgb(64, 64, 64);
            }

            a {
                color: var(--font-color);
            }

            .container {
                color: var(--font-color);
                font-family: 'Open Sans', sans-serif;
                display: grid;
                justify-content: center;
                grid-template-columns: min(100vw, 728px);
                grid-template-rows: auto;
                grid-template-areas:
                    "header"
                    "frame-item"
                    "post-header"
                    "post-item"
                    "footer"
            }

            .header {
                grid-area: header;
                justify-self: center;
                font-family: 'Roboto Mono', monospace;
            }
            
            .header h1 {
                text-align: center;
                font-size: min(11vw, 50pt);
            }

            .header p {
                text-align: center;
                font-size: 20pt;
            }

            .header img {
                max-width: 100%;
            }

            .substack-frame {
                grid-area: frame-item;
                justify-self: stretch;
                background:white;
                height: 100pt;
            }

            .post-header {
                text-align: center;
                font-family: 'Roboto Mono', monospace;
            }

            .post-header img {
                max-width: 100%;
            }

            .post-content {
                grid-area: post-item;
                justify-self: stretch;
                font-size: 14pt;
            }

            .post-content h3 {
                padding-left: 8pt;
                font-size: 18pt;
                font-family: 'Roboto Mono', monospace;
            }

            .post-content li:not(:last-child) { 
               margin-bottom: 10pt;  
            }

            .footer {
                text-align: center;
                grid-area: footer;
            }

        </style>
    </head>
    <body>
        <div class="container">
            <div class="header">
                <img src="/full_logo.png"></img>
                <p>Stay in the loop with our Zero Knowledge Newsletter - Your weekly roundup of the latest news, developments, breakthroughs, and innovations in the world of zero knowledge.</p>
            </div>
            <iframe class="substack-frame" src="https://zknewsletter.substack.com/embed" frameborder="0" scrolling="no"></iframe>
            <div class="post-header">
                <h1>Issue - {{YEAR}}/{{MONTH}}/{{DAY}}</h1>
                <img src="/issues/{{YEAR}}-{{MONTH}}-{{DAY}}-cover.png"></img>
            </div>
            
            <div class="post-content">
                {{POST_CONTENT}}
                <p>To become a sponsor mail us: contact@odra.dev. Much more is available in our <a href="https://github.com/odradev/awesome-zero-knowledge"><b>Awesome Zero Knowledge repository</b></a>. Don't be a stranger, leave us a star.</p>
            </div>
            <div class="footer">
                <p>by <a href="https://odra.dev">odra.dev</a></p>
            </div>
        </div>
  </body>
</html>
"#;